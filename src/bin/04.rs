use std::fmt::Debug;
use std::str::FromStr;

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let (data, size) = parse_row_input_as_data_array::<char>(input);
    let matrix = Matrix { size, data };

    let mut acc = 0;
    
    for y in 0..size.y {
        for x in 0..size.x {
            //print!("{}", matrix[y][x]);
            acc += check_xmas_for_matrix_element(&matrix, Index { y, x });
        }
        //println!();
    }

    Some(acc)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (data, size) = parse_row_input_as_data_array::<char>(input);
    let matrix = Matrix { size, data };

    let mut acc = 0;

    for y in 0..size.y {
        for x in 0..size.x {
            //print!("{}", matrix[y][x]);
            acc += check_x_mas_for_matrix_element(&matrix, Index { y, x });
        }
        //println!();
    }

    Some(acc)
}

fn check_xmas_for_matrix_element(matrix: &Matrix<char>, index: Index) -> u32 {
    if matrix[index.y][index.x] != 'X' {
        return 0;
    }
    let mut res = 0;

    // column down
    if index.y + 3 < matrix.size.y
        && matrix[index.y + 1][index.x] == 'M'
        && matrix[index.y + 2][index.x] == 'A'
        && matrix[index.y + 3][index.x] == 'S'
    {
        res += 1;
    }

    // column up
    if index.y as i32 - 3 >= 0
        && matrix[index.y - 1][index.x] == 'M'
        && matrix[index.y - 2][index.x] == 'A'
        && matrix[index.y - 3][index.x] == 'S'
    {
        res += 1;
    }

    // row right
    if index.x + 3 < matrix.size.y
        && matrix[index.y][index.x + 1] == 'M'
        && matrix[index.y][index.x + 2] == 'A'
        && matrix[index.y][index.x + 3] == 'S'
    {
        res += 1;
    }

    // row left
    if index.x as i32 - 3 >= 0
        && matrix[index.y][index.x - 1] == 'M'
        && matrix[index.y][index.x - 2] == 'A'
        && matrix[index.y][index.x - 3] == 'S'
    {
        res += 1;
    }

    // diag down right
    if index.x + 3 < matrix.size.x
        && index.y + 3 < matrix.size.y
        && matrix[index.y + 1][index.x + 1] == 'M'
        && matrix[index.y + 2][index.x + 2] == 'A'
        && matrix[index.y + 3][index.x + 3] == 'S'
    {
        res += 1;
    }

    // diag up left
    if index.x as i32 - 3 >= 0
        && index.y as i32 - 3 >= 0
        && matrix[index.y - 1][index.x - 1] == 'M'
        && matrix[index.y - 2][index.x - 2] == 'A'
        && matrix[index.y - 3][index.x - 3] == 'S'
    {
        res += 1;
    }

    // diag up right
    if index.x as i32 - 3 >= 0
        && index.y + 3 < matrix.size.y
        && matrix[index.y + 1][index.x - 1] == 'M'
        && matrix[index.y + 2][index.x - 2] == 'A'
        && matrix[index.y + 3][index.x - 3] == 'S'
    {
        res += 1;
    }

    // diag down left
    if index.x + 3 < matrix.size.x
        && index.y as i32 - 3 >= 0
        && matrix[index.y - 1][index.x + 1] == 'M'
        && matrix[index.y - 2][index.x + 2] == 'A'
        && matrix[index.y - 3][index.x + 3] == 'S'
    {
        res += 1;
    }

    res
}

fn check_x_mas_for_matrix_element(matrix: &Matrix<char>, index: Index) -> u32 {
    if matrix[index.y][index.x] != 'A' {
        return 0;
    }

    // Searching for 4 patterns of
    // M.S
    // .A.
    // M.S
    //
    // for center char 'A'

    if index.x + 1 >= matrix.size.x
        || index.y + 1 >= matrix.size.y
        || index.x as i32 - 1 < 0
        || index.y as i32 - 1 < 0
    {
        return 0;
    }

    let mut found_slash_diag = false; // '/'
    let mut found_backslash_diag = false; // '\'

    // diag down right
    if matrix[index.y - 1][index.x - 1] == 'M' && matrix[index.y + 1][index.x + 1] == 'S' {
        found_backslash_diag = true;
    }

    // diag up left
    if matrix[index.y + 1][index.x + 1] == 'M' && matrix[index.y - 1][index.x - 1] == 'S' {
        found_backslash_diag = true;
    }

    // diag up right
    if matrix[index.y + 1][index.x - 1] == 'S' && matrix[index.y - 1][index.x + 1] == 'M' {
        found_slash_diag = true;
    }

    // diag down left
    if matrix[index.y - 1][index.x + 1] == 'S' && matrix[index.y + 1][index.x - 1] == 'M' {
        found_slash_diag = true;
    }

    if found_slash_diag && found_backslash_diag {
        1
    } else {
        0
    }
}

#[derive(Debug, Copy, Clone)]
struct Size {
    x: usize,
    y: usize,
}

#[derive(Debug, Copy, Clone)]
struct Index {
    x: usize,
    y: usize,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(9));
    }
}
