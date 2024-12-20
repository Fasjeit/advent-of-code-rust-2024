use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::fmt::Debug;
use std::str::FromStr;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

advent_of_code::solution!(20);

pub fn part_one(input: &str) -> Option<u64> {
    solve_part_one(input, 100)
}

pub fn part_two(input: &str) -> Option<u64> {
    solve_part_two(input, 100, 20)
}

fn solve_part_one(input: &str, threshold: u64) -> Option<u64> {
    // main idea by https://www.reddit.com/r/adventofcode/comments/1hicdtb/comment/m2yaik2/
    // Implement "reverse Dijkstra" by starting at the end and find paths to all cells.
    // Find the cells achievable by cheat, compare cost diffs.

    let (data, size) = parse_row_input_as_data_array::<char>(input);
    let data_cells: Vec<MapCell> = data.into_iter().map(MapCell::from).collect();

    let mut matrix = Matrix {
        size,
        data: data_cells.clone(),
    };

    // matrix.print();
    // println!();

    let end_position_index = data_cells
        .iter()
        .position(|c| c.target)
        .expect("Cannot determine guard position!");

    let end_position_matrix_index = matrix.get_index_from_position(end_position_index);

    let mut to_visit_set = BinaryHeap::new();
    to_visit_set.push(Reverse((0_u64, end_position_matrix_index)));

    let _result = pseudo_dijkstra(&mut matrix, None, &mut to_visit_set);

    // matrix.print();
    // println!();

    // let index_to_check = Index { x: 1, y: 11 };
    // let test = check_cheat_cuts(&matrix, &index_to_check, 1);
    let mut acc = 0;
    for (i, cell) in matrix.data.iter().enumerate() {
        if !cell.has_wall {
            let index = matrix.get_index_from_position(i);
            acc += check_cheat_cuts(&matrix, &index, threshold);
        }
    }

    Some(acc)
}

fn check_cheat_cuts(matrix: &Matrix<MapCell>, index: &Index, threshold: u64) -> u64 {
    let mut total_cheats = 0;
    let current_cell = &matrix[index.y][index.x];

    for direction in Direction::iter() {
        if let Some(next_index_2) = index
            .navigate_to(matrix, &direction)
            .and_then(|next_index_1| next_index_1.navigate_to(matrix, &direction))
        {
            let cheat_cost = 2;
            if !matrix[next_index_2.y][next_index_2.x].has_wall
                && matrix[next_index_2.y][next_index_2.x].cost != u64::MAX
            {
                // have reachable cell
                let cheat_cell = &matrix[next_index_2.y][next_index_2.x];
                if cheat_cell.cost > current_cell.cost
                    && cheat_cell.cost - current_cell.cost >= threshold + cheat_cost
                {
                    total_cheats += 1;
                    // println!(
                    //     "cheat with saved cost [{}] found at [{:#?}]-[{:#?}]",
                    //     cheat_cell.cost - current_cell.cost,
                    //     index,
                    //     next_index_1
                    // );
                }
            }
        }
    }

    total_cheats
}

fn solve_part_two(input: &str, threshold: u64, cheat_len: u64) -> Option<u64> {
    // same as part 1, but have a cheat of len 20, so need to find all
    // cells reachable with that length, ignoring the walls

    let (data, size) = parse_row_input_as_data_array::<char>(input);
    let data_cells: Vec<MapCell> = data.into_iter().map(MapCell::from).collect();

    let mut matrix = Matrix {
        size,
        data: data_cells.clone(),
    };

    // matrix.print();
    // println!();

    let end_position_index = data_cells
        .iter()
        .position(|c| c.target)
        .expect("Cannot determine guard position!");

    let end_position_matrix_index = matrix.get_index_from_position(end_position_index);

    let mut to_visit_set = BinaryHeap::new();
    to_visit_set.push(Reverse((0_u64, end_position_matrix_index)));

    let _result = pseudo_dijkstra(&mut matrix, None, &mut to_visit_set);

    // matrix.print();
    // println!();

    // let index_to_check = Index { x: 3, y: 7 };
    // let test = check_cheat_cuts_any_len(&matrix, &index_to_check, threshold, cheat_len);
    // todo!();

    let mut acc = 0;
    for (i, cell) in matrix.data.iter().enumerate() {
        if !cell.has_wall {
            let index = matrix.get_index_from_position(i);
            acc += check_cheat_cuts_any_len(&matrix, &index, threshold, cheat_len);
        }
    }

    Some(acc)
}

