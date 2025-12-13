aoc::solution!(6);

pub fn part_one(input: &str) -> Option<u64> {
    let mut it = input.trim().lines().rev();

    // Extract operators
    let operator_line = it.next();
    let operators: Vec<&str> = operator_line.unwrap().split_whitespace().collect();

    // Initialize a cumulative sum/product, starting with the default value
    let mut cumulative: Vec<u64> = operators
        .iter()
        .map(|&op| if op == "*" {1} else {0})
        .collect();

    // Iterate over the input and arange into columns of numbers
    for line in it {
        for (i, col) in line.split_whitespace().enumerate() {
            let value: u64 = col.parse().unwrap();
            if operators[i] == "*" {
                cumulative[i] *= value;
            } else {
                cumulative[i] += value;
            }
        }
    }

    // Grand total
    Some(cumulative.iter().sum())
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
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&aoc::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
