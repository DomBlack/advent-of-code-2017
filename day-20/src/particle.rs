use std::str::FromStr;
use regex::Regex;

use vectors::V3;

/// A Particle
#[derive(Debug, PartialEq, Clone)]
pub struct Particle {
    pub position: V3,
    pub velocity: V3,
    pub acceleration: V3,
}

impl Particle {
    pub fn step(&mut self) {
        self.velocity += &self.acceleration;
        self.position += &self.velocity;
    }
}

lazy_static! {
    static ref PARTICLE_REGEX: Regex = Regex::new(
    r"^p=(<\s*-?\d+,\s*-?\d+,\s*-?\d+>),\s*v=(<\s*-?\d+,\s*-?\d+,\s*-?\d+>),\s*a=(<\s*-?\d+,\s*-?\d+,\s*-?\d+>)$"
    ).expect("Unable to compile regex");
}

impl FromStr for Particle {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        for cap in PARTICLE_REGEX.captures_iter(s.trim()) {
            // Attempt to parse the three vectors
            let position = cap[1].parse();
            let velocity = cap[2].parse();
            let acceleration = cap[3].parse();

            // And merge all results into a single Particle
            return position.and_then(
                | position | velocity.and_then(
                    | velocity | acceleration.map(
                        | acceleration | Particle { position, velocity, acceleration }
                    )
                )
            )
        }

        Err(format!("Particle was not detected in: `{}`", s))
    }
}
