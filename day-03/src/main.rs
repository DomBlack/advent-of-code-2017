extern crate utils;

mod lib;
use lib::*;

use std::collections::HashMap;

#[cfg(not(test))]
fn main() {
    let input: i32 = utils::read_input();

    println!("Part 1: {} ", distance(input));
    println!("Part 2: {} ", sum(input));
}

/// Finds the first value larger than the given `input`
///
/// # Example
///
/// ```
/// 147  142  133  122   59
/// 304    5    4    2   57
/// 330   10    1    1   54
/// 351   11   23   25   26
/// 362  747  806--->   ...
/// ```
///
/// Input of `5` will return `10`
fn sum(input: i32) -> i32 {
    let mut position = Position::default();
    let mut direction = Direction::default();
    let mut grid_size= 0;

    let mut grid = HashMap::new();
    grid.insert(position.clone(), 1);

    loop {
        position = position.mov(&mut direction, &mut grid_size);

        let sum = position.neighbours().iter()
            .map(| pos | grid.get(pos))
            .filter( | opt | opt.is_some() )
            .map( | opt | opt.unwrap() )
            .sum();

        if sum > input {
            return sum;
        } else {
            grid.insert(position.clone(), sum);
        }
    }
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
    use super::sum;

    #[test]
    fn part1_examples() {
        assert_eq!(distance(1),    0,  "Input 1");
        assert_eq!(distance(12),   3,  "Input 12");
        assert_eq!(distance(23),   2,  "Input 23");
        assert_eq!(distance(1024), 31, "Input 1024");
    }

    #[test]
    fn part2() {
        assert_eq!(sum(4),   5,   "Input 4");
        assert_eq!(sum(5),   10,  "Input 5");
        assert_eq!(sum(747), 806, "Input 747");
    }
}