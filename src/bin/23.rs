use indexmap::IndexMap;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

advent_of_code::solution!(23);

pub fn part_one(input: &str) -> Option<u64> {
    // clique problem (finding all size 3 cliques)

    let lines = input.lines();

    // node - connections
    let mut nodes: HashMap<&str, HashSet<&str>> = HashMap::new();

    for line in lines {
        let mut iter = line.split('-');
        let src = iter.next().expect("Expected pair");
        let dst = iter.next().expect("Expected pair");

        nodes
            .entry(src)
            .and_modify(|v| {
                v.insert(dst);
            })
            .or_insert({
                let mut set = HashSet::new();
                set.insert(dst);
                set
            });

        nodes
            .entry(dst)
            .and_modify(|v: &mut HashSet<&str>| {
                v.insert(src);
            })
            .or_insert({
                let mut set = HashSet::new();
                set.insert(src);
                set
            });
    }

    let mut result = 0;

    let tuples = nodes.iter().tuple_combinations::<(_, _, _)>();
    for (
        (node_1_name, node_1_connection),
        (node_2_name, node_2_connection),
        (node_3_name, node_3_connection),
    ) in tuples
    {
        if !node_1_name.starts_with("t")
            && !node_2_name.starts_with("t")
            && !node_3_name.starts_with("t")
        {
            continue;
        }

        // check of three nodes are interconnected
        if node_1_connection.contains(node_2_name)
            && node_1_connection.contains(node_3_name)
            && node_2_connection.contains(node_1_name)
            && node_2_connection.contains(node_3_name)
            && node_3_connection.contains(node_1_name)
            && node_3_connection.contains(node_2_name)
        {
            result += 1;
            //dbg!(&node_1_name, &node_2_name, &node_3_name);
        }
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<String> {
    // max clique problem

    let lines = input.lines();

    // node - connections
    // Use IndexMap for deterministic iteration over keys.
    let mut nodes: IndexMap<&str, HashSet<&str>> = IndexMap::new();

    for line in lines {
        let mut iter = line.split('-');
        let src = iter.next().expect("Expected pair");
        let dst = iter.next().expect("Expected pair");

        nodes.entry(src).or_default().insert(dst);
        nodes.entry(dst).or_default().insert(src);
    }

    // clique is just a set of nodes
    let mut cliques: Vec<HashSet<&str>> = Vec::new();

    for (node, connections) in nodes {
        // if node == "de" || node == "ka" || node == "co" || node == "ta" {
        //     dbg!(&node);
        // }

        // add to any clique that this node have all connections
        // also copy the clique, as a-b can be both an a-b-c and a-b-d
        // and create new own clique for a node
        let mut cliques_to_add: Vec<HashSet<&str>> = Vec::new();

        for clique in &cliques {
            if clique.iter().all(|n| connections.contains(n)) {
                let mut new_clique = clique.clone();
                new_clique.insert(node);
                cliques_to_add.push(new_clique);
            }
        }

        // add new cliques
        cliques.append(&mut cliques_to_add);

        // create new clique for a node
        cliques.push({
            let mut hs = HashSet::new();
            hs.insert(node);
            hs
        });
    }

    //dbg!(&cliques);
    let biggest_clique = cliques
        .iter()
        .sorted_by(|a, b| Ord::cmp(&b.len(), &a.len()))
        .next()
        .unwrap();

    //dbg!(&biggest_clique);

    let biggest_clique_vec_sorted: Vec<&str> = biggest_clique.iter().copied().sorted().collect();
    let result = biggest_clique_vec_sorted.join(",");

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("co,de,ka,ta".to_string()));
    }
}
