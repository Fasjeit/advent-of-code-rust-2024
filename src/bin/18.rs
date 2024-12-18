use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::fmt::Debug;
use strum_macros::EnumIter;

advent_of_code::solution!(18);

pub fn part_one(input: &str) -> Option<u64> {
    let size = Size { x: 71, y: 71 };
    let bytes_number = 1024;
    solve_part_1(input, size, bytes_number)
}

pub fn part_two(input: &str) -> Option<String> {
    let size = Size { x: 71, y: 71 };
    let bytes_number = 1024;
    solve_part_2(input, size, bytes_number)
}

#[allow(clippy::let_and_return)]
fn solve_part_1(input: &str, size: Size, bytes_number: usize) -> Option<u64> {
    let bytes_fall = parse_bytes_fall(input);

    let data: Vec<char> = vec!['.'; size.x * size.y];
    let data_cells: Vec<MapCell> = data.into_iter().map(MapCell::from).collect();

    let mut matrix = Matrix {
        size,
        data: data_cells.clone(),
    };

    //matrix.print();
    //println!();

    // simulate bytes fall
    for byte_pos in bytes_fall.iter().take(bytes_number) {
        matrix[byte_pos.y][byte_pos.x].has_byte = true;
    }

    //matrix.print();

    let start_index = Index { x: 0, y: 0 };
    let end_index = Index {
        x: size.x - 1,
        y: size.y - 1,
    };
    let mut to_visit_set = BinaryHeap::new();
    to_visit_set.push(Reverse((0_u64, start_index)));

    let result = pseudo_dijkstra(&mut matrix, &end_index, &mut to_visit_set);

    result
}

fn solve_part_2(input: &str, size: Size, bytes_number_start: usize) -> Option<String> {
    // same as part 1, but finding the first byte_number that make path unreachable.
    // skipping first bytes_number_start iterations.

    let bytes_fall = parse_bytes_fall(input);

    let data: Vec<char> = vec!['.'; size.x * size.y];
    let data_cells: Vec<MapCell> = data.into_iter().map(MapCell::from).collect();

    let mut matrix = Matrix {
        size,
        data: data_cells.clone(),
    };

    //matrix.print();
    //println!();

    // simulate bytes fall
    for byte_pos in bytes_fall.iter().take(bytes_number_start) {
        matrix[byte_pos.y][byte_pos.x].has_byte = true;
    }

    //matrix.print();
    //println!("After initial fall");

    let start_index = Index { x: 0, y: 0 };
    let end_index = Index {
        x: size.x - 1,
        y: size.y - 1,
    };
    let mut to_visit_set = BinaryHeap::new();
    to_visit_set.push(Reverse((0_u64, start_index)));

    let mut next_byte_index = bytes_number_start;
    matrix.data.iter_mut().for_each(|c| c.cost = u64::MAX);

    //matrix.print();
    //println!();

    #[allow(clippy::partialeq_to_none)]
    while None != pseudo_dijkstra(&mut matrix, &end_index, &mut to_visit_set) {
        let byte_pos = bytes_fall[next_byte_index];
        matrix[byte_pos.y][byte_pos.x].has_byte = true;

        next_byte_index += 1;

        //matrix.print();
        //println!();

        // reset matrix path info.
        matrix.data.iter_mut().for_each(|c| c.cost = u64::MAX);
        to_visit_set = BinaryHeap::new();
        to_visit_set.push(Reverse((0_u64, start_index)));
    }

    //dbg!(next_byte_index - 1);

    let byte_pos = bytes_fall[next_byte_index - 1];

    Some(format!("{},{}", byte_pos.x, byte_pos.y))
}

fn pseudo_dijkstra(
    matrix: &mut Matrix<MapCell>,
    ending_position: &Index,
    to_visit_set: &mut BinaryHeap<Reverse<(u64, Index)>>,
) -> Option<u64> {
    let mut safe_counter = 100000;

    while let Some(Reverse((cost, index))) = to_visit_set.pop() {
        if safe_counter <= 0 {
            panic!("Safe counter stop.");
        }
        safe_counter -= 1;

        if matrix[index.y][index.x].cost != u64::MAX {
            assert!(matrix[index.y][index.x].cost <= cost);
            continue;
        }

        matrix[index.y][index.x].cost = cost;

        //dbg!(&index);

        if index == *ending_position {
            return Some(cost);
        }

        if let Some(next_index) = index.navigate_to(matrix, &Direction::Down) {
            if !matrix[next_index.y][next_index.x].has_byte()
                && matrix[next_index.y][next_index.x].cost >= (cost + 1)
            {
                to_visit_set.push(Reverse((cost + 1, next_index)));
            }
        }

        if let Some(next_index) = index.navigate_to(matrix, &Direction::Right) {
            if !matrix[next_index.y][next_index.x].has_byte()
                && matrix[next_index.y][next_index.x].cost >= (cost + 1)
            {
                to_visit_set.push(Reverse((cost + 1, next_index)));
            }
        }

        if let Some(next_index) = index.navigate_to(matrix, &Direction::Left) {
            if !matrix[next_index.y][next_index.x].has_byte()
                && matrix[next_index.y][next_index.x].cost >= (cost + 1)
            {
                to_visit_set.push(Reverse((cost + 1, next_index)));
            }
        }

        if let Some(next_index) = index.navigate_to(matrix, &Direction::Up) {
            if !matrix[next_index.y][next_index.x].has_byte()
                && matrix[next_index.y][next_index.x].cost >= (cost + 1)
            {
                to_visit_set.push(Reverse((cost + 1, next_index)));
            }
        }
    }

    None
}

#[derive(Debug, Clone)]
struct MapCell {
    visited_backtrack: bool,
    has_byte: bool,
    cost: u64,
}

impl MapCell {
    fn new(has_byte: bool) -> Self {
        MapCell {
            visited_backtrack: false,
            has_byte,
            cost: u64::MAX,
        }
    }

    fn has_byte(&self) -> bool {
        self.has_byte
    }
}

impl From<char> for MapCell {
    fn from(value: char) -> Self {
        match value {
            '.' => MapCell::new(false),
            '#' => MapCell::new(true),
            _ => panic!("Unknown char in map data!"),
        }
    }
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
    #[allow(dead_code)]
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
                if self[y][x].has_byte() {
                    ch = '#'
                } else if self[y][x].visited_backtrack {
                    ch = 'O'
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

fn parse_bytes_fall(input: &str) -> Vec<Index> {
    let mut result = Vec::new();
    input.lines().for_each(|l| {
        let mut line_data = l.split(',');
        let byte_pos = Index {
            x: line_data
                .next()
                .expect("Expected data")
                .parse()
                .expect("Expected int"),
            y: line_data
                .next()
                .expect("Expected data")
                .parse()
                .expect("Expected int"),
        };
        result.push(byte_pos);
    });

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_1() {
        let size = Size { x: 7, y: 7 };
        let result = solve_part_1(
            &advent_of_code::template::read_file("examples", DAY),
            size,
            12,
        );
        assert_eq!(result, Some(22));
    }

    #[test]
    fn test_part_one_2() {
        let size = Size { x: 7, y: 7 };
        let result = solve_part_1(
            &advent_of_code::template::read_file("examples", DAY),
            size,
            19,
        );
        assert_eq!(result, Some(24));
    }

    #[test]
    fn test_part_two() {
        let size = Size { x: 7, y: 7 };
        let result = solve_part_2(
            &advent_of_code::template::read_file("examples", DAY),
            size,
            12,
        );
        assert_eq!(result, Some("6,1".to_string()));
    }
}
