use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, VecDeque};
use std::fmt::Debug;
use std::str::FromStr;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

advent_of_code::solution!(16);

pub fn part_one(input: &str) -> Option<u64> {
    let (data, size) = parse_row_input_as_data_array::<char>(input);
    let data_cells: Vec<MapCell> = data.into_iter().map(MapCell::from).collect();

    let mut matrix = Matrix {
        size,
        data: data_cells.clone(),
    };

    let deer_position_index = data_cells
        .iter()
        .position(|c| c.source)
        .expect("Cannot determine guard position!");

    let deer_position_matrix_index = matrix.get_index_from_position(deer_position_index);

    let end_position_index = data_cells
        .iter()
        .position(|c| c.target)
        .expect("Cannot determine guard position!");

    let end_position_matrix_index = matrix.get_index_from_position(end_position_index);

    let mut to_visit_set = BinaryHeap::new();
    to_visit_set.push(Reverse((
        0_u64,
        deer_position_matrix_index,
        Direction::Right,
    )));

    //matrix.print();

    let result =
        pseudo_dijkstra_step::<true>(&mut matrix, &end_position_matrix_index, &mut to_visit_set);

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (data, size) = parse_row_input_as_data_array::<char>(input);
    let data_cells: Vec<MapCell> = data.into_iter().map(MapCell::from).collect();

    let mut matrix = Matrix {
        size,
        data: data_cells.clone(),
    };

    let deer_position_index = data_cells
        .iter()
        .position(|c| c.source)
        .expect("Cannot determine guard position!");

    let deer_position_matrix_index = matrix.get_index_from_position(deer_position_index);

    let end_position_index = data_cells
        .iter()
        .position(|c| c.target)
        .expect("Cannot determine guard position!");

    let end_position_matrix_index = matrix.get_index_from_position(end_position_index);

    let mut to_visit_set = BinaryHeap::new();
    to_visit_set.push(Reverse((
        0_u64,
        deer_position_matrix_index,
        Direction::Right,
    )));

    let result =
        pseudo_dijkstra_step::<false>(&mut matrix, &end_position_matrix_index, &mut to_visit_set);

    if cfg!(debug_assertions) {
        matrix.print();
    }

    Some(result)
}

fn pseudo_dijkstra_step<const PART1: bool>(
    matrix: &mut Matrix<MapCell>,
    ending_position: &Index,
    to_visit_set: &mut BinaryHeap<Reverse<(u64, Index, Direction)>>,
) -> u64 {
    // https://github.com/smith61/advent_of_code/blob/main/src/year_2024/day_16.rs
    // helped a lot for muti-cost dijkstra... Thanks!

    // the idea is that the node have "id" of Index and direction.
    // different direction nodes treated as different nodes in graph with edge cost of direction.

    let mut safe_counter = 100000;

    let mut path_cost = u64::MAX;

    while let Some(Reverse((cost, index, direction))) = to_visit_set.pop() {
        if safe_counter <= 0 {
            panic!("Safe counter stop.");
        }
        safe_counter -= 1;

        if matrix[index.y][index.x].cost[&direction] != u64::MAX {
            assert!(matrix[index.y][index.x].cost[&direction] <= cost);
            continue;
        }

        if let Some(val) = matrix[index.y][index.x].cost.get_mut(&direction) {
            *val = cost;
        } else {
            panic!("Cannot adjust the cost.")
        }

        if index == *ending_position {
            if PART1 {
                return cost;
            }

            path_cost = path_cost.min(cost);
            continue;
        }

        if !PART1 && cost >= path_cost {
            continue;
        }

        let next_index = index.navigate_to(matrix, &direction).unwrap();
        if !matrix[next_index.y][next_index.x].has_wall()
            && matrix[next_index.y][next_index.x].cost[&direction] >= (cost + 1)
        {
            to_visit_set.push(Reverse((cost + 1, next_index, direction)));
        }

        // just turn without traversing.
        let direction_to_the_right = direction.turn_right();
        if matrix[index.y][index.x].cost[&direction_to_the_right] >= (cost + 1000) {
            to_visit_set.push(Reverse((cost + 1000, index, direction_to_the_right)));
        }

        let direction_to_the_left = direction.turn_left();
        if matrix[index.y][index.x].cost[&direction_to_the_left] >= (cost + 1000) {
            to_visit_set.push(Reverse((cost + 1000, index, direction_to_the_left)));
        }
    }

    if !PART1 {
        // backtrack to find all paths with min cost and count all nodes.
        // at each iteration just trace back to the node with needed (inverse) direction
        // and cost-1 or -1000 depending on direction.
        // part 1 already computed all the costs for us.

        let mut backtrack = VecDeque::new();
        for direction in Direction::iter() {
            if matrix[ending_position.y][ending_position.x].cost[&direction] == path_cost {
                backtrack.push_back((path_cost, ending_position.to_owned(), direction));
            }
        }

        let mut visited_count = 0;
        while let Some((minimum_cost, position, direction)) = backtrack.pop_front() {
            assert_eq!(
                matrix[position.y][position.x].cost[&direction],
                minimum_cost
            );

            if !matrix[position.y][position.x].visited_backtrack {
                matrix[position.y][position.x].visited_backtrack = true;
                visited_count += 1;
            }

            //dbg!(position);
            //dbg!(&direction);
            //dbg!(position.navigate_to(matrix, &direction.reverse()));

            let previous_position = position.navigate_to(matrix, &direction.reverse()).unwrap();

            if minimum_cost >= 1
                && matrix[previous_position.y][previous_position.x].cost[&direction]
                    == (minimum_cost - 1)
            {
                backtrack.push_back((minimum_cost - 1, previous_position, direction));
            }

            if minimum_cost >= 1000 {
                let direction_to_the_right = direction.turn_right();
                if matrix[position.y][position.x].cost[&direction_to_the_right]
                    == (minimum_cost - 1000)
                {
                    backtrack.push_back((minimum_cost - 1000, position, direction_to_the_right));
                }
                let direction_to_the_left = direction.turn_left();
                if matrix[position.y][position.x].cost[&direction_to_the_left]
                    == (minimum_cost - 1000)
                {
                    backtrack.push_back((minimum_cost - 1000, position, direction_to_the_left));
                }
            }
        }

        return visited_count;
    }

    panic!("Path not found!");
}

