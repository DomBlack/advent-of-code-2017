use std::str::FromStr;
use std::ops::AddAssign;
use std::fmt;
use std::num::ParseIntError;

/// A Vector
#[derive(PartialOrd, Ord, PartialEq, Eq, Clone)]
pub struct V3 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl V3 {
    pub fn new(x: i32, y: i32, z: i32) -> Self { V3 { x, y, z } }

    /// The manhattan distance from the origin for this vector
    pub fn manhattan(&self) -> i32 { self.x.abs() + self.y.abs() + self.z.abs() }
}

impl<'a> AddAssign<&'a V3> for V3 {
    fn add_assign(&mut self, rhs: &'a V3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl fmt::Debug for V3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<{}, {}, {}>", self.x, self.y, self.z)
    }
}

impl FromStr for V3 {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();

        if !s.starts_with("<") {
            Err(format!("Vector must start with a `<`. Got `{}`", s))
        } else if !s.ends_with(">") {
            Err(format!("Vector must end with a  `>`. Got `{}`", s))
        } else {
            let mut parts: Vec<Result<i32, ParseIntError>> =
                s[1 .. s.len() - 1 ]
                    .split(",")
                    .map(| p | p.trim().parse())
                    .collect();

            if parts.len() != 3 {
                Err(format!("Expected 3 parts to vector. Got {} parts from `{}`", parts.len(), s))
            } else {
                let x: Result<i32, ParseIntError> = parts.remove(0);
                let y: Result<i32, ParseIntError> = parts.remove(0);
                let z: Result<i32, ParseIntError> = parts.remove(0);

                x.and_then(
                    | x | y.and_then(
                        | y | z.map(
                            | z | V3::new(x, y, z)
                        )
                    )
                ).map_err( | err | format!("Unable to parse component of vector: {}", err))
            }
        }
    }
}

