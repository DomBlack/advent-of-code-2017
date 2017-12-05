#[cfg(not(test))]
const INPUT: &'static str = include_str!("input.txt");

#[cfg(not(test))]
fn main() {
    println!("Part 1: {}", no_of_jumps(INPUT, &increment));
    println!("Part 2: {}", no_of_jumps(INPUT, &increment_or_decrement));
}

/// Loops over the given input of program offsets, and counts how many steps until it leaves the
/// program (applying the `modifier` to change each offset as it visits it)
///
/// # Example
///
/// Input Data:
/// ```
/// 0
/// 3
/// 0
/// 1
/// -3
/// ```
///
/// Modifier: `| x | x + 1` (increments by 1)
///
/// `(0) 3  0  1  -3` - before we have taken any steps.
/// `(1) 3  0  1  -3` - jump with offset `0` (that is, don't jump at all).
///                     Fortunately, the instruction is then incremented to `1`.
/// `2 (3) 0  1  -3`  - step forward because of the instruction we just modified.
///                     The first instruction is incremented again, now to `2`.
/// `2  4  0  1 (-3)` - jump all the way to the end; leave a `4` behind.
/// `2 (4) 0  1  -2`  - go back to where we just were; increment `-3` to `-2`.
/// `2  5  0  1  -2`  - jump `4` steps forward, escaping the maze.
fn no_of_jumps(input: &str, modifier: &Fn(i32) -> i32) -> i32 {
    let mut list: Vec<i32> =
        input
            .split_whitespace().map(|i| i.parse().expect("Unable to parse input"))
            .collect();

    let mut ptr = 0;
    let mut steps = 0;

    loop {
        match list.get_mut(ptr) {
            Some(offset) => {
                steps += 1;
                let new_ptr = ptr as i32 + *offset;
                *offset = modifier(*offset);

                if new_ptr < 0 {
                    break;
                } else {
                    ptr = new_ptr as usize;
                }
            }
            None => break,
        }
    }

    steps
}

/// Increments the given input by 1
fn increment(input: i32) -> i32 {
    input + 1
}

/// Increments the given input by 1 if < 3, otherwise decrements by 1
fn increment_or_decrement(input: i32) -> i32 {
    if input >= 3 {
        input - 1
    } else {
        input + 1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(
            no_of_jumps("0\n3\n0\n1\n-3", &increment),
            5
        )
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            no_of_jumps("0\n3\n0\n1\n-3", &increment_or_decrement),
            10
        )
    }
}
