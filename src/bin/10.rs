use std::collections::HashSet;
use std::fmt::Debug;
use std::str::FromStr;

advent_of_code::solution!(10);

pub fn part_one(input: &str) -> Option<u32> {
    let (data, size) = parse_row_input_as_data_array::<char>(input);
    let data_cells: Vec<MapCell> = data.into_iter().map(MapCell::from).collect();

    let mut matrix = Matrix {
        size,
        data: data_cells.clone(),
    };

    // if cfg!(debug_assertions) {
    //     matrix.print();
    // }

    // find starting locations
    let staring_data: Vec<usize> = matrix
        .data
        .iter()
        .enumerate()
        .filter(|(_, c)| c.hight == 0)
        .map(|(i, _)| i) // Collect only indices
        .collect();

    //dbg!(&staring_data);

    let mut acc = 0;
    for index in staring_data {
        //let matrix = matrix.clone();
        let matrix_index = matrix.get_index_from_position(index);
        dfs(&mut matrix, matrix_index, &mut acc, index);

        //dbg!(&acc);
    }

    //matrix.print();

    Some(acc)
}

pub fn part_two(input: &str) -> Option<u32> {
    // same as part 1, but different dfs.

    let (data, size) = parse_row_input_as_data_array::<char>(input);
    let data_cells: Vec<MapCell> = data.into_iter().map(MapCell::from).collect();

    let mut matrix = Matrix {
        size,
        data: data_cells.clone(),
    };

    // if cfg!(debug_assertions) {
    //     matrix.print();
    // }

    // find starting locations
    let staring_data: Vec<usize> = matrix
        .data
        .iter()
        .enumerate()
        .filter(|(_, c)| c.hight == 0)
        .map(|(i, _)| i) // Collect only indices
        .collect();

    //dbg!(&staring_data);

    let mut acc = 0;
    for index in staring_data {
        //let matrix = matrix.clone();
        let matrix_index = matrix.get_index_from_position(index);

        dfs2(&mut matrix, matrix_index, &mut acc, index);

        //dbg!(&acc);
    }

    //matrix.print();

    Some(acc)
}

fn dfs(
    matrix: &mut Matrix<MapCell>,
    index: Index,
    trail_counter: &mut u32,
    start_index: usize,
) -> bool {
    let current = &matrix[index.y][index.x].clone();

    //dbg!(&index);
    //dbg!(&current.hight);

    if current.hight == 9 {
        if !matrix[index.y][index.x]
            .on_trail_from
            .contains(&start_index)
        {
            // new finish!
            matrix[index.y][index.x].on_trail_from.insert(start_index);
            *trail_counter += 1;
        }
        return true;
    }

    if current.on_trail_from.contains(&start_index) {
        // already on trail, no need to precess.
        return true;
    }

    let mut total_result = false;

    // Up
    if index.y > 0 && matrix[index.y - 1][index.x].hight == current.hight + 1 {
        let rec_result = dfs(
            matrix,
            Index {
                x: index.x,
                y: index.y - 1,
            },
            trail_counter,
            start_index,
        );
        if rec_result {
            total_result = true;
            matrix[index.y][index.x].on_trail_from.insert(start_index);
        }
    }
    // Down
    if index.y < matrix.size.y - 1 && matrix[index.y + 1][index.x].hight == current.hight + 1 {
        let rec_result = dfs(
            matrix,
            Index {
                x: index.x,
                y: index.y + 1,
            },
            trail_counter,
            start_index,
        );
        if rec_result {
            total_result = true;
            matrix[index.y][index.x].on_trail_from.insert(start_index);
        }
    }
    // Right
    if index.x < matrix.size.x - 1 && matrix[index.y][index.x + 1].hight == current.hight + 1 {
        let rec_result = dfs(
            matrix,
            Index {
                x: index.x + 1,
                y: index.y,
            },
            trail_counter,
            start_index,
        );
        if rec_result {
            total_result = true;
            matrix[index.y][index.x].on_trail_from.insert(start_index);
        }
    }
    // Left
    if index.x > 0 && matrix[index.y][index.x - 1].hight == current.hight + 1 {
        let rec_result = dfs(
            matrix,
            Index {
                x: index.x - 1,
                y: index.y,
            },
            trail_counter,
            start_index,
        );
        if rec_result {
            total_result = true;
            matrix[index.y][index.x].on_trail_from.insert(start_index);
        }
    }

    total_result
}

