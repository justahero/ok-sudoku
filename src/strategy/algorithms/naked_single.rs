use crate::{Sudoku, strategy::{Strategy, step::Step}};

#[derive(Debug)]
pub struct NakedSingle {}

impl NakedSingle {
    pub fn new() -> Self {
        Self {}
    }
}

impl Strategy for NakedSingle {
    fn find(&self, sudoku: &Sudoku) -> Option<Step> {
        let result = sudoku
            .iter()
            .find(|(_index, cell)| cell.candidates().count() == 1);

        result.map(|(index, cell)| {
            let candidate = cell.candidates_vec()[0];
            let mut step = Step::new();
            step.set_digit(index, candidate);
            step
        })
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use crate::{Sudoku, strategy::Strategy};
    use super::NakedSingle;

    /// Example taken from:
    /// http://hodoku.sourceforge.net/en/show_example.php?file=n102&tech=Naked+Single
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

        let step = strategy.find(&sudoku).unwrap();
        assert!(step.has_digit());
        assert_eq!((20_usize, 5_u8), *step.digit().unwrap());
    }
}
