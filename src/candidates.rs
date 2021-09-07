use std::ops::BitOrAssign;

use bit_vec::BitVec;

/// Candidates Bitset
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
    pub fn count(&self) -> usize {
        self.0.iter().filter(|v| *v).count()
    }

    /// Returns the intersection of two bit sets
    pub fn intersect(lhs: &Self, rhs: &Self) -> Self {
        let mut lhs = lhs.clone();
        lhs.0.and(&rhs.0);
        lhs
    }

    /// Returns the difference of two bit sets
    pub fn difference(lhs: &Self, rhs: &Self) -> Self {
        let mut lhs = lhs.clone();
        lhs.0.difference(&rhs.0);
        lhs
    }

    /// Returns a new Candidates list containing all entries from both sets
    pub fn or(&mut self, rhs: &Self) -> bool {
        self.0.or(&rhs.0)
    }
}

impl BitOrAssign for Candidates {
    fn bitor_assign(&mut self, rhs: Self) {
        self.or(&rhs);
    }
}
