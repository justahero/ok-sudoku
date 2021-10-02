use std::{fmt::{Debug, Formatter, Result}, ops::{BitAnd, BitOr, BitOrAssign, Shl}};

/// Candidates Bitset
#[derive(Clone, PartialEq, Eq)]
pub struct Candidates(u16);

impl Candidates {
    const ALL_MASK: u16 = 0b111111111;

    /// Creates a new candidates with all candidates
    pub fn all() -> Self {
        Candidates(0b111111111)
    }

    /// Creates a new empty candidates set
    pub fn empty() -> Self {
        Candidates(0b000000000)
    }

    /// Crates a new empty candidates set from value
    pub fn from(bits: u16) -> Self {
        Candidates(bits)
    }

    /// Creates a new Candidates set from a list of given candidate values
    pub fn new(candidates: &[u8]) -> Self {
        let mut result = Candidates::empty();
        candidates.iter().for_each(|&c| result.set(c));
        result
    }

    /// Returns true if the candidates set is empty
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        (self.0 & Self::ALL_MASK) == 0
    }

    /// Sets the given candidate
    #[inline(always)]
    pub fn set(&mut self, candidate: u8) {
        assert!(candidate >= 1);
        self.0 |= 1_u16.shl(candidate - 1)
    }

    /// Unsets the given candidate
    #[inline(always)]
    pub fn unset(&mut self, candidate: u8) {
        assert!(candidate >= 1);
        self.0 &= !1_u16.shl(candidate - 1);
    }

    /// Returns true if candidate is set
    #[inline(always)]
    pub fn get(&self, candidate: u8) -> bool {
        assert!(candidate >= 1);
        self.0 & 1_u16.shl(candidate - 1) > 0
    }

    /// Returns an iterator over all set candidates
    pub fn iter(&self) -> impl Iterator<Item = u8> + '_ {
        (0u16..9)
            .filter(move |index| (self.0 & 1u16.shl(index) > 0))
            .map(|index| (index + 1) as u8)
    }

    /// Returns the number of set candidates
    #[inline(always)]
    pub fn count(&self) -> usize {
        self.0.count_ones() as usize
    }

    /// Returns the intersection of two bit sets
    #[inline(always)]
    pub fn intersect(lhs: &Self, rhs: &Self) -> Self {
        let mut lhs = lhs.clone();
        lhs.0 &= rhs.0;
        lhs
    }
}

impl From<u8> for Candidates {
    fn from(candidate: u8) -> Self {
        Candidates::from(1_u16.shl(candidate - 1))
    }
}

impl Debug for Candidates {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let candidates = self.iter().collect::<Vec<_>>();
        write!(f, "{:?}", candidates)
    }
}

impl BitOr for Candidates {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Candidates(self.0 | rhs.0)
    }
}

impl BitOrAssign for Candidates {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl BitAnd for Candidates {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Candidates(self.0 & rhs.0)
    }
}

#[cfg(test)]
mod tests {
    use crate::Candidates;

    #[test]
    fn is_empty() {
        let candidates = Candidates::empty();
        assert!(candidates.is_empty());
    }

    #[test]
    fn all_candidates_set() {
        let candidates = Candidates::all();
        assert!(!candidates.is_empty());
        assert_eq!(9, candidates.count());
    }

    #[test]
    fn from_candidates() {
        let candidates = Candidates::from(0b0110011);
        assert!(!candidates.is_empty());
        assert_eq!(4, candidates.count());
    }

    #[test]
    fn set_single_candidate() {
        let mut candidates = Candidates::empty();
        candidates.set(4);
        assert!(!candidates.is_empty());
        assert!(candidates.get(4));
        assert_eq!(1, candidates.count());
    }

    #[test]
    fn unset_single_candidate() {
        let mut candidates = Candidates::from(0b000111000);
        candidates.unset(5);
        assert!(!candidates.is_empty());
        assert_eq!(2, candidates.count());
        assert_eq!(
            vec![4, 6],
            candidates.iter().collect::<Vec<_>>(),
        );
    }

    #[test]
    fn unset_same_bit_multiple_times() {
        let mut candidates = Candidates::from(0b000111000);
        candidates.unset(5);
        candidates.unset(5);
        assert!(!candidates.is_empty());
        assert_eq!(2, candidates.count());
        assert_eq!(
            vec![4, 6],
            candidates.iter().collect::<Vec<_>>(),
        );
    }

    #[test]
    fn set_multiple_candidates() {
        let mut candidates = Candidates::empty();
        candidates.set(1);
        candidates.set(3);
        candidates.set(9);

        assert_eq!(3, candidates.count());
        assert!(candidates.get(3));
        assert!(!candidates.get(4));
    }

    #[test]
    fn candidates_iter() {
        let mut candidates = Candidates::empty();
        candidates.set(1);
        candidates.set(3);
        candidates.set(9);

        assert_eq!(
            vec![1, 3, 9],
            candidates.iter().collect::<Vec<_>>(),
        );
    }

    #[test]
    fn intersect_candidates() {
        let lhs = Candidates::from(0b101101);
        let rhs = Candidates::from(0b011100);

        let result = Candidates::intersect(&lhs, &rhs);
        assert_eq!(2, result.count());
        assert_eq!(
            vec![3, 4],
            result.iter().collect::<Vec<_>>(),
        );
    }

    #[test]
    fn candidates_or() {
        let lhs = Candidates::from(0b111000);
        let rhs = Candidates::from(0b011010);

        let result = lhs | rhs;
        assert_eq!(4, result.count());
        assert_eq!(
            vec![2, 4, 5, 6],
            result.iter().collect::<Vec<_>>(),
        );
    }
}
