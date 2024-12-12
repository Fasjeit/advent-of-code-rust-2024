use std::clone;
use std::collections::HashSet;
use std::fmt::Debug;
use std::process::id;
use std::str::FromStr;

advent_of_code::solution!(12);

pub fn part_one(input: &str) -> Option<u32> {
    let (data, size) = parse_row_input_as_data_array::<char>(input);
    let data_cells: Vec<MapCell> = data.into_iter().enumerate().map(|(i,t)|  MapCell::new(i, t)).collect();

    let mut matrix = Matrix{ data:data_cells.clone(), size };

    //matrix.print();

    // let test_i = 7;
    // let index = matrix.get_index_from_position(test_i);
    // let parent = test_i;
    // let mut acc_area = 0;
    // let acc_perimeter = traverse(&mut matrix, index, parent, &mut acc_area);

    // matrix.print();
    // dbg!(&acc_area);
    // dbg!(&acc_perimeter);


    let mut total_price = 0;
    for i in 0..matrix.data.len()
    {
        let index = matrix.get_index_from_position(i);
        let parent = i;

        let mut area = 0;
        let perimeter = traverse(&mut matrix, index, parent, &mut area);

        total_price += area * perimeter;
    }

    Some(total_price)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (data, size) = parse_row_input_as_data_array::<char>(input);
    let data_cells: Vec<MapCell> = data.into_iter().enumerate().map(|(i,t)|  MapCell::new(i, t)).collect();

    let mut matrix = Matrix{ data:data_cells.clone(), size };

    //matrix.print();

    // let test_i = 0;
    // let index = matrix.get_index_from_position(test_i);
    // let parent = test_i;
    // let mut acc_area = 0;
    // let acc_perimeter = traverse_part2(&mut matrix, index, parent, &mut acc_area, FromDirection::None);

    // matrix.print();
    // dbg!(&acc_area);
    // dbg!(&acc_perimeter);


    let mut total_price = 0;
    for i in 0..matrix.data.len()
    {
        let index = matrix.get_index_from_position(i);
        let parent = i;

        let mut area = 0;
        let perimeter = traverse_part2(&mut matrix, index, parent, &mut area, FromDirection::None);

        total_price += area * perimeter;
    }

    Some(total_price)

    //todo!()
}

fn traverse(matrix: &mut Matrix<MapCell>, index:Index, parent:usize, acc_area:&mut u32) -> u32
{
    // let parent = match parent {
    //     Some(p) => p,
    //     None => index,
    // };

    let current = &matrix[index.y][index.x].clone();
    if current.visited
    {
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
            parent,
            acc_area,
        );
        cell_perimeter -=1;
        cell_perimeter += rec_result;
    }
    // Down
    if index.y < matrix.size.y - 1 && matrix[index.y + 1][index.x].garden_type == current.garden_type {
        let rec_result = traverse(
            matrix,
            Index {
                x: index.x,
                y: index.y + 1,
            },
            parent,
            acc_area,
        );
        cell_perimeter -=1;
        cell_perimeter += rec_result;
    }
    // Right
    if index.x < matrix.size.x - 1 && matrix[index.y][index.x + 1].garden_type == current.garden_type {
        let rec_result = traverse(
            matrix,
            Index {
                x: index.x + 1,
                y: index.y,
            },
            parent,
            acc_area,
        );
        cell_perimeter -=1;
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
            parent,
            acc_area,
        );
        cell_perimeter -=1;
        cell_perimeter += rec_result;
    }

    cell_perimeter
}

#[derive(PartialEq, Copy, Clone)]
enum FromDirection {
    Up,
    Down,
    Left,
    Right,
    None
}

fn should_not_exists_check(matrix :&Matrix<MapCell>, index: Index, garden_type: char, directions: (FromDirection, FromDirection)) -> bool
{
    // sample for UP

    // +-+ +-+
    //  T| |K
    // +-+-+-+
    //  X A Y
    // +-+-+-+
    // upper A bound non-exists for empty UP
    // IF
    // Y exists and K non-exists
    // AND
    // X exists and T non-exists

    // need to add have bounds_created check to make initial bounds exists

    let mut should_not_exists = false;
    if let Some(ri) = index.to_direction(matrix, directions.0)
    {
        if matrix[ri.y][ri.x].garden_type == garden_type && matrix[ri.y][ri.x].bounds_created
        {
            if let Some(rui) = ri.to_direction(matrix, directions.1)
            {
                if matrix[rui.y][rui.x].garden_type != garden_type
                {
                    should_not_exists = true;
                }
            }
            else {
                should_not_exists = true;
            }
        }
    }
    should_not_exists
}

