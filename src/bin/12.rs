use std::fmt::Debug;
use std::str::FromStr;

advent_of_code::solution!(12);

pub fn part_one(input: &str) -> Option<u32> {
    let (data, size) = parse_row_input_as_data_array::<char>(input);
    let data_cells: Vec<MapCell> = data
        .into_iter()
        .enumerate()
        .map(|(i, t)| MapCell::new(i, t))
        .collect();

    let mut matrix = Matrix {
        data: data_cells.clone(),
        size,
    };

    //matrix.print();

    // let test_i = 7;
    // let index = matrix.get_index_from_position(test_i);
    // let mut acc_area = 0;
    // let acc_perimeter = traverse(&mut matrix, index, &mut acc_area);

    // matrix.print();
    // dbg!(&acc_area);
    // dbg!(&acc_perimeter);

    let mut total_price = 0;
    for i in 0..matrix.data.len() {
        let index = matrix.get_index_from_position(i);

        let mut area = 0;
        let perimeter = traverse(&mut matrix, index, &mut area);

        total_price += area * perimeter;
    }

    Some(total_price)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (data, size) = parse_row_input_as_data_array::<char>(input);
    let data_cells: Vec<MapCell> = data
        .into_iter()
        .enumerate()
        .map(|(i, t)| MapCell::new(i, t))
        .collect();

    let mut matrix = Matrix {
        data: data_cells.clone(),
        size,
    };

    // matrix.print();

    // let test_i = 15;
    // let index = matrix.get_index_from_position(test_i);
    // let mut acc_area = 0;
    // let acc_sides = traverse_part2(&mut matrix, index, &mut acc_area);

    // matrix.print();
    // dbg!(&acc_area);
    // dbg!(&acc_sides);

    // todo!()

    let mut total_price = 0;
    for i in 0..matrix.data.len() {
        let index = matrix.get_index_from_position(i);

        let mut area = 0;
        let perimeter = traverse_part2(&mut matrix, index, &mut area);

        total_price += area * perimeter;
    }

    Some(total_price)
}

fn traverse(matrix: &mut Matrix<MapCell>, index: Index, acc_area: &mut u32) -> u32 {
    // let parent = match parent {
    //     Some(p) => p,
    //     None => index,
    // };

    let current = &matrix[index.y][index.x].clone();
    if current.visited {
        return 0;
    }

    // perimeter for the current cell + all next traversed cells from the current one.
    // max for cell only is 4. We will subtract 1 for each neighbour cell.
    let mut cell_perimeter = 4;

    *acc_area += 1;

    matrix[index.y][index.x].visited = true;

    // Up
    if index.y > 0 && matrix[index.y - 1][index.x].garden_type == current.garden_type {
        let rec_result = traverse(
            matrix,
            Index {
                x: index.x,
                y: index.y - 1,
            },
            acc_area,
        );
        cell_perimeter -= 1;
        cell_perimeter += rec_result;
    }
    // Down
    if index.y < matrix.size.y - 1
        && matrix[index.y + 1][index.x].garden_type == current.garden_type
    {
        let rec_result = traverse(
            matrix,
            Index {
                x: index.x,
                y: index.y + 1,
            },
            acc_area,
        );
        cell_perimeter -= 1;
        cell_perimeter += rec_result;
    }
    // Right
    if index.x < matrix.size.x - 1
        && matrix[index.y][index.x + 1].garden_type == current.garden_type
    {
        let rec_result = traverse(
            matrix,
            Index {
                x: index.x + 1,
                y: index.y,
            },
            acc_area,
        );
        cell_perimeter -= 1;
        cell_perimeter += rec_result;
    }
    // Left
    if index.x > 0 && matrix[index.y][index.x - 1].garden_type == current.garden_type {
        let rec_result = traverse(
            matrix,
            Index {
                x: index.x - 1,
                y: index.y,
            },
            acc_area,
        );
        cell_perimeter -= 1;
        cell_perimeter += rec_result;
    }

    cell_perimeter
}