fn dfs2(
    matrix: &mut Matrix<MapCell>,
    index: Index,
    trail_counter: &mut u32,
    start_index: usize,
) -> bool {
    let current = &matrix[index.y][index.x].clone();

    // same as part 1, but with total trail counter
    // and without early stop due to already visited!

    //dbg!(&index);
    //dbg!(&current.hight);

    if current.hight == 9 {
        matrix[index.y][index.x].on_trail_from.insert(start_index);
        //dbg!("New finish!");
        *trail_counter += 1;
        return true;
    }

    let mut total_result = false;

    // Up
    if index.y > 0 && matrix[index.y - 1][index.x].hight == current.hight + 1 {
        let rec_result = dfs2(
            matrix,
            Index {
                x: index.x,
                y: index.y - 1,
            },
            trail_counter,
            start_index,
        );
        if rec_result {
            total_result = true;
            matrix[index.y][index.x].on_trail_from.insert(start_index);
        }
    }
    // Down
    if index.y < matrix.size.y - 1 && matrix[index.y + 1][index.x].hight == current.hight + 1 {
        let rec_result = dfs2(
            matrix,
            Index {
                x: index.x,
                y: index.y + 1,
            },
            trail_counter,
            start_index,
        );
        if rec_result {
            total_result = true;
            matrix[index.y][index.x].on_trail_from.insert(start_index);
        }
    }
    // Right
    if index.x < matrix.size.x - 1 && matrix[index.y][index.x + 1].hight == current.hight + 1 {
        let rec_result = dfs2(
            matrix,
            Index {
                x: index.x + 1,
                y: index.y,
            },
            trail_counter,
            start_index,
        );
        if rec_result {
            total_result = true;
            matrix[index.y][index.x].on_trail_from.insert(start_index);
        }
    }
    // Left
    if index.x > 0 && matrix[index.y][index.x - 1].hight == current.hight + 1 {
        let rec_result = dfs2(
            matrix,
            Index {
                x: index.x - 1,
                y: index.y,
            },
            trail_counter,
            start_index,
        );
        if rec_result {
            total_result = true;
            matrix[index.y][index.x].on_trail_from.insert(start_index);
        }
    }

    total_result
}

#[derive(Debug, Clone)]
struct MapCell {
    hight: u32,
    on_trail_from: HashSet<usize>,
}

impl MapCell {
    fn new(hight: u32) -> Self {
        MapCell {
            hight,
            on_trail_from: HashSet::new(),
        }
    }
}

impl From<char> for MapCell {
    fn from(value: char) -> Self {
        MapCell::new(value.to_digit(10).unwrap_or(11))
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
        let y = indx / self.size.y;
        let x = indx - y * self.size.y;
        Index { x, y }
    }
}

impl Matrix<MapCell> {
    #[allow(dead_code)]
    fn print(&self) {
        for y in 0..self.size.y {
            for x in 0..self.size.y {
                let mut ch = self[y][x].hight.to_string();
                if self[y][x].hight == 11 {
                    ch = "~".to_string();
                } else if self[y][x].on_trail_from.is_empty() {
                    ch = ".".to_string();
                }
                // if let Some(Tower { freq }) = self[y][x].tower {
                //     if self[y][x].have_node {
                //         ch = '⊕'
                //     } else {
                //         ch = freq;
                //     }
                // } else if self[y][x].have_node {
                //     ch = '#';
                // }
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
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
