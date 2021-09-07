use crate::Candidates;

/// A single cell on the board with either a digit or a list of candidates
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
