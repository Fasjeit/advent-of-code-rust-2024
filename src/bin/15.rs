use std::fmt::Debug;
use std::str::FromStr;

advent_of_code::solution!(15);

pub fn part_one(input: &str) -> Option<u64> {
    let mut data: Vec<&str> = input.split("\r\n\r\n").collect();
    if data.len() < 2 {
        // Actual data split.
        data = input.split("\n\n").collect();
    }
    let map_data = data[0];
    let commands_data = data[1];

    let (data, size) = parse_row_input_as_data_array::<char>(map_data);
    let data_cells: Vec<MapCell> = data.into_iter().map(MapCell::from).collect();

    let (command_data, command_size) = parse_row_input_as_data_array::<char>(commands_data);
    let commands: Vec<Direction> = command_data.into_iter().map(Direction::from).collect();

    let robot_position_index = data_cells
        .iter()
        .position(|c| c.has_robot())
        .expect("Cannot determine guard position!");

    let mut matrix = Matrix {
        size,
        data: data_cells,
    };

    let mut robot_position_matrix_index = matrix.get_index_from_position(robot_position_index);

    //matrix.print();
    //dbg!(&commands);
    step_robot(&mut matrix, &mut robot_position_matrix_index, &commands);
    // matrix.print();

    Some(compute_gps(&matrix))
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

fn step_robot(matrix: &mut Matrix<MapCell>, mut robot_index: &mut Index, commands: &[Direction]) {
    for command in commands {
        // If wall on the way - continue;
        // Unwrap - always enclosed with walls, so no panic here;
        let next_robot_index = robot_index.navigate_to(matrix, command).unwrap();
        if matrix[next_robot_index.y][next_robot_index.x].has_wall() {
            continue;
        }

        // if Empty - just move robot
        if matrix[next_robot_index.y][next_robot_index.x].empty() {
            matrix[robot_index.y][robot_index.x].cell_state = CellState::Cell(Content::Empty);
            robot_index.x = next_robot_index.x;
            robot_index.y = next_robot_index.y;
            matrix[robot_index.y][robot_index.x].cell_state = CellState::Cell(Content::Robot);
        }

        // If box and can move - move box and robot
        if matrix[next_robot_index.y][next_robot_index.x].has_box() {
            let mut found_spot = false;
            let mut box_spot_box_index = next_robot_index;

            // move in direction, until empty cell met, set this cell index as next box index
            loop {
                // next index
                box_spot_box_index = box_spot_box_index.navigate_to(matrix, command).unwrap();
                if matrix[box_spot_box_index.y][box_spot_box_index.x].has_wall() {
                    // cannot move
                    break;
                }
                if matrix[box_spot_box_index.y][box_spot_box_index.x].has_box() {
                    continue;
                }

                if matrix[box_spot_box_index.y][box_spot_box_index.x].empty() {
                    found_spot = true;
                    break;
                }
            }

            if !found_spot {
                continue;
            }

            // move box
            matrix[box_spot_box_index.y][box_spot_box_index.x].cell_state =
                CellState::Cell(Content::Box);

            // move robot
            matrix[robot_index.y][robot_index.x].cell_state = CellState::Cell(Content::Empty);
            robot_index.x = next_robot_index.x;
            robot_index.y = next_robot_index.y;
            matrix[robot_index.y][robot_index.x].cell_state = CellState::Cell(Content::Robot);
        }

        //matrix.print();
        //println!();
    }
}

fn compute_gps(map: &Matrix<MapCell>) -> u64 {
    let mut result = 0;
    for y in 0..map.size.y {
        for x in 0..map.size.x {
            if let CellState::Cell(Content::Box) = map[y][x].cell_state {
                result += 100 * y + x;
            }
        }
    }
    result as u64
}

#[derive(Debug, Clone)]
enum Content {
    Empty,
    Robot,
    Box,
}

#[derive(Debug, Clone)]
enum CellState {
    Wall,
    Cell(Content),
}

#[derive(Debug, Clone)]
struct MapCell {
    cell_state: CellState,
}

impl MapCell {
    fn new(has_wall: bool, has_box: bool) -> Self {
        match (has_wall, has_box) {
            (true, _) => MapCell {
                cell_state: { CellState::Wall },
            },
            (false, true) => MapCell {
                cell_state: { CellState::Cell(Content::Box) },
            },
            (false, false) => MapCell {
                cell_state: { CellState::Cell(Content::Empty) },
            },
        }
    }

    fn new_with_robot() -> Self {
        MapCell {
            cell_state: { CellState::Cell(Content::Robot) },
        }
    }

    fn has_wall(&self) -> bool {
        if let CellState::Wall = self.cell_state {
            return true;
        }
        return false;
    }

    fn empty(&self) -> bool {
        if let CellState::Cell(Content::Empty) = self.cell_state {
            return true;
        }
        return false;
    }

    fn has_robot(&self) -> bool {
        if let CellState::Cell(Content::Robot) = self.cell_state {
            return true;
        }
        return false;
    }

    fn has_box(&self) -> bool {
        if let CellState::Cell(Content::Box) = self.cell_state {
            return true;
        }
        return false;
    }
}

impl From<char> for MapCell {
    fn from(value: char) -> Self {
        match value {
            '#' => MapCell::new(true, false),
            'O' => MapCell::new(false, true),
            '.' => MapCell::new(false, false),
            '@' => MapCell::new_with_robot(),
            _ => panic!("Unknown char in map data!"),
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

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
enum Direction {
    Up,
    Left,
    Right,
    Down,
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            '^' => Direction::Up,
            '<' => Direction::Left,
            '>' => Direction::Right,
            'v' => Direction::Down,
            _ => panic!("Unknown char in commands data!"),
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
        let y = indx / self.size.y;
        let x = indx - y * self.size.y;
        Index { x, y }
    }
}

impl Matrix<MapCell> {
    #[allow(dead_code)]
    fn print(&self) {
        for y in 0..self.size.y {
            for x in 0..self.size.x {
                let mut ch = '.';
                match &self[y][x].cell_state {
                    CellState::Wall => ch = '#',
                    CellState::Cell(content) => match content {
                        Content::Empty => (),
                        Content::Robot => ch = '@',
                        Content::Box => ch = 'O',
                    },
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
        assert_eq!(result, Some(2028));
    }

    #[test]
    fn test_part_one_2() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(10092));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
