use crate::{Sudoku, Value};

use super::Cell;

/// Sudoku grid to manipulate with all cells + candidates
/// The struct is only a helper grid to manipulate during solving.
#[derive(Debug)]
pub struct Grid {
    /// The list of all cells
    cells: Vec<Cell>,
}

impl Grid {
    /// Creates a Grid from a given Sudoku to solve with strategies
    pub fn new(sudoku: &Sudoku) -> Self {
        let cells = sudoku
            .iter()
            .map(|(index, value)| match value {
                Value::Empty => Cell::new_empty(index),
                Value::Number(digit) => Cell::new_digit(index, digit),
            })
            .collect();

        let mut grid = Grid { cells };
        grid.init_candidates();
        grid
    }

    /// Determines and initializes all empty fields with candidates
    ///
    /// **Note** this will not check or validate the candidates, e.g. empty fields
    ///
    fn init_candidates(&mut self) {
        self.cells.iter_mut().for_each(|cell| {
            if let Some(candidates) = cell.candidates_mut() {
                // TODO continue here?
            }
        });
    }
}

impl From<&Sudoku> for Grid {
    fn from(sudoku: &Sudoku) -> Self {
        Grid::new(sudoku)
    }
}

#[cfg(test)]
mod tests {

}
