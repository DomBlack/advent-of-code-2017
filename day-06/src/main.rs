#[cfg(not(test))]
const INPUT: &'static str = include_str!("input.txt");

#[cfg(not(test))]
fn main() {
    println!("Part 1: {}", how_many_cycles(INPUT, false));
    println!("Part 1: {}", how_many_cycles(INPUT, true));
}

fn how_many_cycles(input: &str, return_cycle_size: bool) -> u32 {
    use std::collections::HashMap;

    let mut banks: Vec<u32> = input.split_whitespace()
            .map( | num | num.parse().expect("Invalid"))
            .collect();

    let mut states_seen = HashMap::new();


    let mut counter = 0;

    while states_seen.contains_key(&banks) == false {
        states_seen.insert(banks.clone(), counter);

        counter = counter + 1;

        let max_index = max_by_index(&banks);
        let mut left_to_split = banks[max_index];
        banks[max_index] = 0;

        let mut indexes =
            (0 .. banks.len()).cycle().skip(max_index + 1);

        while left_to_split > 0 {
            let index = indexes.next().unwrap();
            banks[index] += 1;
            left_to_split -= 1;
        }
    }

    if return_cycle_size {
        counter - states_seen.get(&banks).unwrap()
    } else {
        counter
    }
}

/// Returns the index of the maximum value found (if multiple then the first one)
fn max_by_index(banks: &Vec<u32>) -> usize {
    let mut max_value = 0;
    let mut max_index = 0;

    for index in 0 .. banks.len() {
        if banks[index] > max_value {
            max_value = banks[index];
            max_index = index;
        }
    }

    max_index
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_max_by_index() {
        assert_eq!(max_by_index(&vec![ 0, 3, 1, 2, 3]), 1);
    }

    #[test]
    fn part1_example() {
        assert_eq!(
            how_many_cycles("0 2 7 0", false),
            5
        )
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            how_many_cycles("0 2 7 0", true),
            4
        )
    }
}
