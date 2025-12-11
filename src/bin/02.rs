aoc::solution!(2);

pub fn part_one(input: &str) -> Option<u64> {
    let mut count: u64 = 0;

    // Loop over all ids, and check if they are invalid
    for line in input.trim().split(',') {
        let ranges: Vec<&str> = line.split('-').collect();
        assert!(ranges.len() == 2);

        let lower_range: u64 = ranges[0].parse().unwrap();
        let upper_range: u64 = ranges[1].parse().unwrap();

        for n in lower_range..=upper_range {
            // Convert back to string
            let s = n.to_string();

            // Discard numbers that have odd number of digits
            let k = s.len();
            if k % 2 == 1 {
                continue
            }
            let middle = k / 2;

            // Check if start equals end
            let start: &str = &s[.. middle];
            let end: &str = &s[middle ..];
            
            // If equality, add id
            if start == end {
                count += n;
            }
        }
    }

    Some(count)
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
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&aoc::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
