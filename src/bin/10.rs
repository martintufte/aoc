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
        let goal_state_raw = parts.next().unwrap();
        // println!("Goal state: {goal_state_raw}");
        // Convert to number
        let mut goal: u32 = 0;
        for (exp, ch) in goal_state_raw[1..].chars().enumerate() {
            goal += (if ch == '#' {1} else {0}) * 2u32.pow(exp as u32);
        }
        // println!("Goal number: {goal}");

        // Actions are all except last part
        let mut actions_raw: Vec<&str> = parts.collect();
        actions_raw.pop(); // Ignore last part
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

pub fn part_two(_input: &str) -> Option<u64> {
    None
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
        assert_eq!(result, None);
    }
}