#[derive(PartialEq, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn have_corner(
    matrix: &Matrix<MapCell>,
    index: Index,
    garden_type: char,
    directions: (Direction, Direction),
) -> u32 {
    // https://www.reddit.com/r/adventofcode/comments/1hcdnk0/comment/m1nkmol
    // Help from reddit - the number of sides = number of corners.
    // For each plot, check the neighbors in each pair of orthoganal
    // directions. If neither match the source plot
    //
    // Outer corner:
    // ...
    // ##.
    // ##.
    //
    // If both match the source plot and the corner plot doesn't match
    // Inner corner:
    //
    // ##.
    // ###
    // ###

    // Outer - up and right are not in same set
    // inner - up and right are in the same set, but upper-right - don't.

    // core is written with var names as up-right param.
    // but will work for other directions too.

    let up = index.navigate_to(matrix, directions.0);
    let right = index.navigate_to(matrix, directions.1);

    let outer_corner = match (up, right) {
        (None, None) => true, // Edge of the map.
        (Some(u), Some(r)) // both exists
            if matrix[u.y][u.x].garden_type != garden_type
                && matrix[r.y][r.x].garden_type != garden_type =>
        {
            true // but not the type
        }
        (Some(u), None) if matrix[u.y][u.x].garden_type != garden_type => true, // only one exits, the other - not the same type
        (None, Some(r)) if matrix[r.y][r.x].garden_type != garden_type => true, // only one exits, the other - not the same type
        _ => false, // Any other case
    };

    if outer_corner {
        return 1;
    }

    // Check inner corner
    if matches!(up, Some(u) if matrix[u.y][u.x].garden_type == garden_type)
        && matches!(right, Some(r) if matrix[r.y][r.x].garden_type == garden_type)
    {
        // As both up and right exists -> upper-right exists too!
        let upper_right = up.unwrap().navigate_to(matrix, directions.1).unwrap();
        if matrix[upper_right.y][upper_right.x].garden_type != garden_type {
            // Both have the same garden type and diagonal one have the other.
            // -> inner corner
            return 1;
        }
    }

    0
}

fn traverse_part2(matrix: &mut Matrix<MapCell>, index: Index, acc_area: &mut u32) -> u32 {
    // Same as part 1, but also counting corners
    // not perimeter.

    let current = &matrix[index.y][index.x].clone();
    if current.visited {
        return 0;
    }

    let mut corners = 0;

    *acc_area += 1;

    matrix[index.y][index.x].visited = true;

    // Up
    if index.y > 0 && matrix[index.y - 1][index.x].garden_type == current.garden_type {
        let rec_result = traverse_part2(
            matrix,
            Index {
                x: index.x,
                y: index.y - 1,
            },
            acc_area,
        );
        corners += rec_result;
    }
    // Down
    if index.y < matrix.size.y - 1
        && matrix[index.y + 1][index.x].garden_type == current.garden_type
    {
        let rec_result = traverse_part2(
            matrix,
            Index {
                x: index.x,
                y: index.y + 1,
            },
            acc_area,
        );
        corners += rec_result;
    }
    // Right
    if index.x < matrix.size.x - 1
        && matrix[index.y][index.x + 1].garden_type == current.garden_type
    {
        let rec_result = traverse_part2(
            matrix,
            Index {
                x: index.x + 1,
                y: index.y,
            },
            acc_area,
        );
        corners += rec_result;
    }
    // Left
    if index.x > 0 && matrix[index.y][index.x - 1].garden_type == current.garden_type {
        let rec_result = traverse_part2(
            matrix,
            Index {
                x: index.x - 1,
                y: index.y,
            },
            acc_area,
        );
        corners += rec_result;
    }

    // Now counting corners
    corners += have_corner(
        matrix,
        index,
        current.garden_type,
        (Direction::Up, Direction::Right),
    );

    corners += have_corner(
        matrix,
        index,
        current.garden_type,
        (Direction::Up, Direction::Left),
    );

    corners += have_corner(
        matrix,
        index,
        current.garden_type,
        (Direction::Down, Direction::Right),
    );

    corners += have_corner(
        matrix,
        index,
        current.garden_type,
        (Direction::Down, Direction::Left),
    );

    corners
}

#[derive(Debug, Clone)]
struct MapCell {
    garden_type: char,
    visited: bool,
}

impl MapCell {
    fn new(_id: usize, garden_type: char) -> Self {
        MapCell {
            garden_type,
            visited: false,
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

    fn navigate_to<T>(&self, matrix: &Matrix<T>, direction: Direction) -> Option<Index> {
        match direction {
            Direction::Up => self.up(matrix),
            Direction::Down => self.down(matrix),
            Direction::Left => self.left(matrix),
            Direction::Right => self.right(matrix),
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
            for x in 0..self.size.y {
                let mut ch = self[y][x].garden_type.to_string();
                if self[y][x].visited {
                    ch = ch.to_ascii_lowercase();
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
        assert_eq!(result, Some(140));
    }

    #[test]
    fn test_part_one_2() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(772));
    }

    #[test]
    fn test_part_one_3() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two_1() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(80));
    }

    #[test]
    fn test_part_two_2() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(436));
    }

    #[test]
    fn test_part_two_3() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(1206));
    }

    #[test]
    fn test_part_two_4() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 4,
        ));
        assert_eq!(result, Some(236));
    }

    #[test]
    fn test_part_two_5() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 5,
        ));
        assert_eq!(result, Some(368));
    }
}
