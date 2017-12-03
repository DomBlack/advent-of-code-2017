extern crate utils;

#[cfg(not(test))]
fn main() {
    let input: i32 = utils::read_input();

    println!("Part 1: {} ", distance(input));
}

/// Calculates the grid distance needed to travel in an
/// [Ulam Spiral](https://en.wikipedia.org/wiki/Ulam_spiral)
///
/// # Example
///
/// ```
/// 17  16  15  14  13
/// 18   5   4   3  12
/// 19   6   1   2  11
/// 20   7   8   9  10
/// 21  22  23---> ..
/// ```
///
/// The distance to travel to the centre from `22` is `Up 2`, `Right 1` which is a distance of `3`
fn distance(n: i32) -> i32 {
    let k  = (((n as f32).sqrt() - 1.) / 2.).ceil() as i32;
    let t = 2 * k + 1;
    let mut m  = t.pow(2);
    let t  = t - 1;

    if n >= m - t { return (k - (m - n)).abs() + (-k).abs() } else { m = m -t }
    if n >= m - t { return (-k).abs() + (-k + (m - n)).abs() } else { m = m -t }
    if n >= m - t { return (-k + (m - n)).abs() + (k).abs() } else { return (k).abs() + (k - (m - n - t)).abs() }
}


// Conditionally compile the module `test` only when the test-suite is run.
#[cfg(test)]
mod test {
    use super::distance;

    #[test]
    fn part1_examples() {
        assert_eq!(distance(1),    0,  "Input 1");
        assert_eq!(distance(12),   3,  "Input 12");
        assert_eq!(distance(23),   2,  "Input 23");
        assert_eq!(distance(1024), 31, "Input 1024");
    }
}