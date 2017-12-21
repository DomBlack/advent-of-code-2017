use std::fmt::{Display, Debug, Formatter, Result as FmtResult};
use std::str::FromStr;

/// A Grid
#[derive(Eq, PartialEq, Hash, Clone)]
pub struct Grid {
    cells: Vec<Vec<bool>>
}

impl Grid {
    /// Returns the grid size
    pub fn size(&self) -> usize { self.cells.len() }

    /// Counts how many on cells there are
    pub fn number_of_on(&self) -> usize {
        self.cells
            .iter()
            .map( | row | { row.iter().filter(| &x| *x).count() })
            .sum()
    }

    /// Creates a flipped version of this grid on the Y axis
    ///
    /// ```
    /// #.#    #..
    /// ..# => ..#
    /// #..    #.#
    /// ```
    pub fn flip(&self) -> Self {
        let mut grid = Grid { cells: self.cells.clone() };
        grid.cells.reverse();
        grid
    }

    /// Creates a symmetrical version of this grid with both X & Y axis swapped
    ///
    /// i.e. where row 1 becomes column 1
    ///
    /// ```
    /// #.#    #.#
    /// ..# => ...
    /// #..    ##.
    /// ```
    pub fn symmetric(&self) -> Self {
        let mut cells = self.cells.clone();

        for row in 0 .. self.size() {
            for col in 0 .. self.size() {
                cells[col][row] = self.cells[row][col];
            }
        }

        Grid { cells }
    }

    /// Gets all variations of this grid when flipped and rotated
    pub fn variations(&self) -> Vec<Grid> {
        let mut last_variation = self.clone();
        let mut variations = vec![];

        // Loop four times, final flip brings us back to our current variation
        for _ in 0 .. 4 {
            last_variation = last_variation.symmetric();
            variations.push(last_variation.clone());

            // Rotation 90 degree anti clockwise from last_variation at top of loop
            last_variation = last_variation.flip();
            variations.push(last_variation.clone());
        }

        variations
    }

    /// Splits this grid of cells up into a grid of 2x2 or 3x3 grids
    pub fn split(&self) -> Vec<Vec<Grid>> {
        let mut grid_of_grids = vec![];

        let step: usize = if self.size() % 2 == 0 { 2 } else { 3 };

        let mut row_start = 0;

        while row_start < self.size() {
            let mut grids = vec![];
            let mut col_start = 0;

            while col_start < self.size() {
                let mut cells = vec![];

                for row in row_start .. row_start + step {
                    let mut new_row = vec![];

                    for col in col_start .. col_start + step {
                        new_row.push(self.cells[row][col]);
                    }

                    cells.push(new_row);
                }

                grids.push(Grid { cells });
                col_start += step;
            }

            grid_of_grids.push(grids);
            row_start += step;
        }

        grid_of_grids
    }

    /// Takes a Grid of grids and returns a single Grid of cells
    pub fn merge(grids: Vec<Vec<Grid>>) -> Self {
        let mut cells: Vec<Vec<bool>> = vec![];

        let size: usize = grids.last().unwrap().last().unwrap().size();

        for grid_row in &grids {
            for row in 0 .. size {
                let mut merged_row: Vec<bool> = vec![];

                for sub_grid in grid_row {
                    merged_row.extend(sub_grid.cells[row].clone());
                }

                cells.push(merged_row);
            }
        }

        Grid { cells }
    }
}

impl Default for Grid {
    fn default() -> Self {
        Self {
            cells: vec![vec![false, true, false], vec![false, false, true], vec![true, true, true]]
        }
    }
}

impl FromStr for Grid {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cells:  Vec<Vec<bool>> = vec![];

        // Parse the rows out
        for row in s.trim().split("/") {
            let mut row_cells: Vec<bool> = vec![];

            for cell in row.chars() {
                match cell {
                    '#' => row_cells.push(true),
                    '.' => row_cells.push(false),
                    _   => return Err(format!("Unknown Cell Value: {}", cell)),
                }
            }

            cells.push(row_cells);
        }

