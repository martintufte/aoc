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

pub fn part_two(input: &str) -> Option<u64> {
    let mut it = input.lines().rev();

    // Extract operators
    let operator_line = it.next();
    let operators: Vec<&str> = operator_line.unwrap().split_whitespace().collect();

    // Initialize a cumulative sum/product, starting with the default value
    let mut cumulative: Vec<u64> = operators
        .iter()
        .map(|&op| if op == "*" {1} else {0})
        .collect();

    // Manipulate the order of the text
    let line_width = operator_line.unwrap().len();

    // Create a grid of chars, filtering out newlines
    let mut grid: Vec<char> = input.chars().filter(|&b| b != '\n').collect();
    grid = grid[..grid.len() - line_width].to_vec(); // Drop operator line
    let n_rows = grid.len() / line_width;

    // Sum up total
    let mut op_idx: usize = 0;
    for c in 0..line_width {
        // Construct string
        let mut s = String::new();
        for r in 0..n_rows {
            s.push(grid[c + line_width * r]);

        }

        // Trim string
        let trimmed = s.trim();

        // If empty, advance iterator over operators
        // Else add value to cumulative
        if trimmed == "" {
            op_idx += 1;
        } else {
            let value: u64 = trimmed.parse().unwrap();
            if operators[op_idx] == "*" {
                cumulative[op_idx] *= value;
            } else {
                cumulative[op_idx] += value;
            }
        }

    }

    // Grand total
    Some(cumulative.iter().sum())
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
        assert_eq!(result, Some(3263827));
    }
}
