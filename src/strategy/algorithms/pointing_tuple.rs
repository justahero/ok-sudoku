use crate::{Sudoku, strategy::{Strategy, step::Step}};

pub struct PointingTuple {}

impl PointingTuple {
    pub fn new() -> Self {
        Self {}
    }
}

impl Strategy for PointingTuple {
    fn find(&self, _sudoku: &Sudoku) -> Option<Step> {
        todo!()
    }

    fn name(&self) -> String {
        String::from("Pointing Tuple")
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use crate::{Sudoku, strategy::Strategy};

    use super::PointingTuple;

    // Example: http://hodoku.sourceforge.net/en/show_example.php?file=lc101&tech=Locked+Candidates+Type+1+%28Pointing%29
    #[test]
    fn pointing_tuple_found() {
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
}
