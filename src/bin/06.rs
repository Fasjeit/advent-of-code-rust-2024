use std::collections::HashSet;
use std::fmt::Debug;
use std::str::FromStr;

advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u32> {
    let (data, size) = parse_row_input_as_data_array::<char>(input);
    let data_cells: Vec<MapCell> = data.into_iter().map(MapCell::from).collect();

    let guard_position_index = data_cells
        .iter()
        .position(|c| c.have_guard_visited)
        .expect("Cannot determine guard position!");

    let mut matrix = Matrix {
        size,
        data: data_cells,
    };

    let guard_position = matrix.get_index_from_position(guard_position_index);
    let mut guard = Guard::new(guard_position);
    let mut stop_counter = 10000000; // Safety value only.

    while guard.traverse(&mut matrix, None) == TraverseResult::Continue && stop_counter > 0 {
        //dbg!(guard.position);
        //dbg!(&guard.direction);
        //matrix.print();
        stop_counter -= 1;
    }

    let result = matrix
        .data
        .iter()
        .fold(0, |acc, c| if c.have_guard_visited { acc + 1 } else { acc });

    //dbg!(matrix.data);

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (data, size) = parse_row_input_as_data_array::<char>(input);
    let data_cells: Vec<MapCell> = data.into_iter().map(MapCell::from).collect();

    let guard_position_index = data_cells
        .iter()
        .position(|c| c.have_guard_visited)
        .expect("Cannot determine guard position!");

    let initial_matrix = Matrix {
        size,
        data: data_cells.clone(),
    };

    let mut matrix = initial_matrix.clone();

    let guard_initial_position = matrix.get_index_from_position(guard_position_index);
    let mut guard = Guard::new(guard_initial_position);
    let mut stop_counter = 10000000; // Safety value only.

    while guard.traverse(&mut matrix, None) == TraverseResult::Continue && stop_counter > 0 {
        //dbg!(guard.position);
        //dbg!(&guard.direction);
        //matrix.print();
        stop_counter -= 1;
    }

    let total = matrix
        .data
        .iter()
        .fold(0, |acc, c| if c.have_guard_visited { acc + 1 } else { acc });

    let possible_block_locations = matrix
        .data
        .iter()
        .enumerate()
        .filter(|(index, c)| c.have_guard_visited && *index != guard_position_index);

    let mut loop_counter = 0;
    let mut iter_counter = 1;

    for (index, change_cell) in possible_block_locations {
        println!("Solving [{} / {}]", iter_counter, total - 1); // -1 for starting pos
        iter_counter += 1;

        // create new state set for collision detection
        let mut guard_state_set: HashSet<(Index, Direction)> = HashSet::new();
        // Change cell to Obstacle.
        let mut new_cell = change_cell.clone();
        new_cell.have_obstacle = true;
        new_cell.have_added_obstacle = true;

        // Create new map with obstacle.
        let mut change_data_cells = data_cells.clone();
        change_data_cells[index] = new_cell;

        let mut changed_matrix = Matrix {
            size,
            data: change_data_cells,
        };

        // Create new guard.
        let mut guard = Guard::new(guard_initial_position);

        // Traverse until exit or loop.
        let mut stop_counter = 10000000; // Safety value only.
        while stop_counter > 0 {
            stop_counter -= 1;

            match guard.traverse(&mut changed_matrix, Some(&mut guard_state_set)) {
                TraverseResult::Continue => (),
                TraverseResult::Exit => break, // stop checking this configuration.
                TraverseResult::Loop => {
                    loop_counter += 1;

                    //changed_matrix.print();
                    //println!();

                    break;
                } // loop found!;
            };
        }
    }
    Some(loop_counter)
}

// fn check_cycles(guard_initial_position: Index, map: Matrix<MapCell>) -> bool {
//     false;
// }

#[derive(Debug, Clone)]
struct MapCell {
    have_added_obstacle: bool,
    have_obstacle: bool,
    have_guard_visited: bool,
}

impl MapCell {
    fn new(have_obstacle: bool) -> Self {
        MapCell {
            have_obstacle,
            have_added_obstacle: false,
            have_guard_visited: false,
        }
    }

    fn visit(&mut self) {
        self.have_guard_visited = true;
    }
}

impl From<char> for MapCell {
    fn from(value: char) -> Self {
        match value {
            '.' => MapCell::new(false),
            '#' => MapCell::new(true),
            '^' => {
                let mut cell = MapCell::new(false);
                cell.visit();
                cell
            }
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
                let mut ch = '.';
                if self[y][x].have_added_obstacle {
                    ch = 'O';
                } else if self[y][x].have_obstacle {
                    ch = '#';
                } else if self[y][x].have_guard_visited {
                    ch = 'X'
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

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
enum Direction {
    Up,
    Left,
    Right,
    Down,
}

#[derive(PartialEq)]
enum TraverseResult {
    Continue,
    Exit,
    Loop,
}

struct Guard {
    position: Index,
    direction: Direction,
}

impl Guard {
    fn new(position: Index) -> Self {
        Guard {
            position,
            direction: Direction::Up,
        }
    }

    fn turn_right(&mut self) {
        match self.direction {
            Direction::Up => self.direction = Direction::Right,
            Direction::Left => self.direction = Direction::Up,
            Direction::Right => self.direction = Direction::Down,
            Direction::Down => self.direction = Direction::Left,
        }
    }

    fn traverse(
        &mut self,
        map: &mut Matrix<MapCell>,
        states: Option<&mut HashSet<(Index, Direction)>>,
    ) -> TraverseResult {
        match self.direction {
            Direction::Down => {
                // Check exit bounds.
                if self.position.y + 1 >= map.size.y {
                    return TraverseResult::Exit;
                }

                // Move or rot.
                if map[self.position.y + 1][self.position.x].have_obstacle {
                    self.turn_right();
                } else {
                    self.position.y += 1;
                }
            }
            Direction::Left => {
                // Check exit bounds.
                if self.position.x as i32 - 1 < 0 {
                    return TraverseResult::Exit;
                }

                // Move or rot.
                if map[self.position.y][self.position.x - 1].have_obstacle {
                    self.turn_right();
                } else {
                    self.position.x -= 1;
                }
            }
            Direction::Right => {
                // Check exit bounds.
                if self.position.x + 1 >= map.size.x {
                    return TraverseResult::Exit;
                }

                // Move or rot.
                if map[self.position.y][self.position.x + 1].have_obstacle {
                    self.turn_right();
                } else {
                    self.position.x += 1;
                }
            }
            Direction::Up => {
                // Check exit bounds.
                if self.position.y as i32 - 1 < 0 {
                    return TraverseResult::Exit;
                }

                // Move or rot.
                if map[self.position.y - 1][self.position.x].have_obstacle {
                    self.turn_right();
                } else {
                    self.position.y -= 1;
                }
            }
        };

        // Mark movement on the map.
        map[self.position.y][self.position.x].have_guard_visited = true;

        //dbg!(&states);

        // Check and update state if needed.
        if let Some(st) = states {
            if st.contains(&(self.position, self.direction)) {
                return TraverseResult::Loop;
            } else {
                st.insert((self.position, self.direction));
            }
        };

        TraverseResult::Continue
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
