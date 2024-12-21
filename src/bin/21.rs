use std::collections::HashMap;
use std::fmt::Debug;

advent_of_code::solution!(21);

pub fn part_one(input: &str) -> Option<u64> {
    // huge thanks to https://www.reddit.com/r/adventofcode/comments/1hj2odw/comment/m34tgje
    // for both parts

    let mut acc = 0;

    for line in input.lines() {
        let input_vec: Vec<char> = line.chars().collect();
        let result_main = main_keypad(&input_vec);
        let result_robot_1 = robot_keypad(&result_main);
        let result_robot_2 = robot_keypad(&result_robot_1);

        let mut numeric_part = input_vec.clone();
        numeric_part.truncate(input_vec.len() - 1);

        let numeric_part_string: String = numeric_part.iter().collect();
        let numeric: u64 = numeric_part_string
            .parse()
            .expect("Error parsing numeric part");
        let result = (result_robot_2.len() as u64) * numeric;
        acc += result;
    }

    Some(acc as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    part_two_robots(input, 25)
}

pub fn part_two_robots(input: &str, robots: u64) -> Option<u64> {
    let mut acc = 0;

    for line in input.lines() {
        let input_vec: Vec<char> = line.chars().collect();
        let result_main = main_keypad(&input_vec);

        let mut cache = HashMap::new();
        let result_robots = get_count_many_robots(&result_main, robots, 0, &mut cache);

        let mut numeric_part = input_vec.clone();
        numeric_part.truncate(input_vec.len() - 1);

        let numeric_part_string: String = numeric_part.iter().collect();
        let numeric: u64 = numeric_part_string
            .parse()
            .expect("Error parsing numeric part");
        let result = (result_robots as u64) * numeric;
        acc += result;
    }

    Some(acc as u64)
}

fn get_count_many_robots(
    input: &Vec<char>,
    max_robots: u64,
    robot: u64,
    cache: &mut HashMap<String, Vec<u64>>,
) -> u64 {
    let input_string: String = input.iter().collect();

    // Avoid holding a mutable reference to `cache` for too long
    if let Some(entry) = cache.get_mut(&input_string) {
        if entry[robot as usize] != 0 {
            return entry[robot as usize];
        }
    } else {
        cache.insert(input_string.clone(), vec![0; max_robots as usize]);
    }

    let presses = robot_keypad(input);

    // Re-acquire mutable reference after potential cache modification
    let entry = cache.get_mut(&input_string).unwrap();
    entry[0] = presses.len() as u64;

    if robot == max_robots - 1 {
        return presses.len() as u64;
    }

    let mut acc = 0;
    let splitted_commands = split_command_by_enters(&presses);
    for command in splitted_commands {
        let count = get_count_many_robots(&command, max_robots, robot + 1, cache);
        acc += count;
    }

    // Update the cache after recursion
    let entry = cache.get_mut(&input_string).unwrap();
    entry[robot as usize] = acc;

    acc
}

fn split_command_by_enters(input: &Vec<char>) -> Vec<Vec<char>> {
    let mut result: Vec<Vec<char>> = vec![];

    let mut current: Vec<char> = vec![];
    for press in input {
        current.push(*press);
        if *press == 'A' {
            result.push(current);
            current = vec![];
        }
    }
    result
}

fn robot_keypad(input: &Vec<char>) -> Vec<char> {
    /*
    //     +---+---+
    //     | ^ | A |
    // +---+---+---+
    // | < | v | > |
    // +---+---+---+
     */

    let mut keypad = HashMap::new();
    keypad.insert('_', Index { x: 0, y: 0 });
    keypad.insert('^', Index { x: 1, y: 0 });
    keypad.insert('A', Index { x: 2, y: 0 });
    keypad.insert('<', Index { x: 0, y: 1 });
    keypad.insert('v', Index { x: 1, y: 1 });
    keypad.insert('>', Index { x: 2, y: 1 });

    let mut current_index = Index { x: 2, y: 0 };
    let mut result = vec![];

    for target in input {
        //dbg!(target);
        let target_index = &keypad[target];
        if current_index == *target_index {
            result.push('A');
            continue;
        }

        let mut horizontal = vec![];
        for _dx in 0..(current_index.x.abs_diff(target_index.x)) {
            if current_index.x < target_index.x {
                horizontal.push('>')
            } else {
                horizontal.push('<')
            };
        }

        let mut vertical = vec![];
        for _dy in 0..(current_index.y.abs_diff(target_index.y)) {
            if current_index.y < target_index.y {
                vertical.push('v')
            } else {
                vertical.push('^')
            };
        }

        // prioritization order:
        // 1. moving with least turns
        // 2. moving < over ^ over v over >

        if current_index.x == 0 && target_index.y == 0 {
            result.append(&mut horizontal);
            result.append(&mut vertical);
        } else if current_index.y == 0 && target_index.x == 0 {
            result.append(&mut vertical);
            result.append(&mut horizontal);
        } else if current_index.x > target_index.x {
            result.append(&mut horizontal);
            result.append(&mut vertical);
        } else if current_index.x <= target_index.x {
            result.append(&mut vertical);
            result.append(&mut horizontal);
        } else {
            todo!()
        }

        result.push('A');
        current_index = *target_index;
    }

    result
}

fn main_keypad(input: &Vec<char>) -> Vec<char> {
    /*
    // +---+---+---+
    // | 7 | 8 | 9 |
    // +---+---+---+
    // | 4 | 5 | 6 |
    // +---+---+---+
    // | 1 | 2 | 3 |
    // +---+---+---+
    //	   | 0 | A |
    //	   +---+---+
     */

    let mut keypad = HashMap::new();
    keypad.insert('7', Index { x: 0, y: 0 });
    keypad.insert('8', Index { x: 1, y: 0 });
    keypad.insert('9', Index { x: 2, y: 0 });
    keypad.insert('4', Index { x: 0, y: 1 });
    keypad.insert('5', Index { x: 1, y: 1 });
    keypad.insert('6', Index { x: 2, y: 1 });
    keypad.insert('1', Index { x: 0, y: 2 });
    keypad.insert('2', Index { x: 1, y: 2 });
    keypad.insert('3', Index { x: 2, y: 2 });
    keypad.insert('_', Index { x: 0, y: 3 });
    keypad.insert('0', Index { x: 1, y: 3 });
    keypad.insert('A', Index { x: 2, y: 3 });

    let mut current_index = Index { x: 2, y: 3 };
    let mut result = vec![];

    for target in input {
        let target_index = &keypad[target];
        if current_index == *target_index {
            result.push('A');
            continue;
        }

        let mut horizontal = vec![];
        for _dx in 0..(current_index.x.abs_diff(target_index.x)) {
            if current_index.x < target_index.x {
                horizontal.push('>')
            } else {
                horizontal.push('<')
            };
        }

        let mut vertical = vec![];
        for _dy in 0..(current_index.y.abs_diff(target_index.y)) {
            if current_index.y < target_index.y {
                vertical.push('v')
            } else {
                vertical.push('^')
            };
        }

        // prioritization order:
        // 1. moving with least turns
        // 2. moving < over ^ over v over >
        if current_index.y == 3 && target_index.x == 0 {
            result.append(&mut vertical);
            result.append(&mut horizontal);
        } else if current_index.x == 0 && target_index.y == 3 {
            result.append(&mut horizontal);
            result.append(&mut vertical);
        } else if current_index.x > target_index.x {
            result.append(&mut horizontal);
            result.append(&mut vertical);
        } else if current_index.x <= target_index.x {
            result.append(&mut vertical);
            result.append(&mut horizontal);
        } else {
            todo!()
        }

        result.push('A');
        current_index = *target_index;
    }

    result
}

#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq, PartialOrd, Ord)]
struct Index {
    x: usize,
    y: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_keypad_part1() {
        let input = "029A";
        let input_vec: Vec<char> = input.chars().collect();
        let result = main_keypad(&input_vec);
        let result_string: String = result.into_iter().collect();
        assert_eq!(result_string, "<A^A^^>AvvvA");
    }

    #[test]
    fn test_main_keypad_part1_len() {
        let input = "029A";
        let input_vec: Vec<char> = input.chars().collect();
        let result = main_keypad(&input_vec);
        let result_string: String = result.into_iter().collect();
        assert_eq!(result_string.len(), "<A^A^^>AvvvA".len());
    }

    #[test]
    fn test_main_keypad_with_one_robot_part1_len() {
        let input = "029A";
        let input_vec: Vec<char> = input.chars().collect();
        let result_main = main_keypad(&input_vec);
        let result_robot = robot_keypad(&result_main);
        let result_string: String = result_robot.into_iter().collect();
        dbg!(&result_string);
        assert_eq!(result_string.len(), "v<<A>>^A<A>AvA<^AA>A<vAAA>^A".len());
    }

    #[test]
    fn test_main_keypad_with_two_robot_part1_len_1() {
        let input = "029A";
        let input_vec: Vec<char> = input.chars().collect();
        let result_main = main_keypad(&input_vec);
        let result_robot_1 = robot_keypad(&result_main);
        let result_robot_2 = robot_keypad(&result_robot_1);
        let result_string: String = result_robot_2.into_iter().collect();
        dbg!(&result_string);
        assert_eq!(
            result_string.len(),
            "<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A".len()
        );
    }

    #[test]
    fn test_main_keypad_with_two_robot_part1_len_2() {
        let input = "980A";
        let input_vec: Vec<char> = input.chars().collect();
        let result_main = main_keypad(&input_vec);
        let result_robot_1 = robot_keypad(&result_main);
        let result_robot_2 = robot_keypad(&result_robot_1);
        let result_string: String = result_robot_2.into_iter().collect();
        dbg!(&result_string);
        assert_eq!(
            result_string.len(),
            "<v<A>>^AAAvA^A<vA<AA>>^AvAA<^A>A<v<A>A>^AAAvA<^A>A<vA>^A<A>A".len()
        );
    }

    #[test]
    fn test_main_keypad_with_two_robot_part1_len_3() {
        let input = "179A";
        let input_vec: Vec<char> = input.chars().collect();
        let result_main = main_keypad(&input_vec);
        let result_robot_1 = robot_keypad(&result_main);
        let result_robot_2 = robot_keypad(&result_robot_1);
        let result_string: String = result_robot_2.into_iter().collect();
        dbg!(&result_string);
        assert_eq!(
            result_string.len(),
            "<v<A>>^A<vA<A>>^AAvAA<^A>A<v<A>>^AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A".len()
        );
    }

    #[test]
    fn test_main_keypad_with_two_robot_part1_len_4() {
        let input = "456A";
        let input_vec: Vec<char> = input.chars().collect();
        let result_main = main_keypad(&input_vec);
        let result_robot_1 = robot_keypad(&result_main);
        let result_robot_2 = robot_keypad(&result_robot_1);
        let result_string: String = result_robot_2.into_iter().collect();
        dbg!(&result_string);
        assert_eq!(
            result_string.len(),
            "<v<A>>^AA<vA<A>>^AAvAA<^A>A<vA>^A<A>A<vA>^A<A>A<v<A>A>^AAvA<^A>A".len()
        );
    }

    #[test]
    fn test_main_keypad_with_two_robot_part1_len_5() {
        let input = "379A";
        let input_vec: Vec<char> = input.chars().collect();
        let result_main = main_keypad(&input_vec);
        let result_robot_1 = robot_keypad(&result_main);
        let result_robot_2 = robot_keypad(&result_robot_1);
        let result_string: String = result_robot_2.into_iter().collect();
        dbg!(&result_string);
        assert_eq!(
            result_string.len(),
            "<v<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A".len()
        );
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(126384));
    }

    #[test]
    fn test_part_two_same_as_part_1() {
        let result = part_two_robots(&advent_of_code::template::read_file("examples", DAY), 2);
        assert_eq!(result, Some(126384));
    }
}
