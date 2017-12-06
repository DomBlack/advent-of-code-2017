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
        .map(|num| num.parse().expect("Invalid"))
        .collect();

    let mut states_seen = HashMap::new();


    let mut counter = 0;
    let len = banks.len();

    while states_seen.contains_key(&banks) == false {
        states_seen.insert(banks.clone(), counter);

        counter = counter + 1;

        if let Some((max_index, &max_value)) =
            banks.iter().enumerate().rev()
            .max_by_key(|&(_, val)| val) {

            banks[max_index] = 0;

            (0..len).cycle()
                .skip(max_index + 1).take(max_value as usize)
                .for_each(|i| banks[i] += 1);
        }
    }

    if return_cycle_size {
        counter - states_seen.get(&banks).unwrap()
    } else {
        counter
    }
}


#[cfg(test)]
mod test {
    use super::*;

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
