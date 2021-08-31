use std::collections::HashSet;

use crate::{Sudoku, Value, sudoku::GridError, types::{BLOCKS, COLS, HOUSES, Pos, ROWS}};

use super::{Cell, CellState, Digit};

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

    /// Initializes all empty fields with candidates.
    ///
    /// **Note** this will not check or validate the candidates, e.g. empty fields
    ///
    fn init_candidates(&mut self) {
        for row in 0..9 {
            for col in 0..9 {
                let index = col + row * Sudoku::ROWS;
                // TODO make this more pleasant, bitwise OR ing
                // ?
                if self.cells[index as usize].is_empty() {
                    let _neighbors = self
                        .get_house(index)
                        .map(|neighbor| neighbor.value())
                        .filter(|v| *v == 0)
                        .collect::<HashSet<u8>>();
                }
            }
        }
    }

    /// Returns an iterator over all cells
    pub fn cells(&self) -> impl Iterator<Item = &Cell> {
        self.cells.iter()
    }

    /// Returns the cell at coordinates row, col
    pub fn get(&self, row: u8, col: u8) -> &Cell {
        let index = col + row * Sudoku::ROWS;
        &self.cells[index as usize]
    }

    /// Sets digit to this cell
    pub fn set(&mut self, row: u8, col: u8, digit: u8) {
        let index = col + row * Sudoku::ROWS;
        self.cells[index as usize].state = CellState::Number(Digit::new(digit));
    }

    /// Naive version to check if Sudoku is solved
    /// **Note** ignores any checks that each line, row and block contains of digits 1..9
    pub fn is_solved(&self) -> bool {
        self.cells.iter().all(|cell| cell.is_digit() )
    }

    /// Returns all fields for the given row
    pub fn get_row<'a>(&'a self, row: u8) -> impl Iterator<Item = &Cell> + 'a {
        let indices = &ROWS[row as usize];
        indices
            .iter()
            .map(move |&index| &self.cells[index as usize])
    }

    /// Returns all fields for the given column
    pub fn get_col<'a>(&'a self, col: u8) -> impl Iterator<Item = &Cell> + 'a {
        let indices = &COLS[col as usize];
        indices
            .iter()
            .map(move |&index| &self.cells[index as usize])
    }

    /// Returns all fields from the given block
    pub fn get_block<'a>(&'a self, row: u8, col: u8) -> impl Iterator<Item = &Cell> + 'a {
        let index = Pos::new(row, col).block();
        let indices = &BLOCKS[index as usize];
        indices
            .iter()
            .map(move |&index| &self.cells[index as usize])
    }

    /// Returns the house, all fields from same row, col and block
    pub fn get_house<'a>(&'a self, index: u8) -> impl Iterator<Item = &Cell> + 'a {
        let indices = &HOUSES[index as usize];
        indices
            .iter()
            .map(move |&index| &self.cells[index as usize])
    }

    /// Returns the grid as a Sudoku
    pub fn sudoku(&self) -> Result<Sudoku, GridError> {
        let numbers = self.cells
            .iter()
            .map(|cell| cell.value())
            .collect::<Vec<_>>();
        Sudoku::new(numbers)
    }
}

impl From<Sudoku> for Grid {
    fn from(sudoku: Sudoku) -> Self {
        Grid::new(&sudoku)
    }
}

impl From<&Sudoku> for Grid {
    fn from(sudoku: &Sudoku) -> Self {
        Grid::new(sudoku)
    }
}

#[cfg(test)]
mod tests {
    use crate::{Sudoku, strategy::grid::Grid};

    #[test]
    fn test_init_candidates() {
        let sudoku: Vec<u8> = vec![
            8, 0, 0, 7, 3, 9, 0, 0, 6,
            3, 7, 0, 4, 6, 5, 0, 0, 0,
            0, 4, 0, 1, 8, 2, 0, 0, 9,
            0, 0, 0, 6, 0, 0, 0, 4, 0,
            0, 5, 4, 3, 0, 0, 6, 1, 0,
            0, 6, 0, 5, 0, 0, 0, 0, 0,
            4, 0, 0, 8, 5, 3, 0, 7, 0,
            0, 0, 0, 2, 7, 1, 0, 6, 4,
            1, 0, 0, 9, 4, 0, 0, 0, 2,
        ];

        let grid: Grid = Sudoku::new(sudoku).unwrap().into();
        // grid.get(0, 1).
    }
}
