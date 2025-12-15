use std::collections::{HashMap, HashSet};

aoc::solution!(11);

pub fn part_one(input: &str) -> Option<u64> {
    // Read the input, constructing a directed graph of links
    let mut links: HashMap<&str, Vec<&str>> = HashMap::new();
    for line in input.trim().lines() {
        let mut it = line.split(" ");
        let mut node = it.next().unwrap();
        node = &node[0..node.len()-1];         
        let mut children_nodes: Vec<&str> = Vec::new();
        for children in it {
            children_nodes.push(children);
        }
        links.insert(node, children_nodes);
    }
    // println!("Links: {:?}", links);

    // Keep track of the number of paths and the FIFO queue of the search
    let mut n_paths: HashMap<&str, u64> = HashMap::new();
    let mut frontier: HashSet<&str> = HashSet::new();
    n_paths.insert("you", 1);
    frontier.insert("you");

    while frontier.len() > 0 {
        let mut new_frontier: HashSet<&str> = HashSet::new();
        for &node in frontier.iter() {
            if ! links.contains_key(node) {
                continue;
            }

            for &child in links[node].iter() {
                if n_paths.contains_key(child) {
                    n_paths.insert(child, n_paths[child] + n_paths[node]);  // Add paths from node
                } else {
                    n_paths.insert(child, n_paths[node]);
                }
                new_frontier.insert(child);
            }
        }
        frontier = new_frontier;
    }

    Some(n_paths["out"])
}

pub fn part_two(_input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&aoc::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&aoc::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
