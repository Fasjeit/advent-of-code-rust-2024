use std::fmt::Debug;
use std::str::FromStr;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let (data, _) = parse_row_input::<u32>(input);
    let result = data.into_iter().fold(0, |acc, row| {
        acc + if check_row_stable(&row, false) { 1 } else { 0 }
    });

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (data, _) = parse_row_input::<u32>(input);
    let result = data.into_iter().fold(0, |acc, row| {
        acc + if check_row_stable(&row, true) { 1 } else { 0 }
    });

    Some(result)
}

fn check_row_stable(row: &[u32], fault_tolerant: bool) -> bool {
    // 1. Row values are either increasing or decreasing.
    // 2. Delta (d) between nearest values 1 <= d <= 3

    // d values should all be negative or positive.

    // If fault tolerant check - remove either current, previous,
    // or next element and check again without fault tolerance.

    let mut last_value = row[0] as i32;
    let d = row[1] as i32 - last_value;
    if d == 0 || d.abs() > 3 {
        if !fault_tolerant {
            return false;
        } else {
            // Remove the first or the second element.
            return ({
                let new_row = &remove_element(row, 0);
                check_row_stable(new_row, false)
            } || {
                let new_row = &remove_element(row, 1);
                check_row_stable(new_row, false)
            });
        }
    }

    let initial_d_sign = d.signum();
    last_value = row[1] as i32;

    for index in 2..row.len() {
        let v = &row[index];
        let d = *v as i32 - last_value;
        last_value = *v as i32;

        if d == 0 || d.abs() > 3 {
            if !fault_tolerant {
                return false;
            } else {
                // Remove the current or previous element.
                return ({
                    let new_row = &remove_element(row, index);
                    check_row_stable(new_row, false)
                } || {
                    let new_row = &remove_element(row, index - 1);
                    check_row_stable(new_row, false)
                });
            }
        }

        if d.signum() != initial_d_sign {
            if !fault_tolerant {
                return false;
            } else {
                // Remove the current (i) or previous (i-1) element or even (i-2) element.
                return ({
                    let new_row = &remove_element(row, index);
                    check_row_stable(new_row, false)
                } || {
                    let new_row = &remove_element(row, index - 1);
                    check_row_stable(new_row, false)
                } || {
                    let new_row = &remove_element(row, index - 2);
                    check_row_stable(new_row, false)
                });
            }
        }
    }
    true
}

fn remove_element<T>(input: &[T], index: usize) -> Vec<T>
where
    T: Clone,
{
    if index >= input.len() {
        return input.to_vec();
    }

    let left = &input[..index];
    let right = &input[index + 1..];

    [left, right].concat()
}

fn parse_row_input<T>(input: &str) -> (Vec<Vec<T>>, usize)
where
    T: FromStr + std::fmt::Debug,
    <T as FromStr>::Err: Debug,
{
    let splitted_lines: Vec<&str> = input.lines().collect();
    let size = splitted_lines.len();

    let mut result: Vec<Vec<T>> = Vec::with_capacity(size);

    for line in splitted_lines {
        let splitted = line.split_whitespace();
        result.push(
            splitted
                .map(|s| s.parse().expect("T values expected"))
                .collect(),
        );
    }

    (result, size)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_remove_element() {
        let data = [1, 2, 3, 4, 5];
        let trimmed = remove_element(&data, 2);
        assert_eq!(trimmed, [1, 2, 4, 5])
    }

    #[test]
    fn test_remove_element_first() {
        let data = [1, 2, 3, 4, 5];
        let trimmed = remove_element(&data, 0);
        assert_eq!(trimmed, [2, 3, 4, 5])
    }

    #[test]
    fn test_remove_element_last() {
        let data = [1, 2, 3, 4, 5];
        let trimmed = remove_element(&data, 4);
        assert_eq!(trimmed, [1, 2, 3, 4,])
    }
}
