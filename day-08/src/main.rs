#[macro_use]
extern crate lazy_static;

extern crate regex;

use std::collections::HashMap;

mod instruction;
mod comparison;

use instruction::*;
use comparison::*;

#[cfg(not(test))]
const INPUT: &'static str = include_str!("input.txt");

#[cfg(not(test))]
fn main() {
    let mut map = HashMap::new();
    let mut highest = HashMap::new();

    INPUT.lines()
        .for_each(
            | line | line.parse::<Instruction>().unwrap()
                .execute(&mut map, &mut highest)
        );

    {
        let (register, value) = map.iter().max_by_key(|t| t.1).unwrap();
        println!("Part 1: Register {} has the highest value {}", register, value);
    }

    {
        let (register, value) = highest.iter().max_by_key(|t| t.1).unwrap();
        println!("Part 2: Register {} had the highest value {}", register, value);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &'static str =
        "b inc 5 if a > 1
        a inc 1 if b < 5
        c dec -10 if a >= 1
        c inc -20 if c == 10";

    #[test]
    fn test_comparision_from_string() {
        assert_eq!(
            "abc <= -32".parse::<Comparision>().unwrap(),
            Comparision::new("abc", ComparisionOperator::LTE, -32)
        );

        assert_eq!(
            "fdsf == 123".parse::<Comparision>().unwrap(),
            Comparision::new("fdsf", ComparisionOperator::EQ, 123)
        );
    }

    #[test]
    fn test_instruction_from_str() {
        assert_eq!(
            "b dec 30 if cat < 1".parse::<Instruction>().unwrap(),
            Instruction::new(
                "b", Operation::DEC, 30,
                Comparision::new("cat", ComparisionOperator::LT, 1)
            )
        );

        assert_eq!(
            "x inc -30 if dog > -30".parse::<Instruction>().unwrap(),
            Instruction::new(
                "x", Operation::INC, -30,
                Comparision::new("dog", ComparisionOperator::GT, -30)
            )
        );
    }

    #[test]
    fn test_example() {
        let mut map = HashMap::new();
        let mut highest = HashMap::new();

        TEST_INPUT
            .lines()
            .for_each(
                | line | line.parse::<Instruction>().unwrap()
                    .execute(&mut map, &mut highest)
            );

        {
            let (register, value) = map.iter().max_by_key(|t| t.1).unwrap();
            assert_eq!(1, *value, "Max Register Was Wrong {}", register);
        }

        {
            let (register, value) = highest.iter().max_by_key(|t| t.1).unwrap();
            assert_eq!(10, *value, "Max Register During Process Wrong {}", register);
        }
    }
}
