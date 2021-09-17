use std::fmt;

use crate::sudoku::Sudoku;

#[derive(Debug, PartialEq)]
pub enum SolverError {
    /// There is no single solution found
    Unsolvable,
    /// There is more than one solution
    TooManySolutions(u32),
    /// There are too few given clues to run successfully
    TooFewClues(u32),
}

impl fmt::Display for SolverError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            SolverError::Unsolvable => format!("No solution found."),
            SolverError::TooManySolutions(n) => {
                format!("There is no unique solution (count: {})", n)
            }
            SolverError::TooFewClues(n) => {
                format!("Too few clues given ({})", n)
            }
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug)]
pub struct Solver {
    /// The list of all solutions
    solutions: Vec<Sudoku>,
}

impl Solver {
    /// Tries to find a single unique solution for the given Sudoku
    pub fn find_unique(sudoku: &Sudoku) -> Result<Sudoku, SolverError> {
        let solver = Self::find_all(&mut sudoku.clone());
        match solver.solutions.len() {
            1 => Ok(solver.solutions[0].clone()),
            0 => Err(SolverError::Unsolvable),
            n => Err(SolverError::TooManySolutions(n as u32)),
        }
    }

    /// Useful to find all possible solutions of a given Sudoku
    /// **NOTE** can take a while to run.
    pub fn find_all(sudoku: &Sudoku) -> Solver {
        let mut solver = Self {
            solutions: Vec::new(),
        };

        solver.solve_sudoku(&mut sudoku.clone());
        solver
    }

    /// Brute Force Solver using recursion, trying to find all solutions
    fn solve_sudoku(&mut self, sudoku: &mut Sudoku) {
        if sudoku.is_solved() {
            self.solutions.push(sudoku.clone());
            return;
        }

        // TODO find a way to not start from front
        for row in 0..Sudoku::ROWS {
            for col in 0..Sudoku::COLS {
                if sudoku.get(row, col).is_empty() {
                    for value in 1..=9 {
                        if Self::possible(&sudoku, row, col, value) {
                            sudoku.set(row, col, value);
                            self.solve_sudoku(sudoku);
                            sudoku.unset(row, col);
                        }
                    }
                    // unwind when no candidate were found
                    return;
                }
            }
        }
    }

    /// Slow check if the given value for field row, col can be set
    fn possible(sudoku: &Sudoku, row: u8, col: u8, value: u8) -> bool {
        sudoku.get_house(row, col).find(|cell| cell.digit() == value).is_none()
    }

    /// Returns the list of solutions
    pub fn solutions(&self) -> &[Sudoku] {
        &self.solutions
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use crate::{solver::SolverError, Solver, Sudoku};

    #[test]
    fn solves_sudoku() {
        // taken from https://en.wikipedia.org/wiki/Sudoku
        #[rustfmt::skip]
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
        let result = Solver::find_unique(&sudoku);

        assert!(result.is_ok());
        let actual = result.unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn fails_to_solve_invalid_sudoku() {
        // unsolvable square from: http://sudopedia.enjoysudoku.com/Invalid_Test_Cases.html
        let sudoku =
            "..9.287..8.6..4..5..3.....46.........2.71345.........23.....5..9..4..8.7..125.3..";

        let sudoku = Sudoku::try_from(sudoku).unwrap();
        assert_eq!(
            SolverError::Unsolvable,
            Solver::find_unique(&sudoku).unwrap_err()
        );
    }

    #[test]
    fn finds_2_solutions() {
        // Not Unique - 2 solutions: http://sudopedia.enjoysudoku.com/Invalid_Test_Cases.html
        let sudoku =
            ".39...12....9.7...8..4.1..6.42...79...........91...54.5..1.9..3...8.5....14...87.";
        let sudoku = Sudoku::try_from(sudoku).unwrap();

        let solver = Solver::find_all(&sudoku);
        assert_eq!(2, solver.solutions.len());

        let expected = vec![
            Sudoku::try_from(
                "439658127156927384827431956342516798785294631691783542578149263263875419914362875",
            )
            .unwrap(),
            Sudoku::try_from(
                "439658127156927384827431956642513798785294631391786542578149263263875419914362875",
            )
            .unwrap(),
        ];

        assert_eq!(
            SolverError::TooManySolutions(2),
            Solver::find_unique(&sudoku).unwrap_err()
        );
        assert!(solver
            .solutions
            .iter()
            .all(|solution| expected.contains(solution)));
    }
}