fn traverse_part2(matrix: &mut Matrix<MapCell>, index:Index, parent:usize, acc_area:&mut u32, from_direction: FromDirection) -> u32
{
    // let parent = match parent {
    //     Some(p) => p,
    //     None => index,
    // };

    let current = &matrix[index.y][index.x].clone();
    if current.visited
    {
        return 0;
    }

    dbg!(&index);

    // perimeter for the current cell + all next traversed cells from the current one.
    // max for cell only is 4. We will subtract 1 for each neighbour cell.
    let mut cell_perimeter = 4;

    *acc_area += 1;

    matrix[index.y][index.x].visited = true;

    let mut has_up = true;
    let mut has_down = true;
    let mut has_left = true;
    let mut has_right = true;

    // Up
    if index.y > 0 && matrix[index.y - 1][index.x].garden_type == current.garden_type {
        let rec_result = traverse_part2(
            matrix,
            Index {
                x: index.x,
                y: index.y - 1,
            },
            parent,
            acc_area,
            FromDirection::Up
        );
        cell_perimeter -=1;
        cell_perimeter += rec_result;

        println!("{:#?} has up", &index);
    }
    else {
        has_up = false;
    }

    // Down
    if index.y < matrix.size.y - 1 && matrix[index.y + 1][index.x].garden_type == current.garden_type {
        let rec_result = traverse_part2(
            matrix,
            Index {
                x: index.x,
                y: index.y + 1,
            },
            parent,
            acc_area,
            FromDirection::Down
        );
        cell_perimeter -=1;
        cell_perimeter += rec_result;

        println!("{:#?} has down", &index);
    }
    else {
        has_down = false;
    }

    // Right
    if index.x < matrix.size.x - 1 && matrix[index.y][index.x + 1].garden_type == current.garden_type {
        let rec_result = traverse_part2(
            matrix,
            Index {
                x: index.x + 1,
                y: index.y,
            },
            parent,
            acc_area,
            FromDirection::Right
        );
        cell_perimeter -=1;
        cell_perimeter += rec_result;

        println!("{:#?} has right", &index);
    }
    else
    {
        has_right = false;
    }

    // Left
    if index.x > 0 && matrix[index.y][index.x - 1].garden_type == current.garden_type {
        let rec_result = traverse_part2(
            matrix,
            Index {
                x: index.x - 1,
                y: index.y,
            },
            parent,
            acc_area,
            FromDirection::Left
        );
        cell_perimeter -=1;
        cell_perimeter += rec_result;

        println!("{:#?} has left", &index);
    }
    else
    {
        has_left = false;
    }


    // All checks to remove bounds only after recursion calls
    // to use filled bounds_created for all neighbors.

    if !has_up
    {
        if should_not_exists_check(&matrix, index, current.garden_type, (FromDirection::Right, FromDirection::Up))
         || should_not_exists_check(&matrix, index, current.garden_type, (FromDirection::Left, FromDirection::Up))
        {
            // continue horizontal side
            // exists left and upper-left
            // or right and upper-right

            cell_perimeter -=1;
            println!("{:#?} continue horizontal up", &index);
        }
    }

    if !has_down
    {
        if should_not_exists_check(&matrix, index, current.garden_type, (FromDirection::Right, FromDirection::Down))
         || should_not_exists_check(&matrix, index, current.garden_type, (FromDirection::Left, FromDirection::Down))
        {
                // continue horizontal side
                cell_perimeter -=1;
                println!("{:#?} continue horizontal down", &index);
            }
    }

    if !has_right
    {
        if should_not_exists_check(&matrix, index, current.garden_type, (FromDirection::Up, FromDirection::Right))
         || should_not_exists_check(&matrix, index, current.garden_type, (FromDirection::Down, FromDirection::Right))
    {
        // continue vertical side
        cell_perimeter -=1;
        println!("{:#?} continue vertical right", &index);
    }
    }

    if !has_left
    {
        if should_not_exists_check(&matrix, index, current.garden_type, (FromDirection::Up, FromDirection::Left))
         || should_not_exists_check(&matrix, index, current.garden_type, (FromDirection::Down, FromDirection::Left))
    {
        // continue vertical side
        cell_perimeter -=1;
        println!("{:#?} continue vertical left", &index);
    }
    }

    println!("{:#?} edges:  {}", &index, cell_perimeter);

    matrix[index.y][index.x].bounds_created = true;
    cell_perimeter
}

#[derive(Debug, Clone)]
struct MapCell {
    garden_type: char,
    visited: bool,
    bounds_created: bool,
}

impl MapCell {
    fn new(id: usize, garden_type: char) -> Self {
        MapCell {
            garden_type,
            visited: false,
            bounds_created: false
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

impl Index{
    fn up<T>(&self, _matrix: &Matrix<T>) -> Option<Index>
    {
        if self.y == 0
        {
            return None;
        }
        return Some(Index { x:self.x, y:self.y-1});
    }

    fn left<T>(&self, _matrix: &Matrix<T>) -> Option<Index>
    {
        if self.x == 0
        {
            return None;
        }
        return Some(Index { x:self.x-1, y:self.y});
    }

    fn down<T>(&self, matrix: &Matrix<T>) -> Option<Index>
    {
        if self.y == matrix.size.y-1
        {
            return None;
        }
        return Some(Index { x:self.x, y:self.y+1});
    }

    fn right<T>(&self, matrix: &Matrix<T>) -> Option<Index>
    {
        if self.x == matrix.size.x-1
        {
            return None;
        }
        return Some(Index { x:self.x+1, y:self.y});
    }

    fn to_direction<T>(&self, matrix: &Matrix<T>, direction: FromDirection) -> Option<Index>
    {
        match direction {
            FromDirection::Up => self.up(matrix),
            FromDirection::Down =>self.down(matrix),
            FromDirection::Left => self.left(matrix),
            FromDirection::Right => self.right(matrix),
            FromDirection::None => None,
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
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY,1));
        assert_eq!(result, Some(140));
    }

    #[test]
    fn test_part_one_2() {
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY,2));
        assert_eq!(result, Some(772));
    }

    #[test]
    fn test_part_one_3() {
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY,3));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two_1() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY,1));
        assert_eq!(result, Some(80));
    }

    #[test]
    fn test_part_two_2() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY,2));
        assert_eq!(result, Some(436));
    }

    #[test]
    fn test_part_two_3() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY,3));
        assert_eq!(result, Some(1206));
    }

    #[test]
    fn test_part_two_4() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY,4));
        assert_eq!(result, Some(236));
    }

    #[test]
    fn test_part_two_5() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY,5));
        assert_eq!(result, Some(368));
    }
}
