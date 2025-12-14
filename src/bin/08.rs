use std::collections::{BTreeMap, HashMap};

aoc::solution!(8);

pub fn part_one(input: &str) -> Option<u64> {
    let try_k_connections: u32 = 1000;  // Change to 10 when running part 1 example
    let n_junctions = input.trim().lines().count();
    // println!("Junctions: {n_junctions}");

    // Use squared euclidean distance as metric
    fn euclidean_sq_norm(from: (i64, i64, i64), to: (i64, i64, i64)) -> i64 {
        (from.0 - to.0).pow(2) + (from.1 - to.1).pow(2) + (from.2 - to.2).pow(2)
    }

    // Calculate all pairwise junction distances
    let mut positions: Vec<(i64, i64, i64)> = Vec::new();
    for line in input.trim().lines() {
        let parts: Vec<i64> = line.split(',').map(|s| s.parse().unwrap()).collect();
        positions.push((parts[0], parts[1], parts[2]));
    }
    // println!("Positions: {:?}", positions);

    // Keep track of distances and the from/to idx in a ranked table
    let mut rank_table: BTreeMap<i64, Vec<(usize, usize)>> = BTreeMap::new();
    for from_idx in 0..n_junctions {
        for to_idx in from_idx+1..n_junctions {
            let distance = euclidean_sq_norm(positions[from_idx], positions[to_idx]);

            // Insert distance, while keeping track of duplicates
            match rank_table.get_mut(&distance) {
                Some(value) => {value.push((from_idx, to_idx));},
                None => {rank_table.insert(distance, vec![(from_idx, to_idx)]);},
            };
        }
    }
    // println!("Distance rank table: {:?}", rank_table);

    // Keep track of the groups of circuits
    let mut circuit_groups: Vec<u32> = (0u32 .. (n_junctions as u32)).collect();
    // println!("Circuit groups: {:?}", circuit_groups);

    // Connect the first k junctions
    let mut n_connections: u32 = 0;
    'outer: for (_distance, links) in rank_table {
        for (from_idx, to_idx) in links {
            // Already the same circuit
            let from_circuit_idx = circuit_groups[from_idx];
            let to_circuit_idx = circuit_groups[to_idx];            
            if from_circuit_idx == to_circuit_idx {
                // println!("Group {from_idx} and {to_idx} already connected!");
            } else { // New connection, set all "to indices" to indices from "from indcies" group
                // println!("Connecting {from_idx} and {to_idx}!");
                circuit_groups.iter_mut().for_each(|x| {
                    if *x == to_circuit_idx as u32 {
                        *x = from_circuit_idx as u32;
                    }
                });
            }
            n_connections += 1;
            if n_connections == try_k_connections {
                break 'outer;
            }
        }
    }
    // println!("Circuit groups: {:?}", circuit_groups);

    // Count the circuits
    let mut counts = HashMap::new();
    for circuit in circuit_groups {
        *counts.entry(circuit).or_insert(0) += 1;
    }
    // println!("Counts: {:?}", counts);

    // Sort the counts and find the product of the top 3 cuircuts
    let mut count_values: Vec<_> = counts.values().cloned().collect();
    count_values.sort_unstable_by(|a, b| b.cmp(a)); // descending

    // Take top 3 and multiply
    let product: u64 = count_values.iter().take(3).product();

    Some(product)
}

pub fn part_two(input: &str) -> Option<u64> {
    let n_junctions = input.trim().lines().count();

    // Use squared euclidean distance as metric
    fn euclidean_sq_norm(from: (i64, i64, i64), to: (i64, i64, i64)) -> i64 {
        (from.0 - to.0).pow(2) + (from.1 - to.1).pow(2) + (from.2 - to.2).pow(2)
    }

    // Calculate all pairwise junction distances
    let mut positions: Vec<(i64, i64, i64)> = Vec::new();
    for line in input.trim().lines() {
        let parts: Vec<i64> = line.split(',').map(|s| s.parse().unwrap()).collect();
        positions.push((parts[0], parts[1], parts[2]));
    }

    // Keep track of distances and the from/to idx in a ranked table
    let mut rank_table: BTreeMap<i64, Vec<(usize, usize)>> = BTreeMap::new();
    for from_idx in 0..n_junctions {
        for to_idx in from_idx+1..n_junctions {
            let distance = euclidean_sq_norm(positions[from_idx], positions[to_idx]);

            // Insert distance, while keeping track of duplicates
            match rank_table.get_mut(&distance) {
                Some(value) => {value.push((from_idx, to_idx));},
                None => {rank_table.insert(distance, vec![(from_idx, to_idx)]);},
            };
        }
    }

    // Keep track of the groups of circuits
    let mut circuit_groups: Vec<u32> = (0u32 .. (n_junctions as u32)).collect();

    // Connect the first k junctions
    let mut n_connections_added: u32 = 0;
    let mut product: u64 = 0;
    'outer: for (_distance, links) in rank_table {
        for (from_idx, to_idx) in links {
            // Already the same circuit
            let from_circuit_idx = circuit_groups[from_idx];
            let to_circuit_idx = circuit_groups[to_idx];            
            if from_circuit_idx == to_circuit_idx {
                // println!("Group {from_idx} and {to_idx} already connected!");
            } else { // New connection, set all "to indices" to indices from "from indcies" group
                // println!("Connecting {from_idx} and {to_idx}!");
                circuit_groups.iter_mut().for_each(|x| {
                    if *x == to_circuit_idx as u32 {
                        *x = from_circuit_idx as u32;
                    }
                });
                n_connections_added += 1;
                if n_connections_added == (n_junctions as u32 - 1) {  // Only need n_junctions - 1 to connect all groups
                    product = positions[from_idx].0 as u64 * positions[to_idx].0 as u64;
                    break 'outer;
                }
            }
        }
    }

    // Verify that all groups are connected.
    // println!("Circuit groups: {:?}", circuit_groups);

    Some(product)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&aoc::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&aoc::template::read_file("examples", DAY));
        assert_eq!(result, Some(25272));
    }
}
