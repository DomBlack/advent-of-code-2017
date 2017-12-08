use std::fmt;
use std::str::FromStr;
use std::collections::HashMap;
use regex::Regex;

/// Possible Conditions a program has
#[derive(PartialEq)]
pub enum ComparisionOperator {
    EQ,
    NEQ,
    GT,
    GTE,
    LT,
    LTE,
}

/// A comparision check
#[derive(PartialEq)]
pub struct Comparision {
    pub register: String,
    pub op: ComparisionOperator,
    pub value: i32,
}


impl fmt::Debug for ComparisionOperator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ComparisionOperator::EQ => write!(f, "=="),
            ComparisionOperator::NEQ => write!(f, "!="),
            ComparisionOperator::GT => write!(f, ">"),
            ComparisionOperator::GTE => write!(f, ">="),
            ComparisionOperator::LT => write!(f, "<"),
            ComparisionOperator::LTE => write!(f, "<="),
        }
    }
}

impl FromStr for ComparisionOperator {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "==" => Ok(ComparisionOperator::EQ),
            "!=" => Ok(ComparisionOperator::NEQ),
            ">" => Ok(ComparisionOperator::GT),
            ">=" => Ok(ComparisionOperator::GTE),
            "<" => Ok(ComparisionOperator::LT),
            "<=" => Ok(ComparisionOperator::LTE),
            _ => Err(format!("Unknown Operator: {}", s)),
        }
    }
}

lazy_static! {
    static ref COMPARISION_REGEX: Regex = Regex::new(
    r"^([a-z]+) ([^\s]+) (-?[0-9]+)$"
    ).expect("Unable to compile regex");
}

impl FromStr for Comparision {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        for cap in COMPARISION_REGEX.captures_iter(s.trim()) {
            let op: ComparisionOperator = cap[2].parse().expect("Unable to parse op");
            let value: i32 = cap[3].parse().expect("Unable to parse value");

            return Ok(Comparision::new(&cap[1], op, value));
        }

        Err(format!("Unable to parse {}", s))
    }
}

impl fmt::Debug for Comparision {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {:?} {}", self.register, self.op, self.value)
    }
}


impl Comparision {
    pub fn new(register: &str, op: ComparisionOperator, value: i32) -> Self {
        Comparision { register: String::from(register), op, value }
    }

    pub fn check(&self, registers: &HashMap<String, i32>) -> bool {
        let register: i32 = *registers.get(&self.register).unwrap_or(&0);

        match self.op {
            ComparisionOperator::EQ  => register == self.value,
            ComparisionOperator::NEQ => register != self.value,
            ComparisionOperator::LT  => register < self.value,
            ComparisionOperator::LTE => register <= self.value,
            ComparisionOperator::GT  => register > self.value,
            ComparisionOperator::GTE => register >= self.value,
        }
    }
}
