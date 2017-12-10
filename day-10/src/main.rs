#[cfg(not(test))]
fn main() {
    const INPUT: &str = "212,254,178,237,2,0,1,54,167,92,117,125,255,61,159,164";

    println!("Part 1: {}", check_sum(0 .. 256, INPUT));
    println!("Part 2: {}", knot_hash(INPUT));
}

/// Calculates a check sum for the given input
fn check_sum(range: std::ops::Range<u32>, input: &str) -> u32 {
    let hash_list = compute_sparse_hash(range, 1, parse_as_numbers(input));
    hash_list[0] * hash_list[1]
}

/// Calculates a Knot Hash
fn knot_hash(input: &str) -> String {
    use std::u32;

    let sparse_hash: &[u32] = &compute_sparse_hash(0 .. 256, 64, parse_as_chars(input));

    // Each 16 bytes gets converted using XOR into a single byte and then output as a hex string
    sparse_hash
        .chunks(16)
        .map( | chunk | {
            let dense = chunk.iter().fold(0 as u32, | sum, i | sum ^ *i );

            format!("{:02x}", dense)
        })
        .collect()
}

/// Computes a spare hash for the given input
fn compute_sparse_hash(
    range: std::ops::Range<u32>,
    no_rounds: u8,
    input: Vec<usize>
) -> Vec<u32> {
    let mut hash_list: Vec<u32> = range.collect();
    let len = hash_list.len();

    let mut position = 0;
    let mut skip_size = 0;

    for _ in 0 .. no_rounds {
        for sub_list_size in &input {
            // Reverse the sub list, wrapping around
            for j in 0..(sub_list_size / 2) {
                hash_list.swap((position + j) % len, ((position + sub_list_size) - j - 1) % len);
            }

            position = (position + sub_list_size + skip_size) % len;
            skip_size += 1;
        }
    }

    hash_list
}

/// Parses the string literally comma separated numbers
fn parse_as_numbers(input: &str) -> Vec<usize> {
    input.split(',').map(|s| s.parse().unwrap()).collect()
}

/// Parses the string as ASCII characters and then appends a hard coded suffix
fn parse_as_chars(input: &str) -> Vec<usize> {
    let mut chars: Vec<usize> = input.chars().map(|c| c as usize).collect();
    chars.append(&mut vec![ 17, 31, 73, 47, 23 ]);

    chars
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
        assert_eq!(knot_hash(""), "a2582a3a0e66e6e86e3812dcb672a272");
        assert_eq!(knot_hash("AoC 2017"), "33efeb34ea91902bb2f59c9920caa6cd");
        assert_eq!(knot_hash("1,2,3"), "3efbe78a8d82f29979031a4aa0b16a9d");
        assert_eq!(knot_hash("1,2,4"), "63960835bcdc130f0b66d7ff4f6a5a8e");
    }
}