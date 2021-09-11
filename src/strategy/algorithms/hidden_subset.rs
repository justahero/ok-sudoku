use std::collections::HashMap;

use bit_vec::BitVec;
use itertools::Itertools;

use crate::{Cell, Sudoku, strategy::{Strategy, step::Step}};

/// A Bit Vec indicating which indexes a candidate is in.
#[derive(Debug, Clone, PartialEq, Eq)]
struct IndexVec(BitVec);

impl IndexVec {
    pub fn new() -> Self {
        Self(BitVec::from_elem(82, false))
    }

    pub fn set(&mut self, index: u8) {
        self.0.set(index as usize, true);
    }
}

#[derive(Debug)]
pub struct HiddenSubset {
    count: usize,
}

impl HiddenSubset {
    pub fn pair() -> Self {
        Self { count: 2 }
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

        let group = candidates.iter().permutations(self.count).find(|entries| {
            entries.iter().map(|(_, indexes)| indexes).all_equal()
        });

        dbg!(&group);

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

        for col in sudoku.get_blocks() {
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

    use crate::{Sudoku, strategy::Strategy};

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
    }

    #[test]
    fn find_hidden_subset_pair() {
        let sudoku = r"
            .49132...
            .81479...
            327685914
            .96.51.8.
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
    }
}
