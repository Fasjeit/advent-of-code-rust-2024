use std::collections::HashMap;

advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<u64> {
    const TOTAL_BLINKS: u32 = 25;

    part_one_blinks(input, TOTAL_BLINKS)
}

pub fn part_two(input: &str) -> Option<u128> {
    const TOTAL_BLINKS: u32 = 75;

    part_two_blinks(input, TOTAL_BLINKS)
}

pub fn part_one_blinks(input: &str, blinks: u32) -> Option<u64> {
    // Just create the vec of stones and compute them.

    let mut data = parse_line_as_vec(input);
    //dbg!(&data);

    for _ in 0..blinks {
        data = blink(&mut data);
        //dbg!(&data);
    }

    Some(data.len() as u64)
}

pub fn part_two_blinks(input: &str, blinks: u32) -> Option<u128> {
    // Resulted vec is too big.
    // We will count the number of stones with distinct numbers instead;

    let data = parse_line_as_vec(input);

    // stone value : count of stones with value;
    let mut count: HashMap<u64, u128> = HashMap::new();

    data.iter().for_each(|stone| {
        add_n_or_create(&mut count, *stone as u128, 1);
    });

    for i in 0..blinks {
        count = blink_count(&mut count);
        if cfg!(debug_assertions) {
            println!("Computing {} / {}", i + 1, blinks);
        }
    }

    let result = count.values().sum();
    
    Some(result)
}

fn add_n_or_create(count: &mut HashMap<u64, u128>, key: u128, n: u128) {
    count.entry(key as u64).and_modify(|e| *e += n).or_insert(n);
}

fn blink_count(count: &mut HashMap<u64, u128>) -> HashMap<u64, u128> {
    let mut result = HashMap::<u64, u128>::new();

    for stone in count.keys() {
        // Current number of stones with `stone` number on it.
        let stone_numbers = count[stone];
        if *stone == 0 {
            add_n_or_create(&mut result, 1, stone_numbers);
        } else if base_10_len(*stone) % 2 == 0 {
            // split stone
            let half_ord = base_10_len(*stone) / 2;
            let pow = 10_u64.pow(half_ord);

            let left = stone / pow;
            let right = stone % pow;

            add_n_or_create(&mut result, left as u128, stone_numbers);
            add_n_or_create(&mut result, right as u128, stone_numbers);
        } else {
            add_n_or_create(&mut result, *stone as u128 * 2024, stone_numbers);
        }
    }

    result
}

fn blink(data: &mut Vec<u64>) -> Vec<u64> {
    let mut result = Vec::<u64>::with_capacity(data.len());
    for stone in data {
        if *stone == 0 {
            result.push(1);
        } else if base_10_len(*stone) % 2 == 0 {
            // split stone
            let half_ord = base_10_len(*stone) / 2;
            let pow = 10_u64.pow(half_ord);

            let left = *stone / pow;
            let right = *stone % pow;

            result.push(left);
            result.push(right);
        } else {
            result.push(*stone * 2024);
        }
    }

    result
}

fn base_10_len(n: u64) -> u32 {
    n.checked_ilog10().unwrap_or(0) + 1
}

fn parse_line_as_vec(input: &str) -> Vec<u64> {
    let mut result = Vec::<u64>::new();
    input
        .split_whitespace()
        .for_each(|e| result.push(e.parse().expect("Expected u64 elements")));

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_blinks_6() {
        let result = part_one_blinks(&advent_of_code::template::read_file("examples", DAY), 6);
        assert_eq!(result, Some(22));
    }

    #[test]
    fn test_part_one_blinks_25() {
        let result = part_one_blinks(&advent_of_code::template::read_file("examples", DAY), 25);
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two_blinks_6() {
        let result = part_two_blinks(&advent_of_code::template::read_file("examples", DAY), 6);
        assert_eq!(result, Some(22));
    }

    #[test]
    fn test_part_two_blinks_25() {
        let result = part_two_blinks(&advent_of_code::template::read_file("examples", DAY), 25);
        assert_eq!(result, Some(55312));
    }
}
