use crate::solver::SolverError;

use super::{Strategy, algorithms::NakedSingle};

/// The `StrategySolver` is the struct for solving Sudokus
/// by applying logical strategies that humans can do.
pub struct StrategySolver {
    strategies: Vec<Box<dyn Strategy>>,
}

impl StrategySolver {
    /// Creates a new Solver with a list of strategies
    pub fn new() -> Self {
        let mut solver = StrategySolver {
            strategies: Vec::new(),
        };

        solver.add_default_strategies();
        solver
    }

    /// Solve the Sudoku by applying solving steps.
    pub fn solve() -> Result<(), SolverError> {
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
