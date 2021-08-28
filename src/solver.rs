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
    /// Brute Force Solver
    pub fn solve(sudoku: &Sudoku) -> Result<Sudoku, SolverError> {
        let sudoku = sudoku.clone();
        Self::solve_sudoku(sudoku)
    }

    fn solve_sudoku(mut sudoku: Sudoku) -> Result<Sudoku, SolverError> {
        for y in 0..sudoku.num_rows() {
            for x in 0..sudoku.num_cols() {
                if sudoku.get(x, y) == Some(&Value::Unset) {
                    for value in 1..=9 {
                        let v = Value::Number(value);
                        if Self::possible(&sudoku, y, x, &v) {
                            sudoku.set(x, y, v);
                        }
                    }
                }
            }
        }
        Ok(sudoku)
    }

    fn possible(sudoku: &Sudoku, y: u32, x: u32, value: &Value) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
}
