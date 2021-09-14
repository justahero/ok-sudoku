use std::{collections::HashMap, fmt::Debug};

use bit_vec::BitVec;
use itertools::Itertools;

use crate::{
    strategy::{step::Step, Strategy},
    Cell, Sudoku,
};

/// A Bit Vec indicating which indexes a candidate is in.
#[derive(Clone, PartialEq, Eq)]
struct IndexVec(BitVec);

impl IndexVec {
    pub fn new() -> Self {
        Self(BitVec::from_elem(82, false))
    }

    pub fn set(&mut self, index: u8) {
        self.0.set(index as usize, true);
    }

    pub fn is_set(&self, index: u8) -> bool {
        self.0.get(index as usize).is_some()
    }

    /// Returns the number of set indexes
    pub fn count(&self) -> u8 {
        self.0.iter().filter(|v| *v).count() as u8
    }

    /// Returns an iterator over all indexes
    pub fn iter(&self) -> impl Iterator<Item = u8> + '_ {
        self.0
            .iter()
            .enumerate()
            .filter(|(_index, v)| *v)
            .map(|(index, _)| index as u8)
    }

    /// Returns the intersection of two bit sets
    pub fn intersect(lhs: &Self, rhs: &Self) -> Self {
        let mut lhs = lhs.clone();
        lhs.0.and(&rhs.0);
        lhs
    }
}

impl Debug for IndexVec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({})", self.iter().join(", "))
    }
}

#[derive(Debug)]
pub struct HiddenSubset {
    count: u8,
}

impl HiddenSubset {
    pub fn pair() -> Self {
        Self { count: 2 }
    }

    pub fn triple() -> Self {
        Self { count: 3 }
    }

    pub fn quadruple() -> Self {
        Self { count: 4 }
    }

    fn find_tuple(&self, cells: &Vec<&Cell>) -> Option<Step> {
        // Map all candidates to cell indexes
        let candidates = cells.iter().fold(HashMap::new(), |mut map, &cell| {
            for candidate in cell.candidates_vec() {
                if map.get(&candidate).is_none() {
                    map.insert(candidate, IndexVec::new());
                }
                if let Some(index) = map.get_mut(&candidate) {
                    index.set(cell.index() as u8);
                }
            }
            map
        });

        // Find a group that has the same list of indexes, still contains other candidates to hide within
        let group = candidates
            .iter()
            .filter(|(_index, indexes)| indexes.count() == self.count)
            .permutations(self.count as usize)
            .filter(|entries| entries.len() <= self.count as usize)
            .find(|entries| entries.iter().map(|(_, indexes)| indexes).all_equal());

        if let Some(group) = group {
            // get indexes where hidden subset is found
            let found_indexes = group[0].1.iter().collect::<Vec<_>>();
            let found_candidates = group.iter().map(|(&c, _)| c).collect::<Vec<_>>();

            // get all sorted pairs of (index, candidate)
            let candidates: Vec<(u8, u8)> = candidates
                .iter()
                .filter(|tuple| !found_candidates.contains(tuple.0))
                .flat_map(|(&candidate, indexes)| indexes.iter().map(move |index| (index, candidate)))
                .filter(|(index, _candidate)| found_indexes.contains(&index))
                .sorted_by(|lhs, rhs| lhs.cmp(rhs))
                .collect::<Vec<_>>();

            let mut step = Step::new();
            for (index, candidate) in candidates.iter() {
                step.eliminate_candidate((*index) as usize, *candidate);
            }

            if !candidates.is_empty() {
                return Some(step);
            }
        }

        None
    }
}

impl Strategy for HiddenSubset {
    fn find(&self, sudoku: &Sudoku) -> Option<Step> {
        for row in sudoku.get_rows() {
            if let Some(step) = self.find_tuple(&row) {
                return Some(step);
            }
        }

        for col in sudoku.get_cols() {
            if let Some(step) = self.find_tuple(&col) {
                return Some(step);
            }
        }

        for block in sudoku.get_blocks() {
            if let Some(step) = self.find_tuple(&block) {
                return Some(step);
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use crate::{strategy::Strategy, Sudoku};

    use super::HiddenSubset;

    /// Example of hidden subset: http://hodoku.sourceforge.net/en/show_example.php?file=h202&tech=Hidden+Pair
    #[test]
    fn find_hidden_subset() {
        let sudoku = r"
            ....6....
            ....42736
            ..673..4.
            .94....68
            ....964.7
            6.7.5.923
            1......85
            .6..8.271
            ..5.1..94
        ";

        let mut sudoku = Sudoku::try_from(sudoku).unwrap();
        sudoku.init_candidates();
        let strategy = HiddenSubset::pair();

        let step = strategy.find(&sudoku);
        assert!(step.is_some());

        let step = step.unwrap();
        let expected_eliminated: Vec<(usize, u8)> = vec![
            (0, 2),
            (0, 3),
            (0, 5),
            (0, 8),
            (0, 9),
            (1, 1),
            (1, 2),
            (1, 3),
            (1, 5),
            (1, 8),
        ];
        assert_eq!(&expected_eliminated, step.eliminated_candidates());
    }

    /// Example: http://hodoku.sourceforge.net/en/show_example.php?file=h201&tech=Hidden+Pair
    #[test]
    fn find_hidden_subset_pair() {
        let sudoku = r"
            .49132...
            .81479...
            327685914
            .96.518..
            .75.28...
            .38.46..5
            853267...
            712894563
            964513...
        ";

        let mut sudoku = Sudoku::try_from(sudoku).unwrap();
        sudoku.init_candidates();
        let strategy = HiddenSubset::pair();

        let step = strategy.find(&sudoku);
        assert!(step.is_some());

        let step = step.unwrap();
        assert_eq!(
            &vec![(44_usize, 6_u8)],
            step.eliminated_candidates(),
        );
    }
}
