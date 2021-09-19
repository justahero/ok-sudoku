use crate::{
    strategy::{step::Step, Strategy},
    types::Index,
    Sudoku,
};

use super::find_locked;

pub struct PointingTuple {}

impl<'a> PointingTuple {
    pub fn new() -> Self {
        Self {}
    }
}

impl Strategy for PointingTuple {
    fn find(&self, sudoku: &Sudoku) -> Option<Step> {
        for cells in sudoku.get_blocks() {
            let lines = [&cells[0..3], &cells[3..6], &cells[6..9]];

            if let Some(step) = find_locked(&lines, |index| {
                Box::new(sudoku.get_row(Index::from(index).row()))
            }) {
                return Some(step);
            }
        }

        for cells in sudoku.get_blocks() {
            let lines = [
                &vec![cells[0], cells[3], cells[6]][0..3],
                &vec![cells[1], cells[4], cells[7]][0..3],
                &vec![cells[2], cells[5], cells[8]][0..3],
            ];

            if let Some(step) = find_locked(&lines, |index| {
                Box::new(sudoku.get_col(Index::from(index).col()))
            }) {
                return Some(step);
            }
        }

        None
    }

    fn name(&self) -> String {
        String::from("Pointing Tuple")
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use crate::{strategy::Strategy, Sudoku};

    use super::PointingTuple;

    // Example: http://hodoku.sourceforge.net/en/show_example.php?file=lc101&tech=Locked+Candidates+Type+1+%28Pointing%29
    #[test]
    fn pointing_tuple_in_row_found() {
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
        let strategy = PointingTuple::new();

        let step = strategy.find(&sudoku).expect("Should return step");
        assert_eq!(
            &vec![(18_usize, 5), (19_usize, 5)],
            step.locked_candidates(),
        );
        assert_eq!(&vec![(24_usize, 5)], step.eliminated_candidates());
    }

    // Example: https://www.free-sudoku-puzzle.net/techniques/locked-candidate-pointing?example=3
    #[test]
    fn pointing_tuple_in_col_found() {
        let sudoku = r"
            957842136
            .1.97....
            ..41...9.
            64.59..12
            ....61...
            .91..86..
            38..194..
            4.9.8..21
            1....4.6.
        ";

        let mut sudoku = Sudoku::try_from(sudoku).unwrap();
        sudoku.init_candidates();
        // remove a few candidates to reflect linked Sudoku example exactly
        sudoku.get_mut(68).unset_candidate(6);
        sudoku.get_mut(38).unset_candidate(5);
        let strategy = PointingTuple::new();

        let step = strategy.find(&sudoku).expect("Should return step");
        assert_eq!(
            &vec![(64_usize, 7), (73_usize, 7)],
            step.locked_candidates(),
        );
        assert_eq!(&vec![(37_usize, 7)], step.eliminated_candidates());
    }
}
