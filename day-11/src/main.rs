#[cfg(not(test))]
fn main() {
    static INPUT: &str = include_str!("input.txt");

    println!("Part 1: {}", distance(INPUT));
}

/*
Grid Coordinates (which are flat and have constant movement)

```
   +------+          +------+
  /        \        /        \
 +  -1, 1   +------+   1, 2   +
  \        /        \        /
   +------+   0, 1   +------+
  /        \        /        \
 +  -1, 0   +------+   1, 1   +
  \        /        \        /
   +------+   0, 0   +------+
  /        \        /        \
 +  -1, -1  +------+   1, 0   +
  \        /        \        /
   +------+   0, -1  +------+
           \        /
            +------+
```
*/

/// Calculates the shortest number of steps to get back
fn distance(input: &str) -> i32 {
    let pos: (i32, i32) = input.trim().split(',').fold(
        (0, 0),
        | current, mov | {
            match mov {
                "n"  => (current.0,     current.1 + 1),
                "s"  => (current.0,     current.1 - 1),
                "ne" => (current.0 + 1, current.1 + 1),
                "sw" => (current.0 - 1, current.1 - 1),
                "nw" => (current.0 - 1, current.1),
                "se" => (current.0 + 1, current.1),
                _    => panic!("Unknown movement {}", mov),
            }
        }
    );

    // Because our grid is flat, it's simply the largest of these two
    pos.0.abs().max(pos.1.abs())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_distance() {
        assert_eq!(distance("ne,ne,ne"), 3, "ne,ne,ne");
        assert_eq!(distance("ne,ne,sw,sw"), 0, "ne,ne,sw,sw");
        assert_eq!(distance("ne,ne,s,s"), 2, "ne,ne,s,s");
        assert_eq!(distance("se,sw,se,sw,sw"), 3, "se,sw,se,sw,sw");
    }
}
