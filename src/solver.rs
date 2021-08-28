use std::fmt;

use crate::{Sudoku, sudoku::Value};

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
    /// 
    pub fn solve(sudoku: &Sudoku) -> Result<Sudoku, SolverError> {
        let sudoku = sudoku.clone();
        Self::solve_sudoku(sudoku)
    }

    fn solve_sudoku(sudoku: Sudoku) -> Result<Sudoku, SolverError> {
        for y in 0..sudoku.num_rows() {
            for x in 0..sudoku.num_cols() {
                if sudoku.get(x, y) == Some(&Value::Unset) {
                    
                }
            }
        }
        Ok(sudoku)
    }
}

#[cfg(test)]
mod tests {
}
