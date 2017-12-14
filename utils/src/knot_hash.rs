extern crate std;

/// Calculates a Knot Hash
pub fn hash(input: &str) -> String {
    use std::u32;

    let sparse_hash: &[u32] = &sparse_hash(0 .. 256, 64, parse_as_chars(input));

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
pub fn sparse_hash(
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

/// Parses the string as ASCII characters and then appends a hard coded suffix
fn parse_as_chars(input: &str) -> Vec<usize> {
    let mut chars: Vec<usize> = input.chars().map(|c| c as usize).collect();
    chars.append(&mut vec![ 17, 31, 73, 47, 23 ]);

    chars
}
