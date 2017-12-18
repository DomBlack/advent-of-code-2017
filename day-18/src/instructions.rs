use std::str::FromStr;
use std::collections::HashMap;

/// Parses an input string into a vector of instructions
pub fn parse(s: &str) -> Vec<Instruction> {
    s.trim().lines().map(| l | l.parse().unwrap()).collect()
}

/// A register name
pub type RegisterName = char;

/// The registers collection
pub type Registers = HashMap<RegisterName, i64>;

/// Gets the register value or it's default of `0`
pub fn get_register(register: RegisterName, registers: &Registers) -> i64 {
    registers.get(&register).map( | i | *i).unwrap_or(0)
}

/// Parses a string into a register name
fn parse_register_name(input: &str) -> Result<RegisterName, String> {
    if input.trim().len() != 1 {
        Err(format!("Expected only one character, got input `{}`", input))
    } else {
        Ok(input.trim().chars().next().unwrap())
    }
}

/// A possible value which is either a register name or a raw value
#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Register(RegisterName),
    Value(i64),
}

impl FromStr for Value {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(v) = s.trim().parse().map(Value::Value) {
            Ok(v)
        } else {
            parse_register_name(s).map(Value::Register)
        }
    }
}

impl Value {
    pub fn get(self: &Self, registers: &Registers) -> i64 {
        match self {
            &Value::Register(register) => get_register(register, registers),
            &Value::Value(value)       => value,
        }
    }
}

/// The supported instructions
#[derive(Clone, Debug, PartialEq)]
pub enum Instruction {
    Send(Value),
    Receive(RegisterName),
    Set(RegisterName, Value),
    Add(RegisterName, Value),
    Multiply(RegisterName, Value),
    Modulus(RegisterName, Value),
    Jump(Value, Value),
}

fn parse_register_value_pair(r: &str, v: &str, f: &Fn(RegisterName, Value) -> Instruction) -> Result<Instruction, String> {
    parse_register_name(r).and_then(| r | { Value::from_str(v).map( | v | f(r, v)) })
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.trim().split_whitespace().collect();

        if parts.len() < 2 {
            Err(format!("Expected at least two parts to instruction: `{}`", s))
        } else {
            match parts[0] {
                "snd" => Value::from_str(parts[1]).map(Instruction::Send),
                "rcv" => parse_register_name(parts[1]).map(Instruction::Receive),
                _     => {
                    if parts.len() != 3 {
                        Err(format!("Expected three parts for any instruction which isn't snd or rcv, got `{}`", s))
                    } else {
                        match parts[0] {
                            "set" => parse_register_value_pair(parts[1], parts[2], &Instruction::Set),
                            "add" => parse_register_value_pair(parts[1], parts[2], &Instruction::Add),
                            "mul" => parse_register_value_pair(parts[1], parts[2], &Instruction::Multiply),
                            "mod" => parse_register_value_pair(parts[1], parts[2], &Instruction::Modulus),
                            "jgz" => Value::from_str(parts[1]).and_then( | v1 | Value::from_str(parts[2]).map( | v2 | Instruction::Jump(v1, v2))),
                            _ => Err(format!("Unknown op code `{}`", parts[0])),
                        }
                    }
                }
            }
        }
    }
}
