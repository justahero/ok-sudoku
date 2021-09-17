use std::collections::HashMap;

use crate::{Cell, Sudoku, strategy::{step::Step, Strategy}};

#[derive(Debug)]
pub struct HiddenSingle {}

impl HiddenSingle {
    pub fn new() -> Self {
        Self {}
    }

    fn find_single(sudoku: &Sudoku, cells: &Vec<&Cell>) -> Option<Step> {
        // Map all candidates to cells
        let candidates = cells.iter().fold(HashMap::new(), |mut map, &cell| {
            for candidate in cell.candidates_vec() {
                if map.get(&candidate).is_none() {
                    map.insert(candidate, vec![]);
                }
                if let Some(entries) = map.get_mut(&candidate) {
                    entries.push(cell.index());
                }
            }
            map
        });

        // Find the candidate that is in a single cell
        if let Some((&digit, cells)) = candidates.iter().find(|&(_, list)| list.len() == 1) {
            let cell_index = *cells.first().unwrap();
            let mut step = Step::new();
            step.set_digit(cell_index, digit);

            println!("::: CANDIDATES: {:?}", sudoku.get(cell_index));
            for c in sudoku.get(cell_index).candidates().iter() {
                if c != digit {
                    step.eliminate_candidate(cell_index, c);
                }
            }
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

        let mut sudoku = Sudoku::try_from(sudoku).unwrap();
        sudoku.init_candidates();
        let strategy = HiddenSingle::new();

        let step = strategy.find(&sudoku).unwrap();
        assert_eq!(Some(&(21usize, 6u8)), step.digit());
        assert_eq!(
            &vec![(21usize, 4u8), (21usize, 9u8)],
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
        sudoku.init_candidates();
        let strategy = HiddenSingle::new();

        assert_eq!(None, strategy.find(&sudoku));
    }
}
