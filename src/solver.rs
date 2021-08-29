use std::fmt;

use crate::sudoku::{Sudoku, Value};

pub enum SolverError {
    /// 
    Unsolvable(String),
}

impl fmt::Display for SolverError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            SolverError::Unsolvable(err) => format!("Unsolvable grid: {}", err),
        };
        write!(f, "{}", s)
    }
}

pub struct Solver {}

impl Solver {
    /// Brute Force Solver
    pub fn solve(sudoku: &Sudoku) -> Result<Sudoku, SolverError> {
        let sudoku = sudoku.clone();
        Self::solve_sudoku(sudoku)
    }

    fn solve_sudoku(mut sudoku: Sudoku) -> Result<Sudoku, SolverError> {
        for row in 0..sudoku.num_rows() {
            for col in 0..sudoku.num_cols() {
                if sudoku.get(row, col) == Some(&Value::Unset) {
                    for value in 1..=9 {
                        if Self::possible(&sudoku, row, col, value) {
                            sudoku.set(row, col, Value::Number(value));
                            if sudoku.is_solved() {
                                return Ok(sudoku);
                            }
                            sudoku.set(row, col, Value::Unset);
                        }
                    }
                }
            }
        }
        Err(SolverError::Unsolvable(format!("Failed to find solution")))
    }

    /// Slow check if the given value for field x, y can be set
    fn possible(sudoku: &Sudoku, row: u8, col: u8, value: u8) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
}