        // Double check all rows/cols have the same number of items
        for row in &cells {
            if row.len() != cells.len() {
                return Err(
                    format!(
                        "Row has {} cells, but Grid has {} rows - Should be the same!",
                        row.len(), cells.len()
                    )
                );
            }
        }

        Ok(Grid { cells })
    }
}

impl Debug for Grid {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        let mut first_row = true;
        // Write out the cell
        for row in &self.cells {
            if first_row {
                first_row = false;
            } else {
                write!(f, "/")?;
            }

            for cell in row {
                if *cell { write!(f, "#")?; } else { write!(f, ".")?; }
            }
        }

        Ok(())
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        // Write out the cell
        for row in &self.cells {
            for cell in row {
                if *cell { write!(f, "#")?; } else { write!(f, ".")?; }
            }
            write!(f, "\n")?;
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
            Grid::default(),
            Grid { cells: {
                vec![
                    vec![false, true,  false],
                    vec![false, false, true],
                    vec![true,  true,  true],
                ]
            }}
        )
    }

    #[test]
    fn test_parse_and_default() {
        assert_eq!(
            "##./.#./#..".parse(),
            Ok(Grid { cells: {
                vec![
                    vec![true,  true,  false],
                    vec![false, true,  false],
                    vec![true,  false, false],
                ]
            }})
        )
    }

    #[test]
    fn test_number_on() {
        let grid: Grid = "##./.#./#..".parse().unwrap();

        assert_eq!(grid.number_of_on(), 4);
    }

    #[test]
    fn test_size() {
        assert_eq!(
            Grid::default().size(),
            3
        );

        assert_eq!(
            "##/.#".parse::<Grid>().unwrap().size(),
            2
        )
    }

    #[test]
    fn test_flip() {
        let start: Grid = "#.#/..#/#..".parse().unwrap();
        let expected: Grid = "#../..#/#.#".parse().unwrap();

        assert_eq!(
            start.flip(),
            expected
        );
    }

    #[test]
    fn test_symmetric() {
        let start: Grid = "#.#/..#/#..".parse().unwrap();
        let expected: Grid = "#.#/.../##.".parse().unwrap();

        assert_eq!(
            start.symmetric(),
            expected
        );
    }

    #[test]
    fn test_rotate() {
        let start: Grid = "#.#/..#/#..".parse().unwrap();
        let expected: Grid = "##./.../#.#".parse().unwrap();

        assert_eq!(
            start.rotate90(),
            expected
        );
    }

    #[test]
    fn test_variation() {
        let start: Grid = "#.#/..#/#..".parse().unwrap();

        let variations = start.variations();

        assert_eq!(
            variations.len(),
            8,
            "There should be 8 variations"
        );

        let mut variations_no_duplicates = variations.clone();
        variations_no_duplicates.dedup();
        assert_eq!(
            variations, variations_no_duplicates,
            "There should be no duplicate variations"
        );

        assert_eq!(
            *variations.last().unwrap(),
            start,
            "Last variation should equal the original"
        );
    }

    #[test]
    fn test_split() {
        let grid_3x3: Grid = "..#/#.#/...".parse().unwrap();
        assert_eq!(
            grid_3x3.split(),
            vec![vec![grid_3x3]]
        );

        let grid_4x4: Grid = ".#.#/#..#/##.#/#...".parse().unwrap();
        assert_eq!(
            grid_4x4.split(),
            vec![
                vec![
                    ".#/#.".parse().unwrap(),
                    ".#/.#".parse().unwrap(),
                ],
                vec![
                    "##/#.".parse().unwrap(),
                    ".#/..".parse().unwrap()
                ]
            ]
        );
    }

    #[test]
    fn test_merge() {
        let grid_4x4: Grid = ".#.#/#..#/##.#/#...".parse().unwrap();
        assert_eq!(
            grid_4x4,
            Grid::merge(grid_4x4.split())
        );
    }
}
