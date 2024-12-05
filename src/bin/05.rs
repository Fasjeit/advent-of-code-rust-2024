use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::str::FromStr;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    solve(input, &Day5::Part1)
}

pub fn part_two(input: &str) -> Option<u32> {
    solve(input, &Day5::Part2)
}

enum Day5 {
    Part1,
    Part2
}

fn solve(input :&str, part: &Day5) -> Option<u32>
{
    let mut iterator = input.split("\n\n");
    let graph_data = iterator.next().unwrap();
    let sort_data = iterator.next().unwrap();

    let (parsed_graph_data, _size) = parse_2_tuple_input_with_delimiter::<u32>(graph_data, '|');
    let (lines, _size) = parse_row_input::<u32>(sort_data, ',');

    let mut rules:HashMap<u32, Vec<u32>> = HashMap::new();
    parsed_graph_data.into_iter().for_each(|d| {
        rules.entry(d.0)
             .and_modify(|v| v.push(d.1))
             .or_insert(vec![d.1]);});

    let mut acc = 0;
    for line in lines
    {
        let current_line = &line;

        // Pre building graph not working due to cycles in the initial graph
        // From reddit -
        // The fact that the looping cases never appear in
        // the same test case means it works as long as you
        // only dfs over numbers that are in the same test
        // case.
        //
        // `node_set` is a set of all nodes for current task.
        // Only nods from `node_set` used for dfs path finding.

        let mut node_set:HashSet<u32> = HashSet::new();
        current_line.iter().for_each(|e| {node_set.insert(*e);});

        let mut visited: HashSet<u32> = HashSet::new();
        let mut top_sort: Vec<u32> = Vec::new();

        //dbg!(&node_set);

        // first node we use to sort may not be the smallest one
        // so we add all other nodes to dfs, which was not met before.

        dfs(current_line[0], &node_set, &mut visited, &mut top_sort, &rules);
        while node_set.len() != visited.len(){
            let node = node_set.iter().find(|n| !visited.contains(n)).unwrap();
            dfs(*node, &node_set, &mut visited, &mut top_sort, &rules);
        }

        //dbg!(current_line);
        //dbg!(&top_sort);

        let same = compare_inv(current_line, &top_sort);
        match (part, same) {
            (Day5::Part1, true)=>
            {
                let middle_index = &line.len()/2;
                acc += line[middle_index];
            }
            (Day5::Part2, false)=>
            {
                let middle_index = &top_sort.len()/2;
                acc += top_sort[middle_index];
                //dbg!(current_line.len());
                //dbg!(acc);
                //dbg!(top_sort.len()); }
            },
            _ => ()
        }
    }

    Some(acc)
}

fn compare_inv(first:&[u32], second:&[u32]) -> bool{
    if first.len() != second.len()
    {
        return false;
    }
    for i in 0..first.len() {
        if first[i] != second[second.len()-i-1]
        {
            return false;
        }
    }
    true
}

fn dfs(
    node: u32,
    node_set :&HashSet<u32>, // need to filter paths only to the present nods for a row to avoid cycles
    visited: &mut HashSet<u32>,
    top_sort: &mut Vec<u32>,
    rules: &HashMap<u32, Vec<u32>>,
) {
    if visited.contains(&node) {
        return;
    }
    visited.insert(node);
    for dst in rules.get(&node).unwrap_or(&vec![]) {
        if !node_set.contains(dst) {
            continue;
        }
        dfs(*dst,node_set, visited, top_sort, rules);
    }
    top_sort.push(node);
}

fn parse_row_input<T>(input: &str, delimiter: char) -> (Vec<Vec<T>>, usize)
where
    T: FromStr + std::fmt::Debug,
    <T as FromStr>::Err: Debug,
{
    let splitted_lines: Vec<&str> = input.lines().collect();
    let size = splitted_lines.len();

    let mut result: Vec<Vec<T>> = Vec::with_capacity(size);

    for line in splitted_lines {
        let splitted = line.split(delimiter);
        result.push(
            splitted
                .map(|s| s.parse().expect("T values expected"))
                .collect(),
        );
    }

    (result, size)
}

fn parse_2_tuple_input_with_delimiter<T>(input: &str, delimiter: char) -> (Vec<(T,T)>, usize)
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    let splitted_lines: Vec<&str> = input.lines().collect();
    let size = splitted_lines.len();

    let mut result: Vec<(T,T)> = Vec::with_capacity(size);

    for line in splitted_lines {
        let mut splitted = line.split(delimiter);
        result.push((
            splitted
                .next()
                .expect("Non empty line with delimiter splitted 2 values are expected!")
                .parse()
                .expect("Expected T value"),
            splitted
                .next()
                .expect("Non empty line with delimiter splitted 2 values are expected!")
                .parse()
                .expect("Expected T value"),
        ));
    }

    (result, size)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_top_sort(){
        let node = 47;

        let mut node_set:HashSet<u32> = HashSet::new();
        node_set.insert(47);
        node_set.insert(53);
        node_set.insert(13);
        node_set.insert(61);

        let mut rules:HashMap<u32, Vec<u32>> = HashMap::new();
        rules.insert(47, vec![53,13]);
        rules.insert(53, vec![13,61]);
        rules.insert(13, vec![61]);

        let mut visited: HashSet<u32> = HashSet::new();
        let mut top_sort: Vec<u32> = Vec::new();

        dfs(node, &node_set, &mut visited, &mut top_sort, &rules);

        assert_eq!(top_sort, vec![61,13,53,47])

    }

    #[test]
    fn test_graph_parse() {
        let data = "47|53\n\
                          97|13\n\
                          97|61";
        let (result, size) = parse_2_tuple_input_with_delimiter::<u32>(data, '|');
        assert_eq!(result, vec![(47,53),(97,13),(97,61)]);
        assert_eq!(size, 3);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 1));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 1));
        assert_eq!(result, Some(123));
    }

    #[test]
    fn test_part_two_debugging() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(78));
    }
}
