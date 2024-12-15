use itertools::Itertools;
use std::collections::HashMap;
use std::fmt::Debug;
use std::str::FromStr;

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u32> {
    let (data, size) = parse_row_input_as_data_array::<char>(input);
    let data_cells: Vec<MapCell> = data.into_iter().map(MapCell::from).collect();

    let mut matrix = Matrix {
        size,
        data: data_cells.clone(),
    };

    if cfg!(debug_assertions) {
        matrix.print();
    }

    // Freq - <tower_cell, index>
    let mut freqs_towers_cells = HashMap::<char, Vec<Index>>::new();
    data_cells
        .clone()
        .into_iter()
        .enumerate()
        .for_each(|(pos, c)| {
            if let Some(tower) = c.tower {
                freqs_towers_cells
                    .entry(tower.freq)
                    .and_modify(|e| e.push(matrix.get_index_from_position(pos)))
                    .or_insert(vec![(matrix.get_index_from_position(pos))]);
            }
        });

    if cfg!(debug_assertions) {
        dbg!(&freqs_towers_cells);
    }

    for towers_and_indexes in freqs_towers_cells.values() {
        for (i1, i2) in towers_and_indexes.iter().tuple_combinations() {
            let dx = i2.x as i32 - i1.x as i32;
            let dy = i2.y as i32 - i1.y as i32;

            let node_1_pos_x = i2.x as i32 + dx;
            let node_1_pos_y = i2.y as i32 + dy;

            let node_2_pos_x = i1.x as i32 - dx;
            let node_2_pos_y = i1.y as i32 - dy;

            if (node_1_pos_x >= 0 && node_1_pos_x < matrix.size.x as i32)
                && (node_1_pos_y >= 0 && node_1_pos_y < matrix.size.y as i32)
            {
                matrix[node_1_pos_y as usize][node_1_pos_x as usize].have_node = true;
            }

            if (node_2_pos_x >= 0 && node_2_pos_x < matrix.size.x as i32)
                && (node_2_pos_y >= 0 && node_2_pos_y < matrix.size.y as i32)
            {
                matrix[node_2_pos_y as usize][node_2_pos_x as usize].have_node = true;
            }
        }
    }

    if cfg!(debug_assertions) {
        matrix.print();
    }

    let total_nodes = matrix
        .data
        .iter()
        .fold(0, |acc, c| if c.have_node { acc + 1 } else { acc });

    Some(total_nodes)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (data, size) = parse_row_input_as_data_array::<char>(input);
    let data_cells: Vec<MapCell> = data.into_iter().map(MapCell::from).collect();

    let mut matrix = Matrix {
        size,
        data: data_cells.clone(),
    };

    if cfg!(debug_assertions) {
        matrix.print();
    }

    // Freq - <tower_cell, index>
    let mut freqs_towers_cells = HashMap::<char, Vec<Index>>::new();
    data_cells
        .clone()
        .into_iter()
        .enumerate()
        .for_each(|(pos, c)| {
            if let Some(tower) = c.tower {
                freqs_towers_cells
                    .entry(tower.freq)
                    .and_modify(|e| e.push(matrix.get_index_from_position(pos)))
                    .or_insert(vec![(matrix.get_index_from_position(pos))]);
            }
        });

    if cfg!(debug_assertions) {
        dbg!(&freqs_towers_cells);
    }

    for towers_and_indexes in freqs_towers_cells.values() {
        for (i1, i2) in towers_and_indexes.iter().tuple_combinations() {
            let dx = i2.x as i32 - i1.x as i32;
            let dy = i2.y as i32 - i1.y as i32;

            let mut node_1_pos_x = i2.x as i32;
            let mut node_1_pos_y = i2.y as i32;

            // same as in part 1, but with while loop, instead of single check.

            while (node_1_pos_x >= 0 && node_1_pos_x < matrix.size.x as i32)
                && (node_1_pos_y >= 0 && node_1_pos_y < matrix.size.y as i32)
            {
                matrix[node_1_pos_y as usize][node_1_pos_x as usize].have_node = true;
                node_1_pos_x += dx;
                node_1_pos_y += dy;
            }

            let mut node_2_pos_x = i2.x as i32;
            let mut node_2_pos_y = i2.y as i32;

            while (node_2_pos_x >= 0 && node_2_pos_x < matrix.size.x as i32)
                && (node_2_pos_y >= 0 && node_2_pos_y < matrix.size.y as i32)
            {
                matrix[node_2_pos_y as usize][node_2_pos_x as usize].have_node = true;
                node_2_pos_x -= dx;
                node_2_pos_y -= dy;
            }
        }
    }

    if cfg!(debug_assertions) {
        matrix.print();
    }

    let total_nodes = matrix
        .data
        .iter()
        .fold(0, |acc, c| if c.have_node { acc + 1 } else { acc });

    Some(total_nodes)
}

#[derive(Debug, Clone, Copy)]
struct Tower {
    freq: char,
}

#[derive(Debug, Clone)]
struct MapCell {
    tower: Option<Tower>,
    have_node: bool,
}

impl MapCell {
    fn new(tower: Option<Tower>) -> Self {
        MapCell {
            tower,
            have_node: false,
        }
    }
}

impl From<char> for MapCell {
    fn from(value: char) -> Self {
        match value {
            '.' => MapCell::new(None),
            freq => MapCell::new(Some(Tower { freq })),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
struct Size {
    x: usize,
    y: usize,
}

#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
struct Index {
    x: usize,
    y: usize,
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
    #[allow(dead_code)]
    fn print(&self) {
        for y in 0..self.size.y {
            for x in 0..self.size.x {
                let mut ch = '.';
                if let Some(Tower { freq }) = self[y][x].tower {
                    if self[y][x].have_node {
                        ch = '⊕'
                    } else {
                        ch = freq;
                    }
                } else if self[y][x].have_node {
                    ch = '#';
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(34));
    }

    #[test]
    fn test_part_two_dbg() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(9));
    }
}
