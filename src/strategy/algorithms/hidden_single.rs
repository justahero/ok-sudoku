use itertools::Itertools;

use crate::{
    strategy::{step::Step, Strategy},
    Cell, Sudoku,
};

#[derive(Debug)]
pub struct HiddenSingle {}

impl HiddenSingle {
    pub fn new() -> Self {
        Self {}
    }

    fn find_single(sudoku: &Sudoku, cells: &Vec<&Cell>) -> Option<Step> {
        // Map all candidates to their associated cells
        let candidates = cells
            .iter()
            .flat_map(|&cell| cell.candidates().iter().map(|c| (c, cell)).collect_vec())
            .into_group_map();

        // Find the candidate that is in a single cell
        if let Some((&digit, indexes)) = candidates.iter().find(|&(_, list)| list.len() == 1) {
            let cell = indexes[0];

            let mut step = Step::new();
            step.set_digit(cell.index(), digit);

            // eliminate all other candidates from same cell
            for c in cell.candidates().iter() {
                if c != digit {
                    step.eliminate_candidate(cell.index(), c);
                }
            }

            // eliminate all candidates from same house
            sudoku
                .get_house(cell.index())
                .filter(|&cell| !cells.contains(&cell))
                .filter(|&cell| cell.has_candidate(digit))
                .for_each(|cell| step.eliminate_candidate(cell.index(), digit));

            return Some(step);
        }

        None
    }
}

impl Strategy for HiddenSingle {
    fn find(&self, sudoku: &Sudoku) -> Option<Step> {
        for row in sudoku.get_rows() {
            if let Some(step) = Self::find_single(sudoku, &row) {
                return Some(step);
            }
        }

        for col in sudoku.get_cols() {
            if let Some(step) = Self::find_single(sudoku, &col) {
                return Some(step);
            }
        }

        for block in sudoku.get_blocks() {
            if let Some(step) = Self::find_single(sudoku, &block) {
                return Some(step);
            }
        }

        None
    }

    fn name(&self) -> String {
        String::from("Hidden Single")
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use crate::{strategy::Strategy, Sudoku};

    use super::HiddenSingle;

    #[test]
    fn hidden_single_found() {
        let sudoku = r"
            .28..7...
            .16.83.7.
            ....2.851
            13729....
            ...73....
            ....463.7
            29..7....
            ...86.14.
            ...3..7..
        ";

        let sudoku = Sudoku::try_from(sudoku).unwrap();
        let strategy = HiddenSingle::new();

        let step = strategy.find(&sudoku).unwrap();
        assert_eq!(Some(&(21usize, 6u8)), step.digit());
        assert_eq!(
            &vec![(21usize, 4u8), (21usize, 9u8), (3usize, 6)],
            step.eliminated_candidates()
        );
    }

    #[test]
    fn hidden_single_eliminates_candidates_in_house() {
        let sudoku = r"
            4.....8.5
            .3.......
            ...7.....
            .2.....6.
            ....8.4..
            ....1....
            ...6.3.7.
            5..2.....
            1.4......
        ";

        let mut sudoku = Sudoku::try_from(sudoku).unwrap();
        sudoku.get_mut(46).unset_candidate(4);
        let strategy = HiddenSingle::new();

        let step = strategy.find(&sudoku).unwrap();
        assert_eq!(Some(&(65_usize, 3_u8)), step.digit());
        assert_eq!(
            &vec![
                (65_usize, 6_u8),
                (65_usize, 7_u8),
                (65_usize, 8_u8),
                (65_usize, 9_u8),
                (29_usize, 3_u8),
                (38_usize, 3_u8),
                (47_usize, 3_u8),
                (69_usize, 3_u8),
                (70_usize, 3_u8),
                (71_usize, 3_u8),
            ],
            step.eliminated_candidates()
        );
    }

    #[test]
    fn hidden_single_not_found() {
        let sudoku = r"
            .28..7...
            .16.83.7.
            ...62.851
            13729....
            ...73....
            ....463.7
            29..7....
            ...86.14.
            ...3..7..
        ";

        let mut sudoku = Sudoku::try_from(sudoku).unwrap();
        let strategy = HiddenSingle::new();

        assert_eq!(None, strategy.find(&sudoku));
    }
}
