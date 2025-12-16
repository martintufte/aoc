aoc::solution!(12);

pub fn part_one(input: &str) -> Option<u64> {
    // Parse the five shapes and the grids
    let mut presents: Vec<Vec<bool>> = Vec::new();

    // Width, length, number of presents per idx
    let mut regions: Vec<(u32, u32, Vec<u32>)> = Vec::new();

    for region in input.trim().split("\n\n") {
        // Every present has colon, so extract boolean representation of present
        if region.chars().nth(1).unwrap() == ':' {
            let mut present: Vec<bool> = Vec::new();
            for ch in region.chars() {
                match ch {
                    '#' => present.push(true),
                    '.' => present.push(false),
                    _ => {},
                }
            }
            assert!(present.len() == 9);
            presents.push(present);
        } else { // Extract width and length
            for line in region.lines() {

                let mut parts = line.split(':');
                let width_length = parts.next().unwrap();
                let all_indices = parts.next().unwrap();

                // Parse width and length
                let mut width_length_split = width_length.split('x');
                let width: u32 = width_length_split.next().unwrap().parse().unwrap();
                let length: u32 = width_length_split.next().unwrap().parse().unwrap();

                // Parse indices
                let mut present_indices: Vec<u32> = Vec::new();
                for index in all_indices.trim().split(' ') {
                    let n: u32 = index.parse().unwrap();
                    present_indices.push(n);
                }

                // Add to regions
                regions.push((width, length, present_indices));
            }
        }
    }

    // println!("Presents: {:?}", presents);
    // println!("Grids: {:?}", grids);

    let mut present_sizes: Vec<usize> = Vec::new();
    for present in presents {
        // println!("Present: {:?}", present);
        present_sizes.push(present.iter().filter(|&&b| b).count());
    }

    // println!("Present size: {:?}", present_sizes);

    let mut n_regions: u64 = 0;
    for (w, h, present_idxs) in regions.iter() {
        let mut has_solution = true;
        
        // Prune out regions that has not enough physical space
        let area: u32 = w * h;
        let mut piece_area: u32 = 0;
        for (present_size, idx) in present_sizes.iter().zip(present_idxs) {
            piece_area += *present_size as u32 * idx;
        }
        if piece_area > area {
            has_solution = false;
        } else {
            // println!("Piece area: {area} / {piece_area} ({:?}%)", (piece_area as f32 / area as f32 * 100.0).round());
        }

        if has_solution {
            n_regions += 1;
        }
    }

    Some(n_regions)
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
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&aoc::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
