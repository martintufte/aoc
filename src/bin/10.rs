use std::collections::HashSet;

aoc::solution!(10);

pub fn part_one(input: &str) -> Option<u64> {
    // Parse the input as the desired state and the possible toggles
    let mut running_total = 0;

    // Get the binary expansion of a number
    fn binary_indices(mut n: u32) -> Vec<usize> {
        let mut indices = Vec::new();
        let mut i = 0;

        while n > 0 {
            if n & 1 == 1 {
                indices.push(i);
            }
            n >>= 1;
            i += 1;
        }

        indices
    }

    for line in input.trim().lines() {
        let mut parts = line.split(' ');

        // Goal state is the first part
        let goal_part_one = parts.next().unwrap();
        // println!("Goal state: {goal_part_one}");

        // Convert goal to number
        let mut goal: u32 = 0;
        for (exp, ch) in goal_part_one[1..].chars().enumerate() {
            goal += (if ch == '#' {1} else {0}) * 2u32.pow(exp as u32);
        }
        // println!("Goal number: {goal}");

        // Actions are all except last part
        let mut actions_raw: Vec<&str> = parts.collect();
        let _goal_part_two = actions_raw.pop(); // Ignore part two goal
        // println!("Actions: {:?}", actions_raw);

        // Convert to integers
        let mut actions: Vec<u32> = Vec::new();
        for raw in actions_raw {
            let mut action: u32 = 0;
            for k in raw[1..raw.len()-1].split(',') {
                let k_as_u32: u32 = k.parse().unwrap();
                action += 2u32.pow(k_as_u32);
            }
            actions.push(action);
        }
        // println!("Actions as int: {:?}", actions);

        // Brute force over all binary masks and pick the one with lowest bitsum
        let n_actions = actions.len();
        let mut total: u64 = n_actions as u64 + 1;
        //let solutions: BTreeMap<u32, Vec<u32>> = BTreeMap::new();
        for n in 0..2u32.pow(n_actions as u32) {
            let mut state = 0;  // All switches are initially off
            // println!("Check number {n}");
            for b in binary_indices(n) {
                state ^= actions[b];  // XOR
            }
            if state == goal {
                // Insert solution
                total = total.min(n.count_ones() as u64);
            }
        }

        // println!("Required {total} presses.");

        running_total += total;
    }

    Some(running_total)
}

pub fn part_two(input: &str) -> Option<u64> {
    // Parse the input as the desired state and the possible toggles
    let mut running_total = 0;

    for line in input.trim().lines() {
        let mut parts = line.split(' ');

        // Ignore part one goal
        let _goal_part_one = parts.next().unwrap();

        // Actions are all except last part
        let mut actions_raw: Vec<&str> = parts.collect();
        let goal_state_raw = actions_raw.pop().unwrap();

        // Convert goal to vector
        let mut goal_vec: Vec<u32> = Vec::new();
        for ch in goal_state_raw[1..goal_state_raw.len()-1].split(',') {
            let as_int: u32 = ch.parse().unwrap();
            goal_vec.push(as_int);
        }
        // println!("Goal vector: {:?}", goal_vec);

        // Convert to integers
        let mut actions: Vec<u32> = Vec::new();
        for raw in actions_raw {
            let mut action: u32 = 0;
            for k in raw[1..raw.len()-1].split(',') {
                let k_as_u32: u32 = k.parse().unwrap();
                action += 2u32.pow(k_as_u32);
            }
            actions.push(action);
        }
        // println!("Actions as int: {:?}", actions);

        // Use a HashSet to store the shortest path to all possibilities
        let mut explored_states: HashSet<Vec<u32>> = HashSet::new();

        // Frontier of the search
        let mut frontier: HashSet<Vec<u32>> = HashSet::new();
        frontier.insert(goal_vec.clone());

        // Start with goal vector and iteratively subtract possible actions
        let mut depth: u64 = 0;
        'outer: loop {
            depth += 1;
            let mut new_frontier: HashSet<Vec<u32>> = HashSet::new();

            // Iterate over frontier
            for state in &frontier {
                // Try subtracting all actions, and add to new frontier
                'inner: for &action in actions.iter() {
                    let mut new_state: Vec<u32> = Vec::new();
                    for (idx, curr) in state.iter().enumerate() {
                        let bit = (action >> idx) & 1;
                        if bit > *curr {  // Subtraction is not possible, continue to next
                            continue 'inner;
                        }
                        new_state.push(curr - bit);
                    }

                    // Check if we have reached <0, 0, ...>
                    if new_state.iter().all(|&x| x == 0) {
                        println!("Found solution, depth = {:?}", depth);
                        running_total += depth;
                        break 'outer;
                    }

                    // If not explored, add to new_frontier
                    if ! explored_states.contains(&new_state) {
                        new_frontier.insert(new_state);
                    }
                explored_states.insert(state.clone());
                }
            }
            frontier = new_frontier;  // Update frontier to new frontier
        }
    }

    Some(running_total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&aoc::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&aoc::template::read_file("examples", DAY));
        assert_eq!(result, Some(33));
    }
}
