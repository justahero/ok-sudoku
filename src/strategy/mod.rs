mod algorithms;
mod grid;
mod steps;
mod strategy_solver;

pub use strategy_solver::StrategySolver;

use self::{grid::Grid, steps::Steps};

#[derive(Debug, Clone)]
pub struct Candidates(u16);

impl Candidates {
    pub fn new() -> Candidates {
        Candidates(0)
    }

    /// Sets the given candidate
    pub fn set(&mut self, candidate: u8) {
        self.0 |= 1 << candidate;
    }

    /// Returns true if candidate is set
    pub fn is_set(&self, candidate: u8) -> bool {
        // self.0.get((candidate - 1) as usize).is_some()
        self.0 & (1 << candidate) > 0
    }

    /// Returns the inner bits
    pub fn as_bits(&self) -> &u16 {
        &self.0
    }

    /// Returns an iterator over all candidates
    pub fn iter(&self) -> impl Iterator<Item = u8> + '_ {
        (1u8..=9)
            .into_iter()
            .filter(move |&candidate| self.is_set(candidate))
            .map(|candidate| candidate)
    }

    /// Returns the number of set candidates
    pub fn count(&self) -> u32 {
        self.0.count_ones() as u32
    }
}

// TODO add impls for '&', '|', []: to bool etc that make sense

/// A Cell represents the content of a single field on the grid
#[derive(Debug, Clone)]
pub struct Cell {
    /// The index on the board
    index: u8,
    /// The state of the cell, a number or candidates
    state: CellState,
}

impl Cell {
    /// Creates a Cell with a Digit
    pub fn new_digit(index: u8, digit: u8) -> Self {
        Cell {
            index,
            state: CellState::Number(digit),
        }
    }

    /// Creates a Cell with empty Candidates list
    pub fn new_empty(index: u8) -> Self {
        Cell {
            index,
            state: CellState::Candidate(Candidates::new()),
        }
    }

    /// Returns true if the cell is a digit
    pub fn is_digit(&self) -> bool {
        self.state.is_digit()
    }

    /// Returns true if the cell does not contain a value
    pub fn is_empty(&self) -> bool {
        self.state.is_empty()
    }

    /// Returns the digit value of the cell, either 1-9 or 0 if unset
    pub fn value(&self) -> u8 {
        self.state.value()
    }

    /// Returns the list of candidates
    pub fn candidates(&self) -> Option<&Candidates> {
        self.state.candidates()
    }

    /// Returns the candidates if available
    pub fn candidates_mut(&mut self) -> Option<&mut Candidates> {
        self.state.canddiates_mut()
    }
}

#[derive(Debug, Clone)]
pub enum CellState {
    /// A specific set number
    Number(u8),
    /// A set of candidates
    Candidate(Candidates),
}

impl CellState {
    /// Returns the digit value of the cell, either 1-9 or 0 if unset
    pub fn value(&self) -> u8 {
        match self {
            CellState::Candidate(_) => 0,
            CellState::Number(digit) => *digit,
        }
    }

    /// Returns the list of candidates if available
    pub fn candidates(&self) -> Option<&Candidates> {
        match self {
            CellState::Candidate(candidates) => Some(&candidates),
            _ => None
        }
    }

    /// Returns a mutable reference to list of candidates if available
    pub fn canddiates_mut(&mut self) -> Option<&mut Candidates> {
        match self {
            CellState::Candidate(candidates) => Some(candidates),
            _=> None,
        }
    }

    /// Returns true if the state is valid digit
    pub fn is_digit(&self) -> bool {
        match self {
            CellState::Number(_) => true,
            CellState::Candidate(_) => false,
        }
    }

    /// Returns true if state does not contain a digit
    pub fn is_empty(&self) -> bool {
        match self {
            CellState::Number(_) => false,
            CellState::Candidate(_) => true,
        }
    }
}

/// A `Strategy` is a distinct way to apply logic to determine
/// the next digit.
pub trait Strategy {
    fn find(&self, sudoku: &Grid) -> Option<Steps>;
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
