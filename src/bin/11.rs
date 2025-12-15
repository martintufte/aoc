use std::collections::{HashMap, HashSet, VecDeque};

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

pub fn part_two_first_try(input: &str) -> Option<u64> {
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

    // Keep track of the number of paths and the FIFO queue of the search
    // Need to have a hashset next to the queue to not add duplicates to the queue
    // First number is the total paths, second number is the paths that goes through "dac"
    // Third is the number of paths that goes through "fft", forth is either or
    let mut n_paths: HashMap<&str, (u128, u128, u128, u128)> = HashMap::new();
    let mut frontier: VecDeque<&str> = VecDeque::new();
    let mut processed: HashSet<&str> = HashSet::new();
    n_paths.insert("svr", (1, 0, 0, 0));
    frontier.push_back("svr");
    processed.insert("svr");
    // println!("Links: {:?}", links);

    while let Some(node) = frontier.pop_front() {

        if ! links.contains_key(node) {
            continue;
        }

        // println!("Processing node {node}");
        // println!("{:?}", n_paths[node]);

        // Add n_paths to self if node is dat or fft
        if node == "dac" {
            // println!("Found dac: {:?}", n_paths[node]);
            let node_paths: (u128, u128, u128, u128) = (
                n_paths[node].0,
                n_paths[node].1 + n_paths[node].0,
                n_paths[node].2,
                n_paths[node].3 + n_paths[node].2, // If already have path to "fft", then the path is now to both
            );
            n_paths.insert(node, node_paths);
        } else if node == "fft" {
            // println!("Found fft: {:?}", n_paths[node]);
            let node_paths: (u128, u128, u128, u128) = (
                n_paths[node].0,
                n_paths[node].1,
                n_paths[node].2 + n_paths[node].0,
                n_paths[node].3 + n_paths[node].1, // If already have path to "dac", then the path is now to both
            );
            n_paths.insert(node, node_paths);
        }

        for &child in links[node].iter() {

            if n_paths.contains_key(child) {
                let child_paths: (u128, u128, u128, u128) = (
                    n_paths[child].0 + n_paths[node].0,
                    n_paths[child].1 + n_paths[node].1,
                    n_paths[child].2 + n_paths[node].2,
                    n_paths[child].3 + n_paths[node].3,
                );
                n_paths.insert(child, child_paths);
            } else {
                n_paths.insert(child, n_paths[node]);
            }

            // Add to frontier if not processed
            if !processed.contains(child) {
                processed.insert(child);
                frontier.push_back(child);
            }
        }
    }

    println!("{:?}", n_paths["out"]);
    Some(n_paths["out"].3 as u64)
}

// Second try using topological sort
pub fn part_two(input: &str) -> Option<u64> {
    // Read the input, constructing a directed graph of links
    // Keep track of in degree to construct topological order
    let mut links: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut in_degree: HashMap<&str, usize> = HashMap::new(); 
    for line in input.trim().lines() {
        let mut it = line.split(" ");
        let mut node = it.next().unwrap();
        node = &node[0..node.len()-1];         
        let mut children_nodes: Vec<&str> = Vec::new();
        for child in it {
            children_nodes.push(child);
            *in_degree.entry(child).or_insert(0) += 1
        }
        links.insert(node, children_nodes);
    }

    // println!("Links: {:?}", links);
    // println!("In-degree: {:?}", in_degree);

    // Topological sort, start with a FIFO queue
    let mut queue: VecDeque<&str> = VecDeque::new();
    queue.push_back("svr");  // Only starting node is "svr"

    // Add the rest of the nodes in 
    let mut topological_order: Vec<&str> = Vec::new();
    while let Some(node) = queue.pop_front() {
        topological_order.push(node);
        if let Some(children) = links.get(node) {
            for &child in children {
                let degree = in_degree.get_mut(child).unwrap();
                *degree -= 1;  // Decrement the in degree of the child
                if *degree == 0 {  // No pending ancestors, add to topological order
                    queue.push_back(child);
                }
            }
        }
    }

    // println!("Topological order: {:?}", topological_order);

    // Keep track of the number of paths
    // First number is the total paths, second number is the paths that goes through "dac"
    // Third is the number of paths that goes through "fft", forth is either "dac" or "fft"
    let mut n_paths: HashMap<&str, (u64, u64, u64, u64)> = HashMap::new();
    n_paths.insert("svr", (1, 0, 0, 0));

    for node in topological_order {

        if ! links.contains_key(node) {
            continue;
        }

        // Add n_paths to self if node is dat or fft
        if node == "dac" {
            // println!("Found dac: {:?}", n_paths[node]);
            let node_paths: (u64, u64, u64, u64) = (
                n_paths[node].0,
                n_paths[node].1 + n_paths[node].0,
                n_paths[node].2,
                n_paths[node].3 + n_paths[node].2, // If already have path to "fft", then the path is now to both
            );
            n_paths.insert(node, node_paths);
        } else if node == "fft" {
            // println!("Found fft: {:?}", n_paths[node]);
            let node_paths: (u64, u64, u64, u64) = (
                n_paths[node].0,
                n_paths[node].1,
                n_paths[node].2 + n_paths[node].0,
                n_paths[node].3 + n_paths[node].1, // If already have path to "dac", then the path is now to both
            );
            n_paths.insert(node, node_paths);
        }

        for &child in links[node].iter() {

            if n_paths.contains_key(child) {
                let child_paths: (u64, u64, u64, u64) = (
                    n_paths[child].0 + n_paths[node].0,
                    n_paths[child].1 + n_paths[node].1,
                    n_paths[child].2 + n_paths[node].2,
                    n_paths[child].3 + n_paths[node].3,
                );
                n_paths.insert(child, child_paths);
            } else {
                n_paths.insert(child, n_paths[node]);
            }
        }
    }

    // println!("{:?}", n_paths["out"]);
    Some(n_paths["out"].3 as u64)
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
        assert_eq!(result, Some(2));
    }
}
