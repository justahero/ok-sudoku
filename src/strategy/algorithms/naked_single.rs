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
            .find(|cell| cell.candidates().count() == 1);

        result.map(|cell| {
            let candidate = cell.candidates_vec()[0];
            let mut step = Step::new();
            step.set_digit(cell.index(), candidate);
            step
        })
    }

    fn name(&self) -> String {
        String::from("Naked Single")
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

    #[test]
    fn find_naked_single_examples() {
        let sudokus = [
            r"...26.7.1 68..7..9. 19...45.. 82.1...4 ...46.29. ..5...3.2 8..93... 74.4..5.. 367.3.18..."
        ];

        let strategy = NakedSingle::new();
        for &sudoku in sudokus.iter() {
            let mut sudoku = Sudoku::try_from(sudoku).unwrap();
            sudoku.init_candidates();
            assert!(strategy.find(&sudoku).is_some());
        }
    }

    /// Example Expert #18 from Good Sudoku
    #[test]
    fn finds_naked_single_not_found() {
        let sudoku = r"
            ..4...2..
            76...3...
            9.....75.
            ...7.831.
            .....9...
            .321.68..
            ..5.....8
            ...9...34
            ..7...1..
        ";

        let mut sudoku = Sudoku::try_from(sudoku).unwrap();
        sudoku.init_candidates();
        let strategy = NakedSingle::new();

        assert_eq!(None, strategy.find(&sudoku));
    }
}
