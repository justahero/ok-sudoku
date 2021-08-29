use crate::{Sudoku, strategy::{Strategy, steps::Steps}};

pub struct NakedSingle {
    // TODO?
}

impl NakedSingle {
    pub fn new() -> Self {
        Self {}
    }
}

impl Strategy for NakedSingle {
    fn find(&self, sudoku: &Sudoku) -> Option<Steps> {
        None
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

        let sudoku = Sudoku::try_from(sudoku).unwrap();
        let strategy = NakedSingle::new();

        let result = strategy.find(&sudoku);
        assert!(result.is_some());
    }
}
