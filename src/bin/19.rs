use std::{cmp::Reverse, collections::HashMap, future::ready, result};

use itertools::Itertools;

advent_of_code::solution!(19);

pub fn part_one(input: &str) -> Option<u64> {
    let mut data: Vec<&str> = input.split("\r\n\r\n").collect();
    if data.len() < 2 {
        // Actual data split.
        data = input.split("\n\n").collect();
    }
    let patterns_data = data[0];
    let targets_data = data[1];

    let patterns = parse_patterns(patterns_data);
    let targets = parse_targets(targets_data);

    let result = targets.iter().fold(0, |acc, t| {
        if is_target_possible(t, &patterns) {
            acc + 1
        } else {
            acc
        }
    });

    dbg!(result);

    Some(result as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut data: Vec<&str> = input.split("\r\n\r\n").collect();
    if data.len() < 2 {
        // Actual data split.
        data = input.split("\n\n").collect();
    }
    let patterns_data = data[0];
    let targets_data = data[1];

    let patterns = parse_patterns(patterns_data);
    let targets = parse_targets(targets_data);

    let result = targets.iter().enumerate().fold(0, |acc, (i, t)| {
        println!("Solving {} of {}", i + 1, targets.len());
        acc + count_target_possible(t, &patterns)
    });

    // let result = count_target_possible(&targets[4], &patterns);

    dbg!(result);

    Some(result as u64)
}

fn is_target_possible(target: &str, patterns: &HashMap<usize, Vec<String>>) -> bool {
    if target.len() == 0 {
        return true;
    }

    // Go from bigger size patterns
    for pattern_size in patterns.keys().sorted().rev() {
        for pattern in &patterns[&pattern_size] {
            if target.starts_with(pattern) {
                let result = is_target_possible(&target[*pattern_size..], patterns);
                if result {
                    return true;
                }
            }
        }
    }

    return false;
}

fn count_target_possible(target: &str, patterns: &HashMap<usize, Vec<String>>) -> u32 {
    if target.len() == 0 {
        return 1;
    }

    let mut total_result = 0;

    // Go from bigger size patterns
    for pattern_size in patterns.keys().sorted().rev() {
        for pattern in &patterns[&pattern_size] {
            if target.starts_with(pattern) {
                let result = count_target_possible(&target[*pattern_size..], patterns);
                total_result += result;
            }
        }
    }

    return total_result;
}

fn parse_patterns(input: &str) -> HashMap<usize, Vec<String>> {
    let mut patterns: HashMap<usize, Vec<String>> = HashMap::new();

    for pattern in input.split(", ") {
        patterns
            .entry(pattern.len())
            .and_modify(|e| e.push(pattern.to_string()))
            .or_insert(vec![pattern.to_string()]);
    }

    patterns
}

fn parse_targets(input: &str) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    for line in input.lines() {
        result.push(line.to_string());
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(16));
    }
}
