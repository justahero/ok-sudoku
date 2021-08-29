mod algorithms;
mod steps;
mod strategy_solver;

pub use strategy_solver::StrategySolver;

use crate::Sudoku;

use self::steps::Steps;

/// A `Strategy` is a distinct way to apply logic to determine
/// the next digit.
pub trait Strategy {
    fn find(&self, sudoku: &Sudoku) -> Option<Steps>;
}
