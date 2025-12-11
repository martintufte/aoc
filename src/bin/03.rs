aoc::solution!(3);

pub fn part_one(input: &str) -> Option<u64> {
    let mut total_joltage: u64 = 0;

    // Loop over banks, find joltage
    for line in input.lines() {
        assert!(line.len() >= 2);

        // Naive approach, search for 99, 98, ...
        let order = "987654321";

        'outer: for ch1 in order.chars() {
            if let Some(index) = line.find(ch1) {
                for ch2 in order.chars() {
                    if line[index + 1 ..].find(ch2).is_some() {
                        let joltage: u64 = format!("{}{}", ch1, ch2).parse().unwrap();
                        total_joltage += joltage;

                        break 'outer;  // Break outer loop
                    }
                }
            }
        }
    }

    Some(total_joltage)
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
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&aoc::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
