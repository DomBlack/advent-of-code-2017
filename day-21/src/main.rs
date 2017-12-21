mod grid;
mod patterns;

use grid::*;
use patterns::*;

#[cfg(not(test))]
fn main() {
    let patterns: Patterns = include_str!("input.txt").parse().unwrap();

    let grid = iterate(Grid::default(), 5, &patterns);
    println!("Part 1: {}", grid.number_of_on());

    let grid = iterate(grid, 13, &patterns);
    println!("Part 2: {}", grid.number_of_on());
}

/// Iterates the grid using the given set of patterns
fn iterate(grid: Grid, no: usize, patterns: &Patterns) -> Grid {
    let mut grid = grid;

    for _ in 0 .. no {
        grid = Grid::merge(
            grid.split()
                .iter()
                .map(| r |
                        r.iter()
                            .map(| g | patterns.apply(g))
                            .collect()
                )
                .collect()
        );
    }

    grid
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example() {
        let patterns: Patterns =
            "../.# => ##./#../...\n.#./..#/### => #..#/..../..../#..#"
                .parse().unwrap();

        let grid = iterate(Grid::default(), 2, &patterns);

        assert_eq!(grid.number_of_on(), 12);
    }
}
