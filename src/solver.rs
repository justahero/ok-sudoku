use std::fmt;

use crate::sudoku::{Sudoku, Value};

#[derive(Debug)]
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

pub struct Solver {
    sudoku: Sudoku,
}

impl Solver {
    /// Brute Force Solver
    pub fn solve(sudoku: &Sudoku) -> Result<Sudoku, SolverError> {
        let sudoku = sudoku.clone();
        let mut solver = Self { sudoku };
        if solver.solve_sudoku() {
            Ok(solver.sudoku)
        } else {
            Err(SolverError::Unsolvable(format!("Failed to find solution!")))
        }
    }

    fn solve_sudoku(&mut self) -> bool {
        for row in 0..self.sudoku.num_rows() {
            for col in 0..self.sudoku.num_cols() {
                if self.sudoku.get(row, col) == Some(&Value::Empty) {
                    for value in 1..=9 {
                        if self.possible(row, col, value) {
                            self.sudoku.set(row, col, Value::Number(value));
                            if self.solve_sudoku() {
                                return true;
                            }
                            self.sudoku.unset(row, col);
                        }
                    }
                    // unwind when no candidate was found
                    return false;
                }
            }
        }
        true
    }

    /// Slow check if the given value for field row, col can be set
    fn possible(&self, row: u8, col: u8, value: u8) -> bool {
        if let Some(_) = self.sudoku.get_row(row).find(|&v| Value::Number(value) == v) {
            return false;
        }

        if let Some(_) = self.sudoku.get_col(col).find(|&v| Value::Number(value) == v) {
            return false;
        }

        if let Some(_) = self.sudoku.get_block(row, col).find(|&v| Value::Number(value) == v) {
            return false;
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use crate::{Solver, Sudoku};

    #[test]
    fn solves_sudoku() {
        // taken from https://en.wikipedia.org/wiki/Sudoku
        let expected = vec![
            5, 3, 4, 6, 7, 8, 9, 1, 2,
            6, 7, 2, 1, 9, 5, 3, 4, 8,
            1, 9, 8, 3, 4, 2, 5, 6, 7,
            8, 5, 9, 7, 6, 1, 4, 2, 3,
            4, 2, 6, 8, 5, 3, 7, 9, 1,
            7, 1, 3, 9, 2, 4, 8, 5, 6,
            9, 6, 1, 5, 3, 7, 2, 8, 4,
            2, 8, 7, 4, 1, 9, 6, 3, 5,
            3, 4, 5, 2, 8, 6, 1, 7, 9,
        ];
        let sudoku = r"
            53. .7. ...
            6.. 195 ...
            .98 ... .6.
            8.. .6. ..3
            4.. 8.3 ..1
            7.. .2. ..6
            .6. ... 28.
            ... 419 ..5
            ... .8. .79
        ";

        let expected = Sudoku::new(expected).unwrap();
        let sudoku = Sudoku::try_from(sudoku).unwrap();
        let result = Solver::solve(&sudoku);

        assert!(result.is_ok());
        let actual = result.unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn fails_to_solve_invalid_sudoku() {
        // unsolvable square from: http://sudopedia.enjoysudoku.com/Invalid_Test_Cases.html
        let sudoku = "..9.287..8.6..4..5..3.....46.........2.71345.........23.....5..9..4..8.7..125.3..";

        let sudoku = Sudoku::try_from(sudoku).unwrap();
        assert!(Solver::solve(&sudoku).is_err());
    }
}
