#[cfg(not(test))]
fn main() {
    static INPUT: &str = include_str!("input.txt");

    let (distance_now, max_distance) = distance(INPUT);
    println!("Part 1: {}", distance_now);
    println!("Part 2: {}", max_distance);
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
fn distance(input: &str) -> (i32, i32) {
    let (max_distance, pos) = input.trim().split(',').fold(
        (0, (0, 0)),
        | (max_distance, current), mov | {
            let new_position: (i32, i32) = match mov {
                "n"  => (current.0,     current.1 + 1),
                "s"  => (current.0,     current.1 - 1),
                "ne" => (current.0 + 1, current.1 + 1),
                "sw" => (current.0 - 1, current.1 - 1),
                "nw" => (current.0 - 1, current.1),
                "se" => (current.0 + 1, current.1),
                _    => panic!("Unknown movement {}", mov),
            };

            let new_position_distance = new_position.0.abs().max(new_position.1.abs());

            (max_distance.max(new_position_distance), new_position)
        }
    );

    // Because our grid is flat, it's simply the largest of these two
    let distance_now = pos.0.abs().max(pos.1.abs());
    (distance_now, max_distance)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_distance() {
        assert_eq!(distance("ne,ne,ne").0, 3, "ne,ne,ne");
        assert_eq!(distance("ne,ne,sw,sw").0, 0, "ne,ne,sw,sw");
        assert_eq!(distance("ne,ne,s,s").0, 2, "ne,ne,s,s");
        assert_eq!(distance("se,sw,se,sw,sw").0, 3, "se,sw,se,sw,sw");
    }
}
