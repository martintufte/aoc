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

    // TODO: Pre-compute the rotated and flipped presents

    // TODO: Find the number of regions that can fit the presents
    let mut n_regions: u64 = 0;
    for (idx, (w, h, _present_idxs)) in regions.iter().enumerate() {
        println!("Region {idx} ({w} x {h}):");
        let mut has_solution = false;

        // TODO: Put in dynamic programming solution
        for _ in 0..1 {
            has_solution = true;
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
