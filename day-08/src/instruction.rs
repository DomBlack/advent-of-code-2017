use std::str::FromStr;
use std::collections::HashMap;
use regex::Regex;

use comparison::*;

/// Possible operations a program has
#[derive(Debug, PartialEq)]
pub enum Operation {
    INC,
    DEC,
}

impl FromStr for Operation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "inc" => Ok(Operation::INC),
            "dec" => Ok(Operation::DEC),
            _     => Err(String::from(s)),
        }
    }
}

/// An Instruction
#[derive(Debug, PartialEq)]
pub struct Instruction {
    pub register: String,
    pub op: Operation,
    pub value: i32,
    pub condition: Comparision,
}

lazy_static! {
    static ref INSTRUCTION_REGEX: Regex = Regex::new(
    r"^([a-z]+) ([^\s]+) (-?[0-9]+) if (.*)$"
    ).expect("Unable to compile regex");
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        for cap in INSTRUCTION_REGEX.captures_iter(s.trim()) {
            let op: Operation = cap[2].parse().expect("Unable to parse op");
            let value: i32 = cap[3].parse().expect("Unable to parse value");
            let condition: Comparision = cap[4].parse().expect("Unable to parse comparision");

            return Ok(
                Instruction::new(&cap[1], op, value, condition)
            );
        }

        Err(())
    }
}

impl Instruction {
    pub fn new(register: &str, op: Operation, value: i32, condition: Comparision) -> Self {
        Instruction {
            register: String::from(register),
            op, value, condition
        }
    }

    pub fn execute(&self, registers: &mut HashMap<String, i32>, max_values: &mut HashMap<String, i32>) {
        if self.condition.check(registers) {
            let register = *registers.get(&self.register).unwrap_or(&0);

            let new_value = match self.op {
                Operation::INC => register + self.value,
                Operation::DEC => register - self.value,
            };

            let new_max = match max_values.get(&self.register) {
                Some(value) => new_value.max(*value),
                None        => new_value,
            };

            registers.insert(self.register.clone(), new_value);
            max_values.insert(self.register.clone(), new_max);
        }
    }
}
