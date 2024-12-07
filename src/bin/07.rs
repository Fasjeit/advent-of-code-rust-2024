advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
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

pub fn part_two(input: &str) -> Option<u64> {
    let data = parse_input(input);
    let mut acc_answer = 0;
    for (target, numbers) in data {
        //dbg!(&target);
        //dbg!(&numbers);

        let acc_value = numbers[0];
        let number_index = 0;
        let result = dfs_part_2(&numbers, number_index, acc_value, target);
        if result {
            //dbg!(target);
            acc_answer += target;
        }
    }
    Some(acc_answer)
}

fn dfs_part_2(numbers: &Vec<u64>, number_index: usize, acc_value: u64, target_value: u64) -> bool {
    // dfs from part 1 but with concatenation operation.
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

    dfs_part_2(
        numbers,
        number_index + 1,
        acc_value + numbers[number_index + 1],
        target_value,
    ) || dfs_part_2(
        numbers,
        number_index + 1,
        acc_value * numbers[number_index + 1],
        target_value,
    ) || dfs_part_2(
        numbers,
        number_index + 1,
        {
            // x || y = x*10*ord_10(y) + y
            let ord = base_10_len(numbers[number_index + 1]);
            // println!("DBG!!!");
            // dbg!(acc_value);
            // dbg!(numbers[number_index + 1]);
            // dbg!(ord);
            // dbg!(acc_value * ((10 as u64).pow(ord)) + numbers[number_index + 1]);
            // println!("END: DBG!!!");
            acc_value * (10_u64.pow(ord)) + numbers[number_index + 1]
        },
        target_value,
    )
}

fn base_10_len(n: u64) -> u32 {
    n.checked_ilog10().unwrap_or(0) + 1
}

fn dfs(numbers: &Vec<u64>, number_index: usize, acc_value: u64, target_value: u64) -> bool {
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

    dfs(
        numbers,
        number_index + 1,
        acc_value + numbers[number_index + 1],
        target_value,
    ) || dfs(
        numbers,
        number_index + 1,
        acc_value * numbers[number_index + 1],
        target_value,
    )
}

fn parse_input(input: &str) -> Vec<(u64, Vec<u64>)> {
    let mut result: Vec<(u64, Vec<u64>)> = Vec::new();

    let iterator = input.lines().map(|l| l.split(':'));
    for mut splitted_line in iterator {
        //dbg!(&splitted_line.next());
        //dbg!(&splitted_line.next());
        //todo!();
        let target = splitted_line
            .next()
            .expect("Cannot parse target")
            .parse::<u64>()
            .expect("Cannot parse target value");
        let values: Vec<u64> = splitted_line
            .next()
            .expect("Cannot parse values")
            .split_whitespace()
            .map(|v| v.parse::<u64>().expect("Cannot parse values as u64"))
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

    //156: 15 6
    #[test]
    fn test_dfs_part2_true_dbg() {
        let numbers = vec![15, 6];
        let target = 156;
        let acc_value = numbers[0];
        let number_index = 0;
        let result = dfs_part_2(&numbers, number_index, acc_value, target);
        assert!(result);
    }

    //7290: 6 8 6 15
    #[test]
    fn test_dfs_part2_true_dbg_2() {
        let numbers = vec![6, 8, 6, 15]; // 6 * 8 || 6 * 15
        let target = 7290;
        let acc_value = numbers[0];
        let number_index = 0;
        let result = dfs_part_2(&numbers, number_index, acc_value, target);
        assert!(result);
    }

    // 7290: 48 6 15
    #[test]
    fn test_dfs_part2_true_dbg_2_1() {
        let numbers = vec![48, 6, 15]; // 48 || 6 * 15
        let target = 7290;
        let acc_value = numbers[0];
        let number_index = 0;
        let result = dfs_part_2(&numbers, number_index, acc_value, target);
        assert!(result);
    }

    //192: 17 8 14
    #[test]
    fn test_dfs_part2_true_dbg_3() {
        let numbers = vec![17, 8, 14]; // 17 || 8 + 14
        let target = 192;
        let acc_value = numbers[0];
        let number_index = 0;
        let result = dfs_part_2(&numbers, number_index, acc_value, target);
        assert!(result);
    }

    // 23407953580: 4 9 2 476 260 683
    #[test]
    fn rest_dfs_part2_true_dbg_4() {
        let numbers = vec![4, 9, 2, 476, 260, 683];
        let target = 23407953580;
        let acc_value = numbers[0];
        let number_index = 0;
        let result = dfs_part_2(&numbers, number_index, acc_value, target);
        assert!(result);
    }

    //13504695: 2 4 7 523 47 2 1
    #[test]
    fn rest_dfs_part2_true_dbg_5() {
        let numbers = vec![2, 4, 7, 523, 47, 2, 1];
        let target = 13504695;
        let acc_value = numbers[0];
        let number_index = 0;
        let result = dfs_part_2(&numbers, number_index, acc_value, target);
        assert!(result);
    }

    //17114: 17 100 14
    #[test]
    fn test_dfs_part2_true_dbg_6() {
        let numbers = vec![17, 100, 14]; // 17 || 100 + 14
        let target = 17114;
        let acc_value = numbers[0];
        let number_index = 0;
        let result = dfs_part_2(&numbers, number_index, acc_value, target);
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
        assert_eq!(result, Some(11387));
    }
}
