#[cfg(not(test))]
fn main() {
    const A_SEED: u32 = 618;
    const B_SEED: u32 = 814;

    println!(
        "Part 1: {}",
         matching_lower_16(
             40_000_000,
             Generator::new_a(A_SEED, false),
             Generator::new_b(B_SEED, false)
         )
    );

    println!("Part 2: {}",
         matching_lower_16(
             5_000_000,
             Generator::new_a(A_SEED, true),
             Generator::new_b(B_SEED, true)
         )
    );
}

/// Generator Object
struct Generator {
    /// The factor
    factor: u64,

    /// The previous seed
    previous: u64,

    // All results must be a multiple of
    multiple_of: u64,
}

impl Generator {
    fn new(factor: u64, seed: u32, multiple_of: u64) -> Self { Generator { factor, previous: seed as u64, multiple_of } }
    fn new_a(seed: u32, use_multiple: bool) -> Self { Generator::new(16807, seed, if use_multiple { 4 } else { 1 }) }
    fn new_b(seed: u32, use_multiple: bool) -> Self { Generator::new(48271, seed, if use_multiple { 8 } else { 1 }) }
}

/// The actual implementation of the generator
impl Iterator for Generator {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        // This is like a "do {} while {}"
        while {
            self.previous = (self.previous * self.factor) % 2147483647;
            (self.previous % self.multiple_of) != 0
        } {}

        Some(self.previous as u32)
    }
}

/// Returns the number of matching lower 16 bits in the given `range`
fn matching_lower_16(range: usize, a: Generator, b: Generator) -> usize {
    let mask: u32 = (2 as u32).pow(16) - 1;

    a.zip(b)
        .take(range)
        .filter(|&(a, b)| a & mask == b & mask)
        .count()
}

#[cfg(test)]
mod test {
    const A_SEED: u32 = 65;
    const B_SEED: u32 = 8921;

    use super::*;

    #[test]
    fn test_generator() {
        Generator::new_a(A_SEED, false)
            .zip(vec![1092455, 1181022009, 245556042, 1744312007, 1352636452])
            .for_each(|(generated, expected)| assert_eq!(expected, generated, "Generator A"));

        Generator::new_b(B_SEED, false)
            .zip(vec![430625591, 1233683848, 1431495498, 137874439, 285222916])
            .for_each(|(generated, expected)| assert_eq!(expected, generated, "Generator B"));
    }

    #[test]
    fn test_generator_with_multiple_of() {
        Generator::new_a(A_SEED, true)
            .zip(vec![1352636452, 1992081072, 530830436, 1980017072, 740335192])
            .for_each(|(generated, expected)| assert_eq!(expected, generated, "Generator A"));

        Generator::new_b(B_SEED, true)
            .zip(vec![1233683848, 862516352, 1159784568, 1616057672, 412269392])
            .for_each(|(generated, expected)| assert_eq!(expected, generated, "Generator B"));
    }

    #[test]
    fn test_part_1() {
        assert_eq!(
            matching_lower_16(
                40_000_000,
                Generator::new_a(A_SEED, false),
                Generator::new_b(B_SEED, false)
            ),
            588
        );
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            matching_lower_16(
                5_000_000,
                Generator::new_a(A_SEED, true),
                Generator::new_b(B_SEED, true)
            ),
            309
        );
    }
}
