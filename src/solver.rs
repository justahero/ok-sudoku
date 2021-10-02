use std::fmt;

use crate::sudoku::Sudoku;

#[derive(Debug, PartialEq)]
pub enum SolverError {
    /// There exists a single solution but there is no strategy that finds the next step
    StrategyNotFound,
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
            SolverError::StrategyNotFound => format!("No suitable strategy found, but solution exists."),
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

        solver.solve_sudoku(&mut sudoku.clone(), 0);
        solver
    }

    /// Brute Force Solver using recursion, trying to find all solutions
    fn solve_sudoku(&mut self, sudoku: &mut Sudoku, start_index: usize) {
        if sudoku.is_solved() {
            self.solutions.push(sudoku.clone());
            return;
        }

        for index in start_index..Sudoku::NUM_FIELDS {
            if sudoku.get(index).is_empty() {
                for value in 1..=9 {
                    if Self::possible(&sudoku, index, value) {
                        sudoku.set_digit(index, value);
                        self.solve_sudoku(sudoku, index + 1);
                        sudoku.unset(index);
                    }
                }
                // unwind when no candidate were found
                return;
            }
        }
    }

    /// Slow check if the given value for field row, col can be set
    #[inline(always)]
    fn possible(sudoku: &Sudoku, index: usize, value: u8) -> bool {
        sudoku.get_house(index).find(|cell| cell.digit() == value).is_none()
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

    #[test]
    fn solve_hard_sudokus() {
        let sudokus = [
            r"...1.4.96 ..9....1. 1.59..... ..4..1972 .18792.3. 2974..1.8 ...2.83.9 48.379... 9..5.678.",
            // Y-Wing
            r"51.394.69 .631..4.9 ..47.6.31 ...4..1.. 43..71..2 1.82.9.4. ...942316 641..3..7 329617..4",
        ];

        for sudoku in sudokus.iter() {
            let sudoku = Sudoku::try_from(*sudoku).unwrap();
            assert!(Solver::find_unique(&sudoku).is_ok());
        }
    }
}
