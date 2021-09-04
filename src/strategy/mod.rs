mod algorithms;
mod step;
mod strategy_solver;

use bit_vec::BitVec;
pub use strategy_solver::StrategySolver;

use crate::Sudoku;

use self::step::Step;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Candidates(BitVec);

impl Candidates {
    /// Creates a new candidates with all candidates
    pub fn all() -> Candidates {
        Candidates(BitVec::from_elem(10, true))
    }

    /// Creates a new empty candidates set
    pub fn empty() -> Candidates {
        Candidates(BitVec::from_elem(10, false))
    }

    /// Returns true if the candidates set is empty
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Sets the given candidate
    pub fn set(&mut self, candidate: u8) {
        self.0.set(candidate as usize, true);
    }

    /// Unsets the given candidate
    pub fn unset(&mut self, candidate: u8) {
        self.0.set(candidate as usize, false);
    }

    /// Returns true if candidate is set
    pub fn get(&self, candidate: u8) -> bool {
        self.0.get(candidate as usize).unwrap_or(false)
    }

    /// Returns an iterator over all candidates
    pub fn iter(&self) -> impl Iterator<Item = u8> + '_ {
        self.0
            .iter()
            .enumerate()
            .filter(|(_index, v)| *v)
            .map(|(index, _)| index as u8)
    }

    /// Returns the number of set candidates
    pub fn count(&self) -> u32 {
        self.0.iter().filter(|v| *v).count() as u32
    }

    /// Returns the intersection of two bit sets
    pub fn intersect(lhs: &Self, rhs: &Self) -> Self {
        let mut lhs = lhs.clone();
        lhs.0.and(&rhs.0);
        lhs
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Cell {
    /// A specific set number
    Number(u8),
    /// A set of candidates
    Candidates(Candidates),
}

impl Cell {
    /// Creates an empty Cell without candidates
    pub fn empty() -> Self {
        Cell::Candidates(Candidates::empty())
    }

    /// Sets the digit of this cell
    pub fn set_digit(&mut self, digit: u8) {
        assert!(digit > 0);
        *self = Cell::Number(digit);
    }

    /// Returns the digit value of the cell, either 1-9 or 0 if unset
    pub fn digit(&self) -> u8 {
        match self {
            Cell::Candidates(_) => 0,
            Cell::Number(digit) => *digit,
        }
    }

    /// Sets the list of candidates
    pub fn set_candidates(&mut self, candidates: Candidates) {
        *self = Cell::Candidates(candidates);
    }

    /// Returns the list of candidates if available
    pub fn candidates(&self) -> Candidates {
        match self {
            Cell::Candidates(candidates) => candidates.clone(),
            _ => Candidates::empty(),
        }
    }

    /// Returns an iterator over all candidates
    /// In case the cell contains a digit, this returns no candidates otherwise
    /// the list of candidates
    pub fn candidates_vec(&self) -> Vec<u8> {
        match self {
            Cell::Candidates(candidates) => candidates.iter().collect(),
            _ => Vec::new(),
        }
    }

    /// Returns true if the state is valid digit
    pub fn is_digit(&self) -> bool {
        match self {
            Cell::Number(_) => true,
            Cell::Candidates(_) => false,
        }
    }

    /// Returns true if state does not contain a digit
    pub fn is_empty(&self) -> bool {
        match self {
            Cell::Number(_) => false,
            Cell::Candidates(_) => true,
        }
    }

    /// Unsets the cell to an empty one without candidates
    pub fn unset(&mut self) {
        *self = Cell::Candidates(Candidates::empty())
    }
}

/// A `Strategy` is a distinct way to apply logic to eliminate candidates or determine
/// the next digit.
pub trait Strategy {
    fn find(&self, sudoku: &Sudoku) -> Option<Step>;
}

#[cfg(test)]
mod tests {
    use super::Candidates;

    #[test]
    fn test_candidates_count() {
        let mut candidates = Candidates::empty();
        assert_eq!(0, candidates.count());

        candidates.set(1);
        candidates.set(1);
        candidates.set(2);
        assert_eq!(2, candidates.count());
    }

    #[test]
    fn test_candidates_iterator() {
        let mut candidates = Candidates::empty();
        candidates.set(1);
        candidates.set(4);
        candidates.set(6);

        assert_eq!(vec![1, 4, 6], candidates.iter().collect::<Vec<_>>());
    }
}
