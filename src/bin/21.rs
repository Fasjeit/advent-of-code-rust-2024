use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use std::f32::consts::E;
use std::fmt::Debug;
use std::str::FromStr;
use std::vec;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

advent_of_code::solution!(21);

pub fn part_one(input: &str) -> Option<u64> {
    // main_keypad(&[]);
    None
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

fn robot_keypad(input: &Vec<char>) -> Vec<char> {
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
        let target_index = &keypad[target];
        if current_index == *target_index {
            result.push('A');
            continue;
        }

        if current_index.y > target_index.y {
            // right-left then up
            let second_command = if current_index.x < target_index.x {
                '>'
            } else {
                '<'
            };

            for _dx in 0..(current_index.x.abs_diff(target_index.x)) {
                result.push(second_command);
            }

            for _dy in 0..(current_index.y - target_index.y) {
                result.push('^');
            }
        } else {
            // down then right-left
            for _dy in 0..(target_index.y - current_index.y) {
                result.push('v');
            }

            let second_command = if current_index.x < target_index.x {
                '>'
            } else {
                '<'
            };

            for _dx in 0..(current_index.x.abs_diff(target_index.x)) {
                result.push(second_command);
            }
        }

        result.push('A');
        current_index = *target_index;
    }

    result
}

fn main_keypad(input: &Vec<char>) -> Vec<char> {
    // populate keyboard
    // let mut keyboard = Matrix {
    //     size: Size { x: 3, y: 4 },
    //     data: vec![],
    // };
    // keyboard.data.push(MapCell { value: Some('7') });
    // keyboard.data.push(MapCell { value: Some('8') });
    // keyboard.data.push(MapCell { value: Some('9') });
    // keyboard.data.push(MapCell { value: Some('4') });
    // keyboard.data.push(MapCell { value: Some('5') });
    // keyboard.data.push(MapCell { value: Some('6') });
    // keyboard.data.push(MapCell { value: Some('1') });
    // keyboard.data.push(MapCell { value: Some('2') });
    // keyboard.data.push(MapCell { value: Some('3') });
    // keyboard.data.push(MapCell { value: None });
    // keyboard.data.push(MapCell { value: Some('0') });
    // keyboard.data.push(MapCell { value: Some('A') });

    //keyboard.print();

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

        if current_index.y > target_index.y {
            // need to go up then left-right
            for _dy in 0..(current_index.y - target_index.y) {
                result.push('^');
            }

            let second_command = if current_index.x < target_index.x {
                '>'
            } else {
                '<'
            };

            for _dx in 0..(current_index.x.abs_diff(target_index.x)) {
                result.push(second_command);
            }
        } else {
            // right-left and then down
            let second_command = if current_index.x < target_index.x {
                '>'
            } else {
                '<'
            };

            for _dx in 0..(current_index.x.abs_diff(target_index.x)) {
                result.push(second_command);
            }

            for _dy in 0..(target_index.y - current_index.y) {
                result.push('v');
            }
        }
        result.push('A');
        current_index = *target_index;
    }

    result
}

#[derive(Debug, Clone)]
struct MapCell {
    value: Option<char>,
    // index: Index,
}

#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
struct Size {
    x: usize,
    y: usize,
}

#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq, PartialOrd, Ord)]
struct Index {
    x: usize,
    y: usize,
}

impl Index {
    fn up<T>(&self, _matrix: &Matrix<T>) -> Option<Index> {
        if self.y == 0 {
            return None;
        }
        Some(Index {
            x: self.x,
            y: self.y - 1,
        })
    }

    fn left<T>(&self, _matrix: &Matrix<T>) -> Option<Index> {
        if self.x == 0 {
            return None;
        }
        Some(Index {
            x: self.x - 1,
            y: self.y,
        })
    }

    fn down<T>(&self, matrix: &Matrix<T>) -> Option<Index> {
        if self.y == matrix.size.y - 1 {
            return None;
        }
        Some(Index {
            x: self.x,
            y: self.y + 1,
        })
    }

    fn right<T>(&self, matrix: &Matrix<T>) -> Option<Index> {
        if self.x == matrix.size.x - 1 {
            return None;
        }
        Some(Index {
            x: self.x + 1,
            y: self.y,
        })
    }

    fn navigate_to<T>(&self, matrix: &Matrix<T>, direction: &Direction) -> Option<Index> {
        match direction {
            Direction::Up => self.up(matrix),
            Direction::Down => self.down(matrix),
            Direction::Left => self.left(matrix),
            Direction::Right => self.right(matrix),
        }
    }
}

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy, PartialOrd, Ord, EnumIter)]
enum Direction {
    Up,
    Left,
    Right,
    Down,
}

impl Direction {
    #[allow(dead_code)]
    fn reverse(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            Direction::Down => Direction::Up,
        }
    }

    #[allow(dead_code)]
    fn turn_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    #[allow(dead_code)]
    fn turn_left(&self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
        }
    }
}

#[derive(Debug, Clone)]
struct Matrix<T> {
    size: Size,
    data: Vec<T>,
}

impl<T> Matrix<T> {
    fn get_index_from_position(&self, indx: usize) -> Index {
        let y = indx / self.size.x;
        let x = indx - y * self.size.x;
        Index { x, y }
    }
}

impl Matrix<MapCell> {
    fn has_index(&self, index: &Index) -> bool {
        self.size.x > index.x && self.size.y > index.y
    }

    #[allow(dead_code)]
    fn print(&self) {
        for y in 0..self.size.y {
            for x in 0..self.size.x {
                let mut ch = '.';
                if let Some(x) = self[y][x].value {
                    ch = x
                }
                print!("{}", ch);
            }
            println!();
        }
    }
}

impl Matrix<bool> {
    #[allow(dead_code)]
    fn print(&self) {
        for y in 0..self.size.y {
            for x in 0..self.size.x {
                let mut ch = '.';
                if self[y][x] {
                    ch = '0'
                }
                print!("{}", ch);
            }
            println!();
        }
    }
}

impl<T> std::ops::Index<usize> for Matrix<T> {
    type Output = [T];

    fn index(&self, row: usize) -> &[T] {
        let start = row * self.size.x;
        &self.data[start..start + self.size.x]
    }
}

impl<T> std::ops::IndexMut<usize> for Matrix<T> {
    fn index_mut(&mut self, row: usize) -> &mut [T] {
        let start = row * self.size.x;
        &mut self.data[start..start + self.size.x]
    }
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
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
