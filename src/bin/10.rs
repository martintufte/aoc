use std::collections::{HashMap, HashSet};

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

// Working solution using bhs -> ended up being too time consuming
pub fn part_two_initial_solution(input: &str) -> Option<u64> {
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


// Note: Final solution
// Ended up with parity-based recursive divide-and-conquer with memoization
// Tried dynamical programming, integer programming with optimizations, clever matrix manipulations,
// thought about bidirectional search, min-cut max flow ++ before solving it
pub fn part_two(input: &str) -> Option<u64> {
    // Parse the input as the desired state and the possible toggles
    let mut total_cost = 0;

    // Recursively solve f(n) = min(cost + f((n - element) / 2) for (element, cost) in parity(n % 2))
    fn compute_cost(
        state: Vec<u32>,
        parity_elements: &HashMap<Vec<bool>, Vec<(Vec<u32>, u64)>>,
        memory: &mut HashMap<Vec<u32>, u64>,
        cost_upper_bound: u64,
    ) -> u64 {
        // Maybe return memoized solution
        if let Some(&cost) = memory.get(&state) {
            return cost;
        }

        // Find parity (note: could have also used mod 3 instead of 2, but that seemed unnecessarily complex)
        let parity: Vec<bool> = state.iter().map(|x| x % 2 == 1).collect();

        // Go over all possible elements matching the parity, finding the shortest path
        let mut state_cost: u64 = cost_upper_bound;
        if let Some(elements_costs) = parity_elements.get(&parity) {
            'element_loop: for (element, cost) in elements_costs {
                // Only check element if state >= element
                // At least one element will be accepted as we are guarenteed a solution
                for (s, e) in state.iter().zip(element) {
                    if e > s {
                        continue 'element_loop;
                    }
                }

                let remainder: Vec<u32> = state.iter().zip(element).map(|(s,e)| (s - e) / 2).collect();

                state_cost = state_cost.min(cost + 2 * compute_cost(remainder, &parity_elements, memory, cost_upper_bound));
            }
        } else {
            // Debug: Print dead ends in the search
            // println!("Dead end, returning upper bound for path");
        }

        // Save new cost to memory
        memory.insert(state.clone(), state_cost);

        // Return cost
        state_cost
    }

    for line in input.trim().lines() {
        let mut parts = line.split(' ');

        // Ignore part one goal
        let _goal_part_one = parts.next().unwrap();

        // Actions are all except last part
        let mut actions_raw: Vec<&str> = parts.collect();
        let goal_state_raw = actions_raw.pop().unwrap();
        let n_actions = actions_raw.len();

        // Convert goal to state vector
        let mut goal_state: Vec<u32> = Vec::new();
        // let mut max_goal: u32 = 0;
        for ch in goal_state_raw[1..goal_state_raw.len()-1].split(',') {
            let as_int: u32 = ch.parse().unwrap();
            goal_state.push(as_int);
            // max_goal = max_goal.max(as_int)
        }
        let n_goals = goal_state.len();

        // Parse actions as state vectors
        let mut actions: Vec<Vec<u32>> = vec![vec![0u32; n_goals]; n_actions];
        for (action_idx, raw) in actions_raw.iter().enumerate() {
            for k in raw[1..raw.len()-1].split(',') {
                let idx: usize = k.parse().unwrap();
                actions[action_idx][idx] = 1;
            }
        }
        // Debug: Verify actions
        // println!("Actions as states: {:?}", actions);

        // Create table lookup storing parity for all combinations of actions (elements)
        // Used to hash on the parity later to get all possible contenders
        let mut parity_elements: HashMap<Vec<bool>, Vec<(Vec<u32>, u64)>> = HashMap::new();

        // Go through all combinations of actions, store combination in table with fewest actions needed
        for n in 0..(1 << n_actions) {
            let mut element: Vec<u32> = vec![0; n_goals];
            let mut cost: u64 = 0;
            for (idx, action) in actions.iter().enumerate() {
                if (n >> idx) & 1 == 1 { // bit is 1
                    element = element.iter().zip(action).map(|(x, y)| x + y).collect();
                    cost += 1;
                }
            }
            let parity: Vec<bool> = element.iter().map(|x| x % 2 == 1).collect();

            // Add to parity table
            if !parity_elements.contains_key(&parity) {
                parity_elements.insert(parity, vec![(element, cost)]);
            } else {
                let v = parity_elements.get_mut(&parity).unwrap();
                v.push((element, cost));
            }
        }

        // Debug: Parity elements
        // println!("Parity elements: {:?}", parity_elements);

        // Keep memory of the computed cost
        let mut memory: HashMap<Vec<u32>, u64> = HashMap::new();
        memory.insert(vec![0u32; n_goals], 0u64);

        // Compute lowest cost
        let cost = compute_cost(
            goal_state,
            &parity_elements,
            &mut memory,
            1e10 as u64,  // Workaround for inf, has to be >= sum(state)
        );

        // Debug: Computed lowest cost vs. max goal
        // println!("Cost: {cost}, max cost: {max_goal}");

        // Simply add the maximum of the goal numbers
        total_cost += cost;
    }

    Some(total_cost)
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
