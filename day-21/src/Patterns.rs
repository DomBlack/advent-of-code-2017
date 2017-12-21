use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::FromStr;

use grid::*;

pub struct Patterns {
    map: HashMap<Grid, Grid>
}

impl Patterns {
    pub fn apply(&self, grid: &Grid) -> Grid {
        self.map.get(grid).unwrap_or(grid).clone()
    }
}

impl Default for Patterns {
    fn default() -> Self {
        Patterns { map: HashMap::new() }
    }
}

impl FromStr for Patterns {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut patterns = Patterns::default();

        for line in s.trim().lines() {
            let mut parts: Vec<Grid> =
                line.trim()
                    .split(" => ")
                    .map(| grid | grid.parse().unwrap())
                    .collect();

            if parts.len() != 2 {
                return Err(format!("Invalid number of parts on line `{}`", line));
            }

            let input = parts.remove(0);
            let output = parts.remove(0);

            for pattern in input.variations() {
                patterns.map.insert(pattern.clone(), output.clone());
            }
        }

        Ok(patterns)
    }
}

impl Display for Patterns {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        for (input, output) in &self.map {
            writeln!(f, "{} => {}", input, output)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_default() {
        assert_eq!(
            Patterns::default().map.is_empty(),
            true
        );
    }

    #[test]
    fn test_patterns_parse() {
        const INPUT: &str = "../.# => ##./#../...
                             .#./..#/### => #..#/..../..../#..#";

        let patterns: Patterns = INPUT.parse().unwrap();
        assert_eq!(patterns.map.len(), 12);
    }
}
