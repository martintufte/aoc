aoc::solution!(9);

pub fn part_one(input: &str) -> Option<u64> {
    // Parse input as x, y coordinates
    let mut coordinates: Vec<(i64, i64)> = Vec::new();

    for line in input.trim().lines() {
        let parts: Vec<i64> = line.split(',').map(|s| s.parse().unwrap()).collect();
        coordinates.push((parts[0], parts[1]));
    }

    // Brute force through all pairs and calculate the rectangle size
    let n_coordinates = coordinates.len();
    let mut largest_rectangle: u64 = 1;
    for i in 0..n_coordinates {
        let current_coordinate = coordinates[i];
        for j in i..n_coordinates {
            let other_coordinate = coordinates[j];
            let new_rectangle: u64 = (current_coordinate.0.abs_diff(other_coordinate.0) + 1) * (current_coordinate.1.abs_diff(other_coordinate.1) + 1);
            largest_rectangle = largest_rectangle.max(new_rectangle);
        }
    }

    Some(largest_rectangle)
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
        assert_eq!(result, Some(50));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&aoc::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
