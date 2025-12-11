aoc::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {    
    // Initialize values
    let mut count: u64 = 0;
    let mut value: i32 = 50;

    for line in input.lines() {
        let n: i32 = line[1..].parse().unwrap();
        if line.starts_with("R") {
            value += n;
        } else if line.starts_with("L") {
            value -= n;
        }
        // Add value if modulo is zero
        if value % 100 == 0 {
            count += 1;
        }
    }

    Some(count)
}

pub fn part_two(_input: &str) -> Option<u64> {
    // println!("{input}");
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&aoc::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&aoc::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
