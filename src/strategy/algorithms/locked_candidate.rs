use crate::{
    strategy::{step::Step, Strategy},
    types::Index,
    Sudoku,
};
use super::find_locked;

pub struct LockedCandidate {}

impl LockedCandidate {
    pub fn new() -> Self {
        Self {}
    }
}

impl Strategy for LockedCandidate {
    fn find(&self, sudoku: &Sudoku) -> Option<Step> {
        for row in sudoku.get_rows() {
            let lines = [&row[0..3], &row[3..6], &row[6..9]];

            if let Some(step) = find_locked(&lines, |index| {
                Box::new(sudoku.get_block(Index::from(index).block()))
            }) {
                return Some(step);
            }
        }

        for col in sudoku.get_cols() {
            let lines = [&col[0..3], &col[3..6], &col[6..9]];

            if let Some(step) = find_locked(&lines, |index| {
                Box::new(sudoku.get_block(Index::from(index).block()))
            }) {
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
    fn locked_candidate_row_found() {
        let sudoku = r"
            318..54.6
            ...6.381.
            ..6.8.5.3
            864952137
            123476958
            795318264
            .3.5..78.
            .....73.5
            ....39641
        ";

        let mut sudoku = Sudoku::try_from(sudoku).unwrap();
        sudoku.init_candidates();
        let strategy = LockedCandidate::new();

        let step = strategy.find(&sudoku).unwrap();
        assert_eq!(
            &vec![(10_usize, 7), (11_usize, 7)],
            step.locked_candidates()
        );
        assert_eq!(&vec![(19_usize, 7)], step.eliminated_candidates());
    }

    #[test]
    fn locked_candidate_column_found() {
        let sudoku = r"
            762..8..1
            98......6
            15.....87
            478..3169
            526..9873
            3198..425
            835..1692
            297685314
            641932758
        ";

        let mut sudoku = Sudoku::try_from(sudoku).unwrap();
        sudoku.init_candidates();

        let strategy = LockedCandidate::new();

        let step = strategy.find(&sudoku).unwrap();
        assert_eq!(
            &vec![(14_usize, 4), (23_usize, 4)],
            step.locked_candidates()
        );
        assert_eq!(
            &vec![
                (3_usize, 4),
                (4_usize, 4),
                (12_usize, 4),
                (13_usize, 4),
                (21_usize, 4),
                (22_usize, 4)
            ],
            step.eliminated_candidates(),
        );
    }

    #[test]
    fn locked_candidate_not_in_grid() {
        // Sudoku from http://hodoku.sourceforge.net/en/show_example.php?file=lc101&tech=Locked+Candidates+Type+1+%28Pointing%29
        // It's a different locked candidate strategy and should not be found by the algorithm
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

        // remove the existing locked candidates
        sudoku.get_mut(24).unset_candidate(5);
        sudoku.get_mut(40).unset_candidate(8);

        let strategy = LockedCandidate::new();
        assert_eq!(None, strategy.find(&sudoku));
    }
}
