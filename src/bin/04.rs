use std::char;

aoc::solution!(4);

pub fn part_one(input: &str) -> Option<u64> {
    // Use a count array
    // Combine input into a single trimmed string
    let trimmed: &str = input.trim();
    let contigous_input = trimmed.replace("\n", "");
    let n_cols = trimmed.lines().next().unwrap().len();
    let n_rows = contigous_input.len() / n_cols;

    println!("n_cols {n_cols}");
    println!("n_rows {n_rows}");

    fn locate_in_contiguous(string: &str, row: usize, col: usize, n_cols: usize) -> Option<char> {
        string.chars().nth(row * n_cols + col)
    }

    let mut n_paper_rolls: u64 = 0;
    
    // Loop over each character. Count number of neighbours
    for (i, ch) in contigous_input.chars().enumerate() {
        if ch == '@' {
            let row = i / n_cols;
            let col = i % n_cols;
            let mut n_neighbors: u64 = 0;

            // Limit neighbor search to boundary
            let min_neighbor_row = if row == 0 {0} else {row - 1};
            let max_neighbor_row = if row == n_rows - 1 {n_rows - 1} else {row + 1};
            let min_neighbor_col = if col == 0 {0} else {col - 1};
            let max_neighbor_col = if col == n_cols - 1 {n_cols - 1} else {col + 1};

            for i in min_neighbor_row..=max_neighbor_row {
                for j in min_neighbor_col..=max_neighbor_col {
                    match locate_in_contiguous(&contigous_input, i, j, n_cols) {
                        Some(ch) if ch == '@' => {
                            n_neighbors += 1;
                        },
                        Some(_) => {},
                        None => {},
                    }
                }
            }
            if n_neighbors < 5 {  // less than 5 neighbors as self is counted
                n_paper_rolls += 1;
            }
        }
    }

    Some(n_paper_rolls)
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
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&aoc::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
