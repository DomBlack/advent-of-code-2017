extern crate utils;

use utils::*;

#[cfg(not(test))]
fn main() {
    const INPUT: &str = "212,254,178,237,2,0,1,54,167,92,117,125,255,61,159,164";

    println!("Part 1: {}", check_sum(0 .. 256, INPUT));
    println!("Part 2: {}", knot_hash::hash(INPUT));
}

/// Calculates a check sum for the given input
fn check_sum(range: std::ops::Range<u32>, input: &str) -> u32 {
    let hash_list = knot_hash::sparse_hash(range, 1, parse_as_numbers(input));
    hash_list[0] * hash_list[1]
}

/// Parses the string literally comma separated numbers
fn parse_as_numbers(input: &str) -> Vec<usize> {
    input.split(',').map(|s| s.parse().unwrap()).collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_check_sum() {
        assert_eq!(check_sum(0 .. 5, "3,4,1,5"), 12);
    }

    #[test]
    fn test_knot_hash() {
        assert_eq!(knot_hash::hash(""), "a2582a3a0e66e6e86e3812dcb672a272");
        assert_eq!(knot_hash::hash("AoC 2017"), "33efeb34ea91902bb2f59c9920caa6cd");
        assert_eq!(knot_hash::hash("1,2,3"), "3efbe78a8d82f29979031a4aa0b16a9d");
        assert_eq!(knot_hash::hash("1,2,4"), "63960835bcdc130f0b66d7ff4f6a5a8e");
    }
}