#[derive(Debug, Clone)]
struct MapCell {
    has_wall: bool,
    source: bool,
    target: bool,
    visited_backtrack: bool,
    cost: HashMap<Direction, u64>,
}

impl MapCell {
    fn new(has_wall: bool) -> Self {
        MapCell {
            has_wall,
            source: false,
            target: false,
            visited_backtrack: false,
            cost: {
                let mut hm = HashMap::new();
                hm.insert(Direction::Up, u64::MAX);
                hm.insert(Direction::Right, u64::MAX);
                hm.insert(Direction::Left, u64::MAX);
                hm.insert(Direction::Down, u64::MAX);
                hm
            },
        }
    }

    fn new_deer() -> Self {
        MapCell {
            has_wall: false,
            source: true,
            target: false,
            visited_backtrack: false,
            cost: {
                let mut hm = HashMap::new();
                hm.insert(Direction::Up, u64::MAX);
                hm.insert(Direction::Right, u64::MAX);
                hm.insert(Direction::Left, u64::MAX);
                hm.insert(Direction::Down, u64::MAX);
                hm
            },
        }
    }

    fn new_target() -> Self {
        MapCell {
            has_wall: false,
            source: false,
            target: true,
            visited_backtrack: false,
            cost: {
                let mut hm = HashMap::new();
                hm.insert(Direction::Up, u64::MAX);
                hm.insert(Direction::Right, u64::MAX);
                hm.insert(Direction::Left, u64::MAX);
                hm.insert(Direction::Down, u64::MAX);
                hm
            },
        }
    }

    fn has_wall(&self) -> bool {
        self.has_wall
    }
}

impl From<char> for MapCell {
    fn from(value: char) -> Self {
        match value {
            '.' => MapCell::new(false),
            '#' => MapCell::new(true),
            'S' => MapCell::new_deer(),
            'E' => MapCell::new_target(),
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
                if self[y][x].has_wall() {
                    ch = '#'
                } else if self[y][x].source {
                    ch = 'S'
                } else if self[y][x].target {
                    ch = 'E'
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
    fn test_part_one_1() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(7036));
    }

    #[test]
    fn test_part_one_2() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(11048));
    }

    #[test]
    fn test_part_two_1() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(45));
    }

    #[test]
    fn test_part_two_2() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(64));
    }
}
