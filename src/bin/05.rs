use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::result;
use std::str::FromStr;

// use petgraph::graph::{NodeIndex, DiGraph};
// use petgraph::algo::{dijkstra, min_spanning_tree};
// use petgraph::data::FromElements;
// use petgraph::dot::{Dot, Config};
// use petgraph::unionfind::UnionFind;
// use petgraph::visit::NodeIndexable;
// use petgraph::visit::EdgeRef;
// use petgraph::algo::has_path_connecting;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let mut iterator =
        input.split("\n\n").into_iter();
    let graph_data = iterator.next().unwrap();
    let sort_data = iterator.next().unwrap();

    let (parsed_graph_data, _size) = parse_2_tuple_input_with_delimiter::<u32>(graph_data, '|');
    let g = create_graph::<u32>(parsed_graph_data);

    let (lines, size) = parse_row_input::<usize>(sort_data, ',');

    //let mut res:Vec<bool> =Vec::with_capacity(lines.len());
    let mut acc = 0;
    for i in 0..lines.len()
    {
        //res.push(check_order(&g, &lines[i]));
        if check_order(&g, &lines[i])
        {
            let middle_index = &lines[i].len()/2;
            acc += lines[i][middle_index];
        }
    }

    // Not working due to cycles in the initial graph
    // From reddit -
    // The fact that the looping cases never appear in
    // the same test case means it works as long as you
    // only dfs over numbers that are in the same test
    // case (correct me if I'm wrong)

    //dbg!(res);

    Some(acc as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn dfs(
    node: i32,
    node_set :HashSet<i32>,
    visited: &mut HashSet<i32>,
    topsort: &mut Vec<i32>,
    rules: &HashMap<i32, Vec<i32>>,
) {
    if visited.contains(&node) {
        return;
    }
    visited.insert(&node)
    for dst in rules.get(&node).unwrap_or(&vec![]) {
        if !node_set.contains(dst) {
            continue;
        }
        dfs(*dst,node_set, visited, topsort, rules);
    }
    topsort.push(node);
}

fn create_graph<T>(data :Vec<(T,T)>) -> DiGraph<T, ()>
where
    T: std::marker::Copy,
    T: Into<NodeIndex>,
    T: Default
{
    return DiGraph::<T, ()>::from_edges(&data);
}

fn has_path<T>(g : &DiGraph<T, ()>, path:(usize,usize))->bool
where
    T: std::marker::Copy,
    T: Into<NodeIndex>
{
    let has_path = has_path_connecting(g, NodeIndex::new(path.0),NodeIndex::new(path.1), None);
    has_path
}

fn compare<T>(g : &DiGraph<T, ()>, first:usize, second:usize)-> i32
where T: Into<NodeIndex>,
T: std::marker::Copy,
{
    return if has_path(g, (first, second)) { -1 } else { 1};
}

fn check_order<T>(g : &DiGraph<T, ()>, data:&Vec<usize>) -> bool
where T: Into<NodeIndex>,
T: std::marker::Copy,
{
    for i in 1..data.len()
    {
        if compare(g, data[i-1], data[i]) > 0 { return false;}
    }

    return true;
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
    fn test_graph_build() {
        let data = vec![(1,2),(97,13),(97,61)];
        let g = create_graph(data);

        let res = has_path(&g, (1,2));
        assert_eq!(res, true);

        let res = has_path(&g, (2,1));
        assert_eq!(res, false);

        let res = has_path(&g, (1,1));
        assert_eq!(res, true);

        let res = has_path(&g, (1,13));
        assert_eq!(res, false);

        // compare

        let res = compare(&g, 1,2);
        assert_eq!(res, -1);

        let res = compare(&g, 2,1);
        assert_eq!(res, 1);

        let res = compare(&g, 1,13); // for non comparable is always true (no path)
        assert_eq!(res, 1);

        let res = compare(&g, 13,1); // for non comparable is always true (no path)
        assert_eq!(res, 1);
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
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
