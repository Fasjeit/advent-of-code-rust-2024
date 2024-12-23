use std::{
    collections::{HashMap, HashSet},
    sync::{Arc, Mutex},
    thread,
};

advent_of_code::solution!(22);

pub fn part_one(input: &str) -> Option<u64> {
    let mut secrets: Vec<u64> = Vec::new();
    input
        .lines()
        .for_each(|l| secrets.push(l.parse().expect("Expected u64")));

    let result = secrets.iter().fold(0, |acc: u64, s| {
        let mut secret = *s;
        prg_n(&mut secret, 2000);
        acc + secret
    });

    Some(result)
}

pub fn part_two_single_thread(input: &str) -> Option<u64> {
    // thanks to https://www.reddit.com/r/adventofcode/comments/1hjroap/comment/m393l1y/
    // for general idea.

    // just wait for sequence of length 3
    // compute its cvalue and store in dictionary.

    let mut secrets: Vec<u64> = Vec::new();
    input
        .lines()
        .for_each(|l| secrets.push(l.parse().expect("Expected u64")));

    let mut diff_dict = HashMap::new();

    for mut secret in secrets {
        let mut current_sequence = Vec::new();
        let mut sequence_set = HashSet::new();

        // Process the initial value
        let mut prev_price = get_price(&secret);

        // Iterate through 2001 prices (initial + 2000 iterations)
        for _ in 0..=2000 {
            let current_price = get_price(&secret);

            // Calculate the difference and manage the sequence
            let diff = get_price_diff(&current_price, &prev_price);
            prev_price = current_price;

            current_sequence.push(diff);

            if current_sequence.len() > 4 {
                current_sequence.remove(0); // Keep the last 4 elements
            }

            if current_sequence.len() > 3 {
                // Update `diff_dict` if the sequence is new
                if !sequence_set.contains(&current_sequence) {
                    sequence_set.insert(current_sequence.clone());

                    // add to or update the diff_dict
                    diff_dict
                        .entry(current_sequence.clone())
                        .and_modify(|e| {
                            *e += current_price;
                        })
                        .or_insert(current_price);
                }
            }

            // Update the next number using `prg`
            prg(&mut secret);
        }
    }

    let max = diff_dict.values().max().unwrap();
    Some(*max)
}

pub fn part_two(input: &str) -> Option<u64> {
    // gpt helped multithread version

    let mut secrets: Vec<u64> = Vec::new();
    input
        .lines()
        .for_each(|l| secrets.push(l.parse().expect("Expected u64")));

    let diff_dict = Arc::new(Mutex::new(HashMap::new()));

    // Create a thread pool and split the work
    let mut handles = vec![];
    let secrets_per_thread = secrets.len() / 4; // Adjust the number of threads as needed

    for chunk in secrets.chunks(secrets_per_thread) {
        let diff_dict = Arc::clone(&diff_dict);
        let chunk = chunk.to_vec(); // Copy the chunk for thread ownership

        let handle = thread::spawn(move || {
            let mut local_diff_dict = HashMap::new();

            for mut secret in chunk {
                let mut current_sequence = Vec::new();
                let mut sequence_set = HashSet::new();

                let mut prev_price = get_price(&secret);

                for _ in 0..=2000 {
                    let current_price = get_price(&secret);
                    let diff = get_price_diff(&current_price, &prev_price);
                    prev_price = current_price;

                    current_sequence.push(diff);

                    if current_sequence.len() > 4 {
                        current_sequence.remove(0);
                    }

                    #[allow(clippy::collapsible_if)]
                    if current_sequence.len() > 3 {
                        if !sequence_set.contains(&current_sequence) {
                            sequence_set.insert(current_sequence.clone());

                            local_diff_dict
                                .entry(current_sequence.clone())
                                .and_modify(|e| {
                                    *e += current_price;
                                })
                                .or_insert(current_price);
                        }
                    }

                    prg(&mut secret);
                }
            }

            // Merge local results into the shared HashMap
            let mut global_diff_dict = diff_dict.lock().unwrap();
            for (key, value) in local_diff_dict {
                global_diff_dict
                    .entry(key)
                    .and_modify(|e| {
                        *e += value;
                    })
                    .or_insert(value);
            }
        });

        handles.push(handle);
    }

    // Wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }

    // Find the maximum value in the shared HashMap
    let diff_dict = diff_dict.lock().unwrap();
    let max = diff_dict.values().max().cloned().unwrap();
    Some(max)
}

fn prg_n(secret: &mut u64, iteration: u64) {
    for _i in 0..iteration {
        prg(secret);
    }
}

fn prg(secret: &mut u64) {
    // step 1
    let to_mix = *secret * 64;
    mix(secret, to_mix);
    prune(secret);

    // step 2
    let to_mix = *secret / 32;
    mix(secret, to_mix);
    prune(secret);

    // step 3
    let to_mix = *secret * 2048;
    mix(secret, to_mix);
    prune(secret);
}

fn mix(secret: &mut u64, value: u64) {
    *secret ^= value;
}

fn prune(secret: &mut u64) {
    *secret %= 16777216;
}

fn get_price(secret: &u64) -> u64 {
    secret % 10
}

fn get_price_diff(current_price: &u64, last_price: &u64) -> i64 {
    *current_price as i64 - *last_price as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mix() {
        let mut secret = 42;
        mix(&mut secret, 15);
        assert_eq!(secret, 37);
    }

    #[test]
    fn test_prune() {
        let mut secret = 100000000;
        prune(&mut secret);
        assert_eq!(secret, 16113920);
    }

    #[test]
    fn test_10_prg() {
        let mut secret = 123;

        prg(&mut secret);
        assert_eq!(secret, 15887950);

        prg(&mut secret);
        assert_eq!(secret, 16495136);

        prg(&mut secret);
        assert_eq!(secret, 527345);

        prg(&mut secret);
        assert_eq!(secret, 704524);

        prg(&mut secret);
        assert_eq!(secret, 1553684);

        prg(&mut secret);
        assert_eq!(secret, 12683156);

        prg(&mut secret);
        assert_eq!(secret, 11100544);

        prg(&mut secret);
        assert_eq!(secret, 12249484);

        prg(&mut secret);
        assert_eq!(secret, 7753432);

        prg(&mut secret);
        assert_eq!(secret, 5908254);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(37327623));
    }

    #[test]
    fn test_part_two_single_thread() {
        let result = part_two_single_thread(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(23));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(23));
    }
}
