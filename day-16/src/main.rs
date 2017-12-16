#[cfg(not(test))]
fn main() {
    const INPUT: &str = include_str!("input.txt");

    let instructions = parse(INPUT);
    let original_order = "abcdefghijklmnop";

    println!(
        "Part 1: {}",
        run_instructions(original_order, &instructions, 1)
    );

    println!(
        "Part 2: {}",
        run_instructions(original_order, &instructions, 1_000_000_000)
    );
}

pub enum Instruction {
    Spin(usize),
    Exchange(usize, usize),
    Partner(char, char),
}

/// Parse the input into a vector of instructions to execute
fn parse(input: &str) -> Vec<Instruction> {
    input.trim().split(',').map( | instruction: &str | {
        match &instruction[0 .. 1] {
            "s" => Instruction::Spin(instruction[1..].parse().unwrap()),
            "x" => {
                let (a, b) = get_parts_of_instruction::<usize>(instruction);
                Instruction::Exchange(a, b)
            },
            "p" => {
                let (a, b) = get_parts_of_instruction::<char>(instruction);
                Instruction::Partner(a, b)
            },
            _   => panic!("Unknown instruction {}", instruction),
        }
    }).collect()
}

/// Runs the given `programs` through the `instructions` `limit` times
fn run_instructions(programs: &str, instructions: &Vec<Instruction>, limit: u32) -> String {
    use std::iter::FromIterator;
    use std::collections::HashMap;

    let mut programs: Vec<char> = programs.trim().chars().collect();

    let mut known = HashMap::new();

    // Start looping
    for loop_no in 0 .. limit {
        // Run the full set of instructions
        for i in instructions {
            match *i {
                Instruction::Spin(number) => {
                    for _ in 0..number {
                        let c = programs.pop().unwrap();
                        programs.insert(0, c);
                    }
                },
                Instruction::Exchange(a, b) => {
                    programs.swap(a, b);
                },
                Instruction::Partner(a, b) => {
                    let a_i = programs.iter().position(|p| *p == a).unwrap();
                    let b_i = programs.iter().position(|p| *p == b).unwrap();
                    programs.swap(a_i, b_i);
                },
            }
        }

        // Look for a cycle
        let order = String::from_iter(&programs);
        match known.insert(order.clone(), loop_no) {
            None => (),
            Some(cycle_beginning) => {
                // Now we know the cycle, we can skip looping and just get the answer at
                // "limit"
                let cycle_len = loop_no - cycle_beginning;
                let remaining = (limit - loop_no - 1) % cycle_len;
                let target_index = cycle_beginning + remaining;

                let (k, _) = known.iter()
                    .find( | &(_, &v) | v == target_index ).unwrap();

                return (*k).clone();
            },
        };
    }

    // Return our output string
    String::from_iter(programs)
}

/// Converts a string like `a/b` into a tuple `(a, b)` with type `T`
fn get_parts_of_instruction<T : std::str::FromStr>(instruction: &str) -> (T, T)
    where <T as std::str::FromStr>::Err: std::fmt::Debug
{
    let mut parts = instruction[1..].split("/").map(| p | p.parse().unwrap());

    let a: T = parts.next().unwrap();
    let b: T = parts.next().unwrap();
    (a, b)
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "s1,x3/4,pe/b";

    #[test]
    fn test_spin() {
        assert_eq!(
            run_instructions(
                "abcde",
                &vec![Instruction::Spin(3)],
                1
            ),
            "cdeab"
        );
    }

    #[test]
    fn test_example() {
        let instructions = parse(INPUT);

        assert_eq!(
            run_instructions("abcde", &instructions, 1),
            "baedc"
        );

        assert_eq!(
            run_instructions("abcde", &instructions, 2),
            "ceadb"
        );
    }

    #[test]
    fn test_cycle_code() {
        let instructions = parse(INPUT);

        assert_eq!(
            run_instructions("abcde", &instructions, 1_234),
            "ceadb"
        );
    }
}