use std::{
    collections::{HashMap, HashSet},
    env::current_exe,
};

advent_of_code::solution!(22);

pub fn part_one(input: &str) -> Option<u128> {
    let mut secrets: Vec<u128> = Vec::new();
    input
        .lines()
        .for_each(|l| secrets.push(l.parse().expect("Expected u128")));

    let result = secrets.iter().fold(0, |acc: u128, s| {
        let mut secret = *s;
        prg_n(&mut secret, 2000);
        acc + secret
    });

    Some(result)
}

pub fn part_two(input: &str) -> Option<u128> {
    // thanks to https://www.reddit.com/r/adventofcode/comments/1hjroap/comment/m393l1y/
    // for general idea.

    let mut secrets: Vec<u128> = Vec::new();
    input
        .lines()
        .for_each(|l| secrets.push(l.parse().expect("Expected u128")));

    let mut diff_dict = HashMap::new();

    for t in secrets {
        //dbg!(t);
        let mut current_prices = Vec::new();
        let last_price = get_price(&t);
        current_prices.push(last_price);

        let mut current = t;

        for _iteration in 0..2000 {
            let mut nex_num = current;
            prg(&mut nex_num);
            let current_price = get_price(&nex_num);
            current_prices.push(current_price);
            current = nex_num;
        }

        //dbg!(&current_prices);

        let mut current_sequence = Vec::new();
        let mut sequence_set = HashSet::new();
        let mut prev_num = current_prices[0]; // Start with the first price.

        for n in 1..current_prices.len() {
            let next_num = current_prices[n];
            let diff = get_price_diff(&next_num, &prev_num);
            prev_num = next_num; // Update `prev_num` here.

            current_sequence.push(diff);

            // Manage sequence length:
            if current_sequence.len() > 4 {
                current_sequence.remove(0); // Remove first element if too long.
            }
            if current_sequence.len() <= 3 {
                continue; // Skip if the sequence is too short.
            }

            // Add unique sequence to `sequence_set`:
            if sequence_set.contains(&current_sequence) {
                continue; // Skip duplicate sequences.
            }
            sequence_set.insert(current_sequence.clone());

            // Update `diff_dict` for the sequence:
            diff_dict
                .entry(current_sequence.clone())
                .and_modify(|e| {
                    *e += next_num;
                })
                .or_insert(next_num);
        }
    }

    let max = diff_dict.values().max().unwrap();
    Some(*max)
}

fn prg_n(secret: &mut u128, iteration: u128) {
    for _i in 0..iteration {
        prg(secret);
    }
}

fn prg(secret: &mut u128) {
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

fn mix(secret: &mut u128, value: u128) {
    *secret = *secret ^ value;
}

fn prune(secret: &mut u128) {
    *secret = *secret % 16777216;
}

fn get_price(secret: &u128) -> u128 {
    secret % 10
}

fn get_price_diff(current_price: &u128, last_price: &u128) -> i64 {
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
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(23));
    }
}
