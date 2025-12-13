use std::collections::BTreeMap;
aoc::solution!(5);

pub fn part_one(input: &str) -> Option<u64> {
    // Separate input into ranges and items to check
    let mut lower_vec: Vec<u64> = Vec::new();
    let mut upper_vec: Vec<u64> = Vec::new();

    for line in input.trim().split('\n') {
        if line.len() == 0 {
            break
        }
        let (start, end) = line
            .split_once('-')
            .map(|(a, b)| (a.parse::<u64>().unwrap(), b.parse::<u64>().unwrap()))
            .unwrap();
        lower_vec.push(start);
        upper_vec.push(end);
    }

    // Length of ranges is start of items to check minus 1
    let n_range = lower_vec.len();

    // Iterate over items and count the number of spoiled
    let mut n_fresh: u64 = 0;
    
    for line in input.trim().split('\n').skip(n_range + 1) {
        let k: u64 = line.parse().unwrap();
        let mut is_fresh: bool = false;

        for (lower, upper) in lower_vec.iter().zip(upper_vec.iter()) {
            if *lower <= k && k <= *upper {  // is in interval, so item is fresh
                is_fresh = true;
                break;
            }
        }

        // println!("Item: {k}, Fresh: {is_fresh}");
        if is_fresh {
            n_fresh += 1;
        }
    }

    Some(n_fresh)
}

pub fn part_two(input: &str) -> Option<u64> {
    // Separate input into ranges
    let mut lower_vec: Vec<u64> = Vec::new();
    let mut upper_vec: Vec<u64> = Vec::new();

    for line in input.trim().split('\n') {
        if line.len() == 0 {
            break
        }
        let (start, end) = line
            .split_once('-')
            .map(|(a, b)| (a.parse::<u64>().unwrap(), b.parse::<u64>().unwrap()))
            .unwrap();
        lower_vec.push(start);
        upper_vec.push(end);
    }

    // Keep track of number of openings, closures of intervals,
    // so iterating over them to count the number of items
    let mut table: BTreeMap<u64, (u32, u32)> = BTreeMap::new();
    for (lower, upper) in lower_vec.iter().zip(upper_vec.iter()) {
        match table.get_mut(lower) {
            Some(value) => {*value = (value.0 + 1, value.1);},
            None => {table.insert(*lower, (1, 0));},
        };
        match table.get_mut(upper) {
            Some(value) => {*value = (value.0, value.1 + 1);},
            None => {table.insert(*upper, (0, 1));},
        };
    }

    // Iterate over the table, and count size of regions with open intervals
    let mut count: u64 = 0;
    let mut openness: u32 = 0;
    let mut open_at: Option<u64> = None;
    for (k, v) in table {        
        // Add lower openings
        openness += v.0;

        // Start new open range, set open_at
        if openness == v.0 && v.0 > 0 {
            open_at = Some(k);
        }

        // Subtract upper openings
        openness -= v.1;
        if openness == 0 && open_at.is_some() { // Add length of open interval
            // println!("Found interval ({:?}-{k})", open_at.unwrap());
            count += k - open_at.unwrap() + 1;
            open_at = None;
        }
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
        assert_eq!(result, Some(14));
    }
}
