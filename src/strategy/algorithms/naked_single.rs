use crate::{Sudoku, strategy::{Strategy, step::Step}};

/// TODO hold data?
pub struct NakedSingle {
    // TODO?
}

impl NakedSingle {
    pub fn new() -> Self {
        Self {}
    }
}

impl Strategy for NakedSingle {
    fn find(&self, sudoku: &Sudoku) -> Option<Step> {
        let result = sudoku
            .iter()
            .find(|(_index, cell)| {
                if let Some(candidates) = cell.candidates() {
                    candidates.count() == 1
                } else {
                    false
                }
            });

        result.map(|(_index, _cell)| {
            Step::new()
        })
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use crate::{Sudoku, strategy::Strategy};

    use super::NakedSingle;

    /// Example taken from: http://hodoku.sourceforge.net/en/tech_singles.php
    #[test]
    fn finds_naked_single() {
        let sudoku = r"
            .1.9..74.
            ...8....3
            .7.32.69.
            ..4.3.2..
            ...6.2...
            ..8.1.3..
            .81.7..3.
            3....8...
            .69..3.2.
        ";

        let mut sudoku = Sudoku::try_from(sudoku).unwrap();
        sudoku.init_candidates();
        let strategy = NakedSingle::new();

        let result = strategy.find(&sudoku.into());
        assert!(result.is_some());
    }
}
