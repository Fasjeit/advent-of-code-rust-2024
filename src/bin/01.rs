use std::collections::HashMap;
use std::fmt::Debug;
use std::str::FromStr;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let (mut first, mut second, size) = parse_2_column_input_same_size::<u32>(input);

    first.sort();
    second.sort();

    let mut delta: u32 = 0;
    for i in 0..size {
        delta += (first[i] as i32 - second[i] as i32).unsigned_abs()
    }

    Some(delta)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (first, second, size) = parse_2_column_input_same_size::<u32>(input);

    // Create hash_map for first list.
    let mut first_table = HashMap::with_capacity(size);
    first.into_iter().for_each(|i| {
        first_table.entry(i).and_modify(|e| *e += 1).or_insert(1);
    });

    // Compute the number of elements in second list, which are present in first_set.
    let mut second_table = HashMap::with_capacity(size);
    second.into_iter().for_each(|i| {
        if first_table.contains_key(&i) {
            second_table.entry(i).and_modify(|e| *e += 1).or_insert(1);
        }
    });

    let res = second_table.iter().fold(0, |acc, (key, value)| {
        acc + (key * value * first_table[key])
    });
    Some(res)
}

fn parse_2_column_input_same_size<T>(input: &str) -> (Vec<T>, Vec<T>, usize)
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    let splitted_lines: Vec<&str> = input.lines().collect();
    let size = splitted_lines.len();

    let mut first: Vec<T> = Vec::with_capacity(size);
    let mut second: Vec<T> = Vec::with_capacity(size);

    for line in splitted_lines {
        let mut splitted = line.split_whitespace();
        first.push(
            splitted
                .next()
                .expect("Non empty line with whitespace splitted 2 values are expected!")
                .parse()
                .expect("Expected T value"),
        );
        second.push(
            splitted
                .next()
                .expect("Non empty line with whitespace splitted 2 values are expected!")
                .parse()
                .expect("Expected T value"),
        );
    }

    (first, second, size)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
