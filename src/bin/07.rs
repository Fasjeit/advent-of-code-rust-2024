advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u128> {
    let data = parse_input(input);
    let mut acc_answer = 0;
    for (target, numbers) in data {
        //dbg!(&target);
        //dbg!(&numbers);

        let acc_value = numbers[0];
        let number_index = 0;
        let result = dfs(&numbers, number_index, acc_value, target);
        if result {
            //dbg!(target);
            acc_answer += target;
        }
    }
    Some(acc_answer)
}

#[allow(unused_variables)]
pub fn part_two(input: &str) -> Option<u128> {
    None
}

pub enum Operation {
    Plus,
    Mul,
}

fn dfs(numbers: &Vec<u128>, number_index: usize, acc_value: u128, target_value: u128) -> bool {
    if number_index == numbers.len() - 1 {
        if acc_value == target_value {
            return true;
        } else {
            //dbg!(acc_value);
            return false;
        }
    }

    if acc_value > target_value {
        //dbg!(acc_value);
        return false;
    }

    if dfs(
        numbers,
        number_index + 1,
        acc_value + numbers[number_index + 1],
        target_value,
    ) {
        true
    } else {
        dfs(
            numbers,
            number_index + 1,
            acc_value * numbers[number_index + 1],
            target_value,
        )
    }
}

fn parse_input(input: &str) -> Vec<(u128, Vec<u128>)> {
    let mut result: Vec<(u128, Vec<u128>)> = Vec::new();

    let iterator = input.lines().map(|l| l.split(':'));
    for mut splitted_line in iterator {
        //dbg!(&splitted_line.next());
        //dbg!(&splitted_line.next());
        //todo!();
        let target = splitted_line
            .next()
            .expect("Cannot parse target")
            .parse::<u128>()
            .expect("Cannot parse target value");
        let values: Vec<u128> = splitted_line
            .next()
            .expect("Cannot parse values")
            .split_whitespace()
            .map(|v| v.parse::<u128>().expect("Cannot parse values as u128"))
            .collect();
        result.push((target, values));
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dfs_true() {
        let numbers = vec![81, 40, 27];
        let target = 3267;
        let acc_value = numbers[0];
        let number_index = 0;
        let result = dfs(&numbers, number_index, acc_value, target);
        assert!(result);
    }

    #[test]
    fn test_dfs_false() {
        let numbers = vec![17, 15];
        let target = 83;
        let acc_value = numbers[0];
        let number_index = 0;
        let result = dfs(&numbers, number_index, acc_value, target);
        assert!(!result);
    }

    //184299: 2 9 8 94 975 2 7 1 6 4 3 1
    #[test]
    fn test_dfs_true_dbg() {
        let numbers = vec![2, 9, 8, 94, 975, 2, 7, 1, 6, 4, 3, 1];
        let target = 184299;
        let acc_value = numbers[0];
        let number_index = 0;
        let result = dfs(&numbers, number_index, acc_value, target);
        assert!(result);
    }

    // 735: 5 147 1
    #[test]
    fn test_dfs_true_dbg_2() {
        let numbers = vec![5, 147, 1];
        let target = 735;
        let acc_value = numbers[0];
        let number_index = 0;
        let result = dfs(&numbers, number_index, acc_value, target);
        assert!(result);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
