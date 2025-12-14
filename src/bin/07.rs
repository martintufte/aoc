aoc::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    // Parse lines, get line width
    let mut it = input.trim().lines();

    // Create beam mask with initial source
    let source_line = it.next().unwrap();
    let line_width = source_line.len();
    let mut beam_mask: Vec<bool> = vec![false; line_width];
    if let Some(source_idx) = source_line.find('S') {
        beam_mask[source_idx] = true;
    }

    // Count number of splits
    let mut n_splits: u64 = 0;

    // Iterate over the splitter lines
    for line in it {
        // Clone old beam
        let mut new_beam: Vec<bool> = beam_mask.clone();

        // Keep track of which indices contain the splitter
        let mut splitters: Vec<usize> = Vec::new();
        for (idx, ch) in line.char_indices() {
            if ch == '^' {
                // Store splitter
                splitters.push(idx);

                // Splitter reached
                if beam_mask[idx] == true {
                    new_beam[idx - 1] = true;
                    new_beam[idx + 1] = true;
                    n_splits += 1;
                }
            }
        }

        // Stop the beam behind the splitters
        for idx in splitters {
            new_beam[idx] = false;
        }

        // Update beam mask
        beam_mask = new_beam;
    }

    Some(n_splits)
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
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&aoc::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
