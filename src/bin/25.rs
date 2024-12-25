use std::fmt::Debug;
use std::str::FromStr;

advent_of_code::solution!(25);

pub fn part_one(input: &str) -> Option<u64> {
    let mut data: Vec<&str> = input.split("\r\n\r\n").collect();
    if data.len() < 2 {
        // Actual data split.
        data = input.split("\n\n").collect();
    }

    let mut locks = Vec::<Lock>::new();
    let mut keys = Vec::<Key>::new();
    for entry in data {
        let parsed = Data::from(entry);
        match parsed {
            Data::KeyData(key) => keys.push(key),
            Data::LockData(lock) => locks.push(lock),
        }
    }

    let mut result = 0;
    for key in &keys {
        for lock in &locks {
            if key.matches_lock(lock) {
                result += 1;
            }
        }
    }

    Some(result)
}

pub fn part_two(_input: &str) -> Option<u64> {
    None
}

enum Data {
    KeyData(Key),
    LockData(Lock),
}

impl From<&str> for Data {
    fn from(input: &str) -> Self {
        let (data, size) = parse_row_input_as_data_array::<char>(input);
        let matrix = Matrix { size, data };
        let mut heights: Vec<u8> = vec![];
        for x in 0..size.x {
            let mut colomn_height = 0;
            for y in 0..size.y {
                if matrix[y][x] == '#' {
                    colomn_height += 1;
                }
            }
            heights.push(colomn_height - 1);
        }

        if matrix[0][0] == '#' {
            Data::LockData(Lock { heights })
        } else {
            Data::KeyData(Key { heights })
        }
    }
}

struct Key {
    heights: Vec<u8>,
}

impl Key {
    fn matches_lock(&self, lock: &Lock) -> bool {
        for h in 0..self.heights.len() {
            if self.heights[h] + lock.heights[h] > 5 {
                return false;
            }
        }
        true
    }
}

struct Lock {
    heights: Vec<u8>,
}

struct Matrix<T> {
    size: Size,
    data: Vec<T>,
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

fn parse_row_input_as_data_array<T>(input: &str) -> (Vec<T>, Size)
where
    T: FromStr + std::fmt::Debug,
    <T as FromStr>::Err: Debug,
{
    let splitted_lines: Vec<&str> = input.lines().collect();
    let size_y = splitted_lines.len();
    let size_x = splitted_lines[0].len();

    let result: Vec<T> = input
        .chars()
        .filter(|c| *c != '\n' && *c != '\r')
        .map(|c| c.to_string().parse().expect("T values expected"))
        .collect();

    (
        result,
        Size {
            x: size_x,
            y: size_y,
        },
    )
}

#[derive(Debug, Copy, Clone)]
struct Size {
    x: usize,
    y: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
