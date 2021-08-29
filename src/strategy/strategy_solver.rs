use crate::{Sudoku, solver::SolverError};

use super::{Strategy, algorithms::NakedSingle};

/// The `StrategySolver` is the struct for solving Sudokus
/// by applying logical strategies that humans can do.
pub struct StrategySolver {
    /// The initial Sudoku
    sudoku: Sudoku,
    /// List of all strategies
    strategies: Vec<Box<dyn Strategy>>,
}

impl StrategySolver {
    /// Creates a new Solver with a list of strategies
    pub fn new(sudoku: &Sudoku) -> Self {
        let mut solver = StrategySolver {
            sudoku: sudoku.clone(),
            strategies: Vec::new(),
        };
        solver.add_default_strategies();
        solver
    }

    /// Solve the Sudoku by applying solving steps.
    pub fn solve(&self) -> Result<(), SolverError> {
        Ok(())
    }

    /// Adds all available default strategies
    fn add_default_strategies(&mut self) {
        self.push_strategy(Box::new(NakedSingle::new()));
    }

    /// Adds a single strategy
    pub fn push_strategy(&mut self, strategy: Box<dyn Strategy>) {
        self.strategies.push(strategy);
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use crate::Sudoku;

    use super::StrategySolver;

    #[test]
    fn solve_sudokus() {
        // A few sudokus found here: https://sandiway.arizona.edu/sudoku/examples.html
        let sudokus = vec![
            r"...26.7.1 68..7..9. 19...45.. 82.1...4. ..46.29.. .5...3.28 ..93...74 .4..5.36 7.3.18...",
        ];

        for s in sudokus {
            let sudoku = Sudoku::try_from(s).unwrap();
            let solver = StrategySolver::new(&sudoku);
            assert!(solver.solve().is_ok());
        }
    }
}
