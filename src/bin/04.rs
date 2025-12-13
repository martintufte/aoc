aoc::solution!(4);

pub fn part_one(input: &str) -> Option<u64> {
    // Trim input and count number of columns and rows
    let input= input.trim();
    let n_cols = input.lines().next()?.len();
    let n_rows = input.lines().count();

    println!("n_cols {n_cols}");
    println!("n_rows {n_rows}");

    // Create a grid of chars, filtering out newlines
    let grid: Vec<u8> = input.bytes().filter(|&b| b != b'\n').collect();

    let mut n_paper_rolls: u64 = 0;
    
    // Loop over each character. Count number of neighbours
    for idx in 0..grid.len() {
        if grid[idx] != b'@' {
            continue
        }
        let row = idx / n_cols;
        let col = idx % n_cols;
        let mut n_neighbors: u64 = 0;

        // Check 3x3 neighborhood
        for dr in [-1isize, 0, 1] {
            for dc in [-1isize, 0, 1] {
                let r = row as isize + dr;
                let c = col as isize + dc;

                if r >= 0 && r < n_rows as isize && c >= 0 && c < n_cols as isize {
                    let nidx = (r as usize) * n_cols + (c as usize);
                    n_neighbors += (grid[nidx] == b'@') as u64;
                }
            }
        }

        if n_neighbors < 5 {  // less than 5 neighbors as self is counted
            n_paper_rolls += 1;
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
