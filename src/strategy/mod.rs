mod algorithms;
mod step;
mod strategy_solver;

pub use strategy_solver::StrategySolver;

use crate::Sudoku;

use self::step::Step;

/// A `Strategy` is a distinct way to apply logic to eliminate candidates or determine
/// the next digit.
pub trait Strategy {
    /// Finds the next step or None
    fn find(&self, sudoku: &Sudoku) -> Option<Step>;
    /// Returns the name of the strategy
    fn name(&self) -> String;
}
