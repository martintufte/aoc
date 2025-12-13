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

pub fn part_two(_input: &str) -> Option<u64> {
    None
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
        assert_eq!(result, None);
    }
}
