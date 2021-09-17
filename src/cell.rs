use crate::Candidates;

/// The state of single cell on the board with either a digit or a list of candidates
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CellState {
    /// A specific set digit
    Number(u8),
    /// A set of candidates
    Candidates(Candidates),
}

impl CellState {
    /// Creates an empty Cell without candidates
    pub fn empty() -> Self {
        CellState::Candidates(Candidates::empty())
    }

    /// Sets a digit for this cell
    pub fn number(digit: u8) -> Self {
        CellState::Number(digit)
    }

    /// Sets the digit of this cell
    pub fn set_digit(&mut self, digit: u8) {
        assert!(digit > 0);
        *self = CellState::Number(digit);
    }

    /// Returns the digit value of the cell, either 1-9 or 0 if unset
    pub fn digit(&self) -> u8 {
        match self {
            CellState::Candidates(_) => 0,
            CellState::Number(digit) => *digit,
        }
    }

    /// Returns true if the state contains this candidate
    pub fn has_candidate(&self, candidate: u8) -> bool {
        if let CellState::Candidates(candidates) = self {
            return candidates.get(candidate)
        }
        false
    }

    /// Unsets the candidate from this Cell, returns true if unset
    pub fn unset_candidate(&mut self, candidate: u8) -> bool {
        match self {
            CellState::Candidates(candidates) => {
                candidates.unset(candidate);
                true
            }
            CellState::Number(_) => false,
        }
    }

    /// Sets the list of candidates
    pub fn set_candidates(&mut self, candidates: Candidates) {
        *self = CellState::Candidates(candidates);
    }

    /// Returns the list of candidates if available
    pub fn candidates(&self) -> Candidates {
        match self {
            CellState::Candidates(candidates) => candidates.clone(),
            _ => Candidates::empty(),
        }
    }

    /// Returns an iterator over all candidates
    /// In case the cell contains a digit, this returns no candidates otherwise
    /// the list of candidates
    pub fn candidates_vec(&self) -> Vec<u8> {
        match self {
            CellState::Candidates(candidates) => candidates.iter().collect(),
            _ => Vec::new(),
        }
    }

    /// Returns true if the state is valid digit
    pub fn is_digit(&self) -> bool {
        match self {
            CellState::Number(_) => true,
            CellState::Candidates(_) => false,
        }
    }

    /// Returns true if state does not contain a digit
    pub fn is_empty(&self) -> bool {
        match self {
            CellState::Number(_) => false,
            CellState::Candidates(_) => true,
        }
    }

    /// Unsets the cell to an empty one without candidates
    pub fn unset(&mut self) {
        *self = CellState::Candidates(Candidates::empty())
    }
}

/// A single cell on the Sudoku grid with index and state
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cell {
    /// The index on the Sudoku board for convenvience
    /// See `Sudoku` for details on the board.
    index: usize,
    /// The state of the cell
    state: CellState,
}

impl Cell {
    /// Creates an empty Cell without candidates
    pub fn empty(index: usize) -> Self {
        Self {
            index,
            state: CellState::empty(),
        }
    }

    /// Creates a new Cell with a set number / digit
    pub fn number(index: usize, digit: u8) -> Self {
        Self {
            index,
            state: CellState::number(digit),
        }
    }

    /// Sets the digit of this cell
    pub fn set_digit(&mut self, digit: u8) {
        assert!(digit > 0);
        self.state.set_digit(digit);
    }

    /// Returns the digit value of the cell, either 1-9 or 0 if unset
    pub fn digit(&self) -> u8 {
        self.state.digit()
    }

    /// Returns true if this cell contains this candidate
    pub fn has_candidate(&self, candidate: u8) -> bool {
        self.state.has_candidate(candidate)
    }

    /// Sets the list of candidates
    pub fn set_candidates(&mut self, candidates: Candidates) {
        self.state.set_candidates(candidates);
    }

    /// Unsets a single candidate from the cell, returns true if successful
    pub fn unset_candidate(&mut self, candidate: u8) -> bool {
        self.state.unset_candidate(candidate)
    }

    /// Returns the list of candidates if available
    pub fn candidates(&self) -> Candidates {
        self.state.candidates()
    }

    /// Returns an iterator over all candidates
    /// In case the cell contains a digit, this returns no candidates otherwise
    /// the list of candidates
    pub fn candidates_vec(&self) -> Vec<u8> {
        self.state.candidates_vec()
    }

    /// Returns true if the state is valid digit
    pub fn is_digit(&self) -> bool {
        self.state.is_digit()
    }

    /// Returns true if state does not contain a digit
    pub fn is_empty(&self) -> bool {
        self.state.is_empty()
    }

    /// Unsets the cell to an empty one without candidates
    pub fn unset(&mut self) {
        self.state.unset();
    }

    /// Returns the associated index with this cell
    pub fn index(&self) -> usize {
        self.index
    }
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
