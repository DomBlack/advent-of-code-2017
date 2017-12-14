extern crate utils;
use utils::*;

#[cfg(not(test))]
fn main() {
    println!("Part 1: {}", number_of_squares_used("oundnydw"));
    println!("Part 2: {}", number_of_regions("oundnydw"));
}

/// Creates a row from a given input and the row number
fn create_row(input: &str, row_no: usize) -> [bool; 128] {
    let mut row = [false; 128];

    // Hash the input
    let hash = knot_hash::hash(format!("{}-{}", input, row_no).as_str());

    // Update the row array
    let mut counter = 0;
    hash.chars().for_each( | char | {
        let number = char.to_digit(16).expect("Unable to parse char");

        if number & 8 == 8 { row[counter] = true; }
        if number & 4 == 4 { row[counter + 1] = true; }
        if number & 2 == 2 { row[counter + 2] = true; }
        if number & 1 == 1 { row[counter + 3] = true; }

        counter = counter + 4;
    });

    // Return the row
    row
}

/// Creates the full grid given an input
fn create_grid(input: &str) -> [[bool; 128]; 128] {
    let mut grid = [[false; 128]; 128];

    (0 .. 128).for_each(| row_no | grid[row_no] = create_row(input, row_no));

    grid
}

/// Counts the number of squares used within the 128x128 grid
fn number_of_squares_used(input: &str) -> u32 {
    (0 .. 128)
        .map(| row_no | {
            create_row(input, row_no)
                .iter().fold(
                0,
                | accum, cell | if *cell { accum + 1 } else { accum }
            )
        })
        .sum()
}

// Modifies the grid and un sets any set neighbors
fn clear_neighbours(grid: &mut [[bool; 128]; 128], x: usize, y: usize) {
    grid[x][y] = false;

    // Check left
    if x > 0 && grid[x-1][y] {
        clear_neighbours(grid, x - 1, y);
    }

    // Check right
    if x < 127 && grid[x+1][y] {
        clear_neighbours(grid, x + 1, y);
    }

    // Clear up
    if y > 0 && grid[x][y - 1] {
        clear_neighbours(grid, x, y - 1);
    }

    // Clear down
    if y < 127 && grid[x][y + 1] {
        clear_neighbours(grid, x, y + 1);
    }
}

/// Counts the number of regions used within the 128x128 grid
/// Where a region is a group of used squares all adjacent (not diagonal)
fn number_of_regions(input: &str) -> u32 {
    let mut grid = create_grid(input);

    let mut count = 0;

    for x in 0 .. 128 {
        for y in 0 .. 128 {
            if grid[x][y] {
                count = count + 1;
                clear_neighbours(&mut grid, x, y);
            }
        }
    }

    count
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_number_squares_used() {
        assert_eq!(
            number_of_squares_used("flqrgnkx"),
            8108
        );
    }

    #[test]
    fn test_number_of_regions() {
        assert_eq!(
            number_of_regions("flqrgnkx"),
            1242
        );
    }
}
