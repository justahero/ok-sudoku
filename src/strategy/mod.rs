mod algorithms;
mod steps;
mod strategy_solver;

use std::num::NonZeroU8;

use bitvec::prelude::*;
pub use strategy_solver::StrategySolver;

use crate::Sudoku;

use self::steps::Steps;

#[derive(Debug)]
pub struct Candidates(BitArray);

impl Candidates {
    pub fn new() -> Candidates {
        Candidates(bitarr![0; 9])
    }

    /// Sets the given candidate
    pub fn set(&mut self, candidate: u8) {
        let digit = Digit::new(candidate);
        self.0.set(digit.index() as usize, true);
    }

    /// Returns true if candidate is set
    pub fn is_set(index: u8) -> bool {
        false
    }

    /// Returns an iterator over all candidates
    pub fn iter(&self) -> impl Iterator<Item = u8> + '_ {
        self.0.iter_ones().map(|v| v as u8 + 1)
    }

    /// Returns the number of set candidates
    pub fn count(&self) -> usize {
        self.0.count_ones()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd)]
pub struct Digit(NonZeroU8);

impl Digit {
    pub fn new(digit: u8) -> Self {
        assert!(digit > 0);
        assert!(digit <= 9);
        Self::new_unchecked(digit).unwrap()
    }

    pub fn new_unchecked(digit: u8) -> Option<Self> {
        NonZeroU8::new(digit).map(Self)
    }

    pub fn index(&self) -> u8 {
        (self.0.get() - 1) as u8
    }
}

/// A Cell represents the content of a single field on the grid
#[derive(Debug)]
pub enum Cell {
    /// A specific set number
    Number(Digit),
    /// A set of candidates
    Candidate(Candidates),
}

/// A `Strategy` is a distinct way to apply logic to determine
/// the next digit.
pub trait Strategy {
    fn find(&self, sudoku: &Sudoku) -> Option<Steps>;
}

#[cfg(test)]
mod tests {
    use super::Candidates;

    #[test]
    fn test_candidates_count() {
        let mut candidates = Candidates::new();
        assert_eq!(0, candidates.count());

        candidates.set(1);
        candidates.set(1);
        candidates.set(2);
        assert_eq!(2, candidates.count());
    }

    #[test]
    fn test_candidates_iterator() {
        let mut candidates = Candidates::new();
        candidates.set(1);
        candidates.set(4);
        candidates.set(6);

        assert_eq!(vec![1, 4, 6], candidates.iter().collect::<Vec<_>>());
    }
}
