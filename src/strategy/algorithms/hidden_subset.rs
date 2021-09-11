use crate::{Cell, Sudoku, strategy::{Strategy, step::Step}};

#[derive(Debug)]
pub struct HiddenSubset {
    count: usize,
}

impl HiddenSubset {
    pub fn pair() -> Self {
        Self { count: 2 }
    }

    fn find_tuple(_cells: &Vec<&Cell>) -> Option<Step> {
        None
    }
}

impl Strategy for HiddenSubset {
    fn find(&self, sudoku: &Sudoku) -> Option<Step> {
        for row in sudoku.get_rows() {
            if let Some(step) = Self::find_tuple(&row) {
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
}
