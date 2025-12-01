use std::ops::{Div, Mul, Sub};

advent_of_code::solution!(13);

#[allow(non_snake_case)]
pub fn part_one(input: &str) -> Option<u64> {
    let inputs = parse_input(input);

    let mut acc_result = 0;

    // let (A, B, T) = inputs[3];
    // acc_result += solve_part_1_task(A, B, T);
    // dbg!(&acc_result);

    for input in inputs {
        let (A, B, T) = input;

        // part1 takes too long.
        // use part 2 code instead.
        // acc_result += solve_part_1_task(A, B, T);
        acc_result += solve_part_2_task(A, B, T);
        //dbg!(&acc_result);
    }

    Some(acc_result)
}

#[allow(non_snake_case)]
pub fn part_two(input: &str) -> Option<u64> {
    let inputs = parse_input(input);

    let mut acc_result = 0;

    for input in inputs {
        let (A, B, T) = input;
        acc_result += solve_part_2_task(
            A,
            B,
            Index {
                x: T.x + 10000000000000,
                y: T.y + 10000000000000,
            },
        );
        //dbg!(&acc_result);
    }

    Some(acc_result)
}

// Working part 1, but slow.
// We will use part2 code for both parts instead.

// #[allow(non_snake_case)]
// fn solve_part_1_task(A: Index, B: Index, T: Index) -> u64 {
//     const MAX_ITERATION: u64 = 100;

//     let mut solutions: Vec<(u64, u64)> = Vec::with_capacity(2);

//     for x in 1..MAX_ITERATION {
//         if x * A.x > T.x || x * A.y > T.y {
//             break;
//         }

//         let up = T - (x * A);
//         // dbg!(x);
//         // dbg!(&T);
//         // dbg!(&(x * A));
//         // dbg!(&up);
//         // dbg!(&B);

//         if matches!(up, Some(u) if u.divisible_by(B)) {
//             let y = up.unwrap() / B;
//             //dbg!((x, y));
//             solutions.push((x, y));
//         }
//     }

//     if solutions.len() == 0 {
//         return 0;
//     }

//     if solutions.len() == 1 {
//         return 3 * solutions[0].0 + solutions[0].1;
//     }

//     let mut min_value = u64::max_value();
//     for solution in solutions {
//         let value = 3 * solution.0 + solution.1;
//         if value < min_value {
//             min_value = value
//         }
//     }
//     min_value
// }
//
// impl Index {
//     fn divisible_by(&self, other: Index) -> bool {
//         self.x % other.x == 0 && self.y % other.y == 0 && self.x / other.x == self.y / other.y
//     }
// }

#[allow(non_snake_case)]
fn solve_part_2_task(A: Index, B: Index, T: Index) -> u64 {
    // xA + yB = T
    // is a system of linear equations
    // xA.0 + yB.0 = T.0
    // xA.1 + yB.1 = T.1
    //
    // y = (T.x - xA.0) / B.0
    // x = (B.1 * T.0 - B.0 * T.1) /
    //     (A.0 * B.1 - A.1 * B.0)

    let x_u = B.y as i64 * T.x as i64 - B.x as i64 * T.y as i64;
    let x_b = A.x as i64 * B.y as i64 - A.y as i64 * B.x as i64;

    if x_u % x_b != 0 {
        return 0;
    }

    let x = (x_u / x_b) as u64;

    let y_u = T.x - A.x * x;
    let y_b = B.x;

    if !y_u.is_multiple_of(y_b) {
        return 0;
    }

    let y = y_u / y_b;

    3 * x + y
}

#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
struct Index {
    x: u64,
    y: u64,
}

impl Sub for Index {
    type Output = Option<Self>;

    fn sub(self, rhs: Index) -> Self::Output {
        if rhs.x > self.x || rhs.y > self.y {
            return None;
        }

        Some(Index {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        })
    }
}

impl Div for Index {
    type Output = u64;

    fn div(self, rhs: Index) -> u64 {
        self.x / rhs.x
    }
}

impl Mul<Index> for u64 {
    type Output = Index;

    fn mul(self, rhs: Index) -> Self::Output {
        Index {
            x: self * rhs.x,
            y: self * rhs.y,
        }
    }
}

fn parse_input(input: &str) -> Vec<(Index, Index, Index)> {
    let mut result = Vec::new();
    let mut lines = input.lines();

    while let (Some(a_line), Some(b_line), Some(prize_line)) =
        (lines.next(), lines.next(), lines.next())
    {
        let a = parse_index_ab(a_line);
        let b = parse_index_ab(b_line);
        let prize = parse_index_t(prize_line);

        result.push((a, b, prize));

        // Skip the blank line
        lines.next();
    }

    result
}

fn parse_index_ab(line: &str) -> Index {
    let parts: Vec<&str> = line.split_whitespace().collect();
    let x = parts[2][2..parts[2].len() - 1]
        .parse::<u64>()
        .expect("u32 values expected"); // Extract X+.. and parse
    let y = parts[3][2..].parse::<u64>().expect("u32 values expected"); // Extract Y+.. and parse
    Index { x, y }
}

fn parse_index_t(line: &str) -> Index {
    let parts: Vec<&str> = line.split_whitespace().collect();
    let x = parts[1][2..parts[1].len() - 1]
        .parse::<u64>()
        .expect("u32 values expected"); // Extract X+.. and parse
    let y = parts[2][2..].parse::<u64>().expect("u32 values expected"); // Extract Y+.. and parse
    Index { x, y }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(875318608908));
    }
}
