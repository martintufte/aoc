use std::char;

aoc::solution!(3);

pub fn part_one(input: &str) -> Option<u64> {
    let mut total_joltage: u64 = 0;
    let order = "987654321";

    // Loop over banks, find joltage
    for line in input.lines() {
        assert!(line.len() >= 2);

        // Naive approach, search for 99, 98, ...
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

pub fn part_two(input: &str) -> Option<u64> {
    let mut total_joltage: u64 = 0;
    let order = "987654321";
    let n_batteries = 12;

    // Find the first occurance with more than k remaining chars in the string
    fn greedy_find_first(string: &str, ch: char, k: usize) -> Option<usize> {
        match string.find(ch) {
            Some(idx) if (string.len() - idx) > k => Some(idx),
            Some(_) => None,
            None => None,
        }
    }

    // Loop over banks, find joltage
    for line in input.lines() {
        assert!(line.len() >= n_batteries);

        // Greedy approach, search for first digit with n-1 left
        // then start searching from the remaining subset
        let mut joltage: u64 = 0;
        let mut remaining_string: &str = &line;

        for k in (0 .. n_batteries).rev() {
            for ch in order.chars() {
                match greedy_find_first(remaining_string, ch, k) {
                    Some(idx) => {
                        let digit: u64 = ch.to_digit(10).unwrap() as u64;
                        joltage = joltage * 10 + digit;
                        remaining_string = &remaining_string[idx + 1 ..];
                        break;
                    },
                    None => continue,
                }
            }
        }
        total_joltage += joltage;
    }

    Some(total_joltage)
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
        assert_eq!(result, Some(3121910778619));
    }
}