fn check_cheat_cuts_any_len(
    matrix: &Matrix<MapCell>,
    index: &Index,
    threshold: u64,
    cheat_len: u64,
) -> u64 {
    let mut total_cheats = 0;
    let current_cell = &matrix[index.y][index.x];

    // all reachable cells are in "diamond" shape
    // with max x + y <= cheatlen

    let dy_range = cheat_len as i64;
    for dy in -dy_range..=dy_range {
        let dx_range = cheat_len as i64 - dy.abs();
        for dx in -dx_range..=dx_range {
            // if dx == -2 && dy == -4 {
            //     println!("DBG!");
            // }

            if dx == 0 && dy == 0 {
                continue;
            }

            let next_index_1 = Index {
                x: (index.x as i64 + dx) as usize,
                y: (index.y as i64 + dy) as usize,
            };

            if !matrix.has_index(&next_index_1) {
                continue;
            }

            let cheat_cell = &matrix[next_index_1.y][next_index_1.x];

            // same check as for part 1
            if !cheat_cell.has_wall && cheat_cell.cost != u64::MAX {
                // have reachable cell
                let cheat_cost = dx.unsigned_abs() + dy.unsigned_abs();
                let saved_cost =
                    cheat_cell.cost as i64 - current_cell.cost as i64 - cheat_cost as i64;
                if saved_cost >= threshold as i64 {
                    total_cheats += 1;
                    // println!(
                    //     "cheat with saved cost [{}] of len [{cheat_cost}] found at [{:#?}]-[{:#?}]",
                    //     saved_cost, index, next_index_1
                    // );
                }
            }
        }
    }

    total_cheats
}

fn pseudo_dijkstra(
    matrix: &mut Matrix<MapCell>,
    ending_position: Option<&Index>,
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

        if let Some(ending_position) = ending_position {
            if index == *ending_position {
                return Some(cost);
            }
        }

        if let Some(next_index) = index.navigate_to(matrix, &Direction::Down) {
            if !matrix[next_index.y][next_index.x].has_wall()
                && matrix[next_index.y][next_index.x].cost >= (cost + 1)
            {
                to_visit_set.push(Reverse((cost + 1, next_index)));
            }
        }

        if let Some(next_index) = index.navigate_to(matrix, &Direction::Right) {
            if !matrix[next_index.y][next_index.x].has_wall()
                && matrix[next_index.y][next_index.x].cost >= (cost + 1)
            {
                to_visit_set.push(Reverse((cost + 1, next_index)));
            }
        }

        if let Some(next_index) = index.navigate_to(matrix, &Direction::Left) {
            if !matrix[next_index.y][next_index.x].has_wall()
                && matrix[next_index.y][next_index.x].cost >= (cost + 1)
            {
                to_visit_set.push(Reverse((cost + 1, next_index)));
            }
        }

        if let Some(next_index) = index.navigate_to(matrix, &Direction::Up) {
            if !matrix[next_index.y][next_index.x].has_wall()
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
    has_wall: bool,
    source: bool,
    target: bool,
    cost: u64,
}

impl MapCell {
    fn new(has_wall: bool) -> Self {
        MapCell {
            has_wall,
            source: false,
            target: false,
            cost: { u64::MAX },
        }
    }

    fn new_deer() -> Self {
        MapCell {
            has_wall: false,
            source: true,
            target: false,
            cost: { u64::MAX },
        }
    }

    fn new_target() -> Self {
        MapCell {
            has_wall: false,
            source: false,
            target: true,
            cost: { u64::MAX },
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
    fn has_index(&self, index: &Index) -> bool {
        self.size.x > index.x && self.size.y > index.y
    }

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
                } else if self[y][x].cost < 10 {
                    ch = self[y][x].cost.to_string().chars().collect::<Vec<char>>()[0]
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
        let result = solve_part_one(&advent_of_code::template::read_file("examples", DAY), 20);
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_one_2() {
        let result = solve_part_one(&advent_of_code::template::read_file("examples", DAY), 2);
        assert_eq!(result, Some(44));
    }

    #[test]
    fn test_part_one_3() {
        let result = solve_part_one(&advent_of_code::template::read_file("examples", DAY), 64);
        assert_eq!(result, Some(1));
    }

    #[test]
    fn test_part_two_1() {
        let result = solve_part_two(
            &advent_of_code::template::read_file("examples", DAY),
            76,
            20,
        );
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two_2() {
        let result = solve_part_two(
            &advent_of_code::template::read_file("examples", DAY),
            74,
            20,
        );
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two_3() {
        let result = solve_part_two(
            &advent_of_code::template::read_file("examples", DAY),
            50,
            20,
        );
        assert_eq!(result, Some(285));
    }

    #[test]
    fn test_part_two_4() {
        let result = solve_part_two(&advent_of_code::template::read_file("examples", DAY), 50, 4);
        assert_eq!(result, Some(12));
    }
}
