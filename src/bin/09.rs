use std::collections::HashSet;

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

// Note: Implementation is super-slow, takes ~1000s to run
pub fn part_two(input: &str) -> Option<u64> {
    // Parse input as x, y coordinates
    let mut coordinates: Vec<(u32, u32)> = Vec::new();

    for line in input.trim().lines() {
        let parts: Vec<u32> = line.split(',').map(|s| s.parse().unwrap()).collect();
        coordinates.push((parts[0], parts[1]));
    }

    // Create HashMap of perimiter
    let mut perimeter: HashSet<(u32, u32)> = HashSet::new();
    for (first, second) in coordinates.iter().zip(coordinates.iter().cycle().skip(1)) {
        // Add line segment indices, first point + add points in-between
        if first.0 == second.0 {  // Matching x-coordinate
            if first.1 < second.1 {
                for idx in first.1..=second.1 {
                    perimeter.insert((first.0, idx));
                }
            } else {
                for idx in second.1..=first.1 {
                    perimeter.insert((first.0, idx));
                }
            }
        } else {  // Matching y-coordinate
            if first.0 < second.0 {
                for idx in first.0..=second.0 {
                    perimeter.insert((idx, first.1));
                }
            } else {
                for idx in second.0..=first.0 {
                    perimeter.insert((idx, first.1));
                }
            }
        }
    }

    // Brute force through all pairs and calculate the rectangle size
    let n_coordinates = coordinates.len();
    let mut largest_rectangle: u64 = 0;  // 1396494456

    for i in 0..n_coordinates {
        let current_coordinate = coordinates[i];
        'middle: for j in i..n_coordinates {
            let new_coordinate = coordinates[j];
            let new_rectangle: u64 = (current_coordinate.0.abs_diff(new_coordinate.0) as u64 + 1) * (current_coordinate.1.abs_diff(new_coordinate.1) as u64 + 1);

            // If rectangle is bigger, check that the interior perimeter does not contain any perimeter tiles
            if new_rectangle > largest_rectangle {
                if current_coordinate.0 == new_coordinate.0 {  // Safely skip when one coordinate matches
                    // println!("1 Skipped: {new_rectangle}");
                    continue 'middle;
                } else if current_coordinate.1 == new_coordinate.1 {
                    // println!("2 Skipped: {new_rectangle}");
                    continue 'middle;
                }

                // Construct interior points
                let x_high: u32 = (current_coordinate.0 - 1).max(new_coordinate.0 - 1);
                let x_low: u32 = (current_coordinate.0 + 1).min(new_coordinate.0 + 1);
                let y_high: u32 = (current_coordinate.1 - 1).max(new_coordinate.1 - 1);
                let y_low: u32 = (current_coordinate.1 + 1).min(new_coordinate.1 + 1);

                // Check if line segments between interior corners are contained in perimeter -> reject 
                for x in x_low..=x_high {
                    if perimeter.contains(&(x, y_low)) {
                        // println!("3 Skipped: {new_rectangle}");
                        continue 'middle;
                    } else if perimeter.contains(&(x, y_high)) {
                        // println!("4 Skipped: {new_rectangle}");
                        continue 'middle;
                    }
                }
                for y in y_low..=y_high {
                    if perimeter.contains(&(x_low, y)) {
                        // println!("5 Skipped: {new_rectangle}");
                        continue 'middle;
                    } else if perimeter.contains(&(x_high, y)) {
                        // println!("6 Skipped: {new_rectangle}");
                        continue 'middle;
                    }
                }

                // Found new largest rectangle!
                println!("Found new largest rectangle: {new_rectangle}");
                largest_rectangle = new_rectangle;
            }
        }
    }

    Some(largest_rectangle)
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
        assert_eq!(result, Some(24));
    }
}
