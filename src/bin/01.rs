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

pub fn part_two(input: &str) -> Option<u64> {
    // Initialize values
    let mut count: u64 = 0;
    let mut value: i32 = 50;

    // Implement mathematical positive modulo
    fn modulo(number: i32, m: i32) -> i32 {
        ((number % m) + m) % m
    }

    // Keep value between 0 and 99, track "offset" from zero
    for line in input.lines() {
        let n: i32 = line[1..].parse().unwrap();
        if line.starts_with("R") {
            count += ((value + n) / 100) as u64;
            value += n;
        } else if line.starts_with("L") {
            let offset: i32 = if value == 0 {0} else {100 - value};
            count += ((offset + n) / 100) as u64;
            value -= n;
        }
        value = modulo(value, 100);
    }

    Some(count)
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
        assert_eq!(result, Some(6));
    }
}
