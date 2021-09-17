use itertools::Itertools;

use crate::{
    strategy::{step::Step, Strategy},
    Cell, Sudoku,
};

pub struct LockedCandidate {}

impl LockedCandidate {
    pub fn new() -> Self {
        Self {}
    }

    /// Finds candidate that is confined to a row or column in a single block.
    ///
    /// The algorithm works as follows
    /// * for a list of cells (a row or column) it is checked that the cells that are shared in the same block (3 contiguous cells)
    ///   contain at least a pair of the same candidate (pointing)
    /// * it is checked no other row or column of the same block contains the candidate
    /// * then it is checked the same candidate is present in the same row / column in other blocks
    /// * when all these conditions are met, the candidates in the other blocks of the same row or column can be eliminated
    ///
    fn find_locked_candidate(&self, sudoku: &Sudoku, cells: &Vec<&Cell>) -> Option<Step> {
        assert!(cells.len() == 9);

        // get all mini rows / mini cols
        let groups = [&cells[0..3], &cells[3..6], &cells[6..9]];

        for candidate in 1_u8..=9 {
            /*
            // filter cells with the candidate in same group
            let cells = &groups
                .iter()
                .map(|&group| {
                    group
                        .iter()
                        .filter(|&&cell| cell.has_candidate(candidate))
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();

            // check that one line contains at least two entries, while the other are empty
            // let (list, others): (Vec<_>, Vec<_>) = cells.iter().partition(|&group| group.len() >= 2);
            */
        }

        None
    }
}

impl Strategy for LockedCandidate {
    fn find(&self, sudoku: &Sudoku) -> Option<Step> {
        for row in sudoku.get_rows() {
            if let Some(step) = self.find_locked_candidate(sudoku, &row) {
                return Some(step);
            }
        }

        None
    }

    fn name(&self) -> String {
        String::from("Locked Candidate")
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use crate::{strategy::Strategy, Sudoku};

    use super::LockedCandidate;

    #[test]
    fn find_locked_candidate() {
        let sudoku = r"
            984......
            ..25...4.
            ..19.4..2
            ..6.9723.
            ..36.2...
            2.9.3561.
            195768423
            427351896
            638..9751
        ";

        let mut sudoku = Sudoku::try_from(sudoku).unwrap();
        sudoku.init_candidates();
        let strategy = LockedCandidate::new();

        let step = strategy.find(&sudoku);
        assert!(step.is_some());

        let step = step.unwrap();
        assert_eq!(
            &vec![(18_usize, 5), (19_usize, 5)],
            step.locked_candidates()
        );
        assert_eq!(&vec![(24_usize, 5)], step.eliminated_candidates());
    }
}
