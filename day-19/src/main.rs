#[cfg(not(test))]
fn main() {
    const INPUT: &str = include_str!("input.txt");

    let (letters_found, steps_taken) = run_maze(INPUT);

    println!("Part 1: {}", letters_found);
    println!("Part 2: {}", steps_taken);
}

/// A strut representing our position at any given time
struct Position {
    x: usize,
    y: usize,
}

/// The current direction of our travels within the maze
#[derive(PartialEq)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

/// Runs though the given maze input, looking for each letter we pass and how many steps we take
fn run_maze(input: &str) -> (String, u32) {
    let mut found_letters = Vec::new();
    let mut steps_taken = 0;

    // Parse the maze
    let maze: Vec<Vec<char>> = input.lines().map( | line | line.chars().collect()).collect();

    // Starting position
    let mut position = Position {
        x: maze[0].iter().enumerate().find(| &(_, &c) | c == '|').unwrap().0,
        y: 0,
    };
    let mut direction = Direction::DOWN;

    // Start travelling
    while travel(&maze, &mut found_letters, &mut position, &mut steps_taken, &direction) {
        // Work out the new direction
        if direction == Direction::DOWN || direction == Direction::UP {
            if get_tile(&maze, &Position { x: position.x - 1, y: position.y }) != ' ' {
                direction = Direction::LEFT;
            } else {
                direction = Direction::RIGHT;
            }
        } else {
            if get_tile(&maze, &Position { x: position.x, y: position.y - 1 }) != ' ' {
                direction = Direction::UP;
            } else {
                direction = Direction::DOWN;
            }
        }
    }

    (found_letters.iter().collect(), steps_taken)
}

/// overflow safe tile get
fn get_tile(maze: &Vec<Vec<char>>, position: &Position) -> char {
    maze.get(position.y).and_then( | row | row.get(position.x)).map_or(' ', | c | *c)
}

/// Travels through the maze in a given direction until it can not longer move in that direction
fn travel(maze: &Vec<Vec<char>>, found_letters: &mut Vec<char>, position: &mut Position, steps_taken: &mut u32, direction: &Direction) -> bool {
    loop {
        match *direction {
            Direction::UP    => position.y -= 1,
            Direction::DOWN  => position.y += 1,
            Direction::LEFT  => position.x -= 1,
            Direction::RIGHT => position.x += 1,
        }
        *steps_taken += 1;

        let current_letter: char = get_tile(&maze, &position);

        if current_letter == ' ' || current_letter == '+' {
            return current_letter == '+';
        } else if current_letter != '|' && current_letter != '-' {
            found_letters.push(current_letter)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str =
"     |
     |  +--+
     A  |  C
 F---|----E|--+
     |  |  |  D
     +B-+  +--+";

    #[test]
    fn test_run_maze() {
        let (letters_found, steps_taken) = run_maze(INPUT);

        assert_eq!(letters_found, "ABCDEF");
        assert_eq!(steps_taken, 38);
    }
}
