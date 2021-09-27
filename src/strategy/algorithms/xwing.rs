use crate::{Sudoku, strategy::{step::Step, Strategy}};

use super::find_fish;
pub struct XWing {}

impl XWing {
    pub fn new() -> Self {
        Self {}
    }
}

impl Strategy for XWing {
    fn find(&self, sudoku: &Sudoku) -> Option<Step> {
        if let Some(step) = find_fish(2, sudoku, |c| c.row(), |c| c.col()) {
            return Some(step);
        }
        if let Some(step) = find_fish(2, sudoku, |c| c.col(), |c| c.row()) {
            return Some(step);
        }

        None
    }

    fn name(&self) -> String {
        String::from("XWing (Fish)")
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use crate::{Sudoku, strategy::{Strategy, algorithms::xwing::XWing}};

    /// See example: http://hodoku.sourceforge.net/en/tech_fishb.php
    #[test]
    fn find_xwing_in_rows() {
        let sudoku = r"
            .41729.3.
            760..34.2
            .3264.719
            4.39..17.
            6.7..49.3
            19537..24
            214567398
            376.9.541
            958431267
        ";

        let mut sudoku = Sudoku::try_from(sudoku).unwrap();
        sudoku.init_candidates();
        let strategy = XWing::new();

        let step = strategy.find(&sudoku).unwrap();

        assert_eq!(&vec![(31, 5)], step.eliminated_candidates());
        assert_eq!(
            &vec![(13, 5), (16, 5), (40, 5), (43, 5)],
            step.locked_candidates(),
        );
    }

    /// See example: http://hodoku.sourceforge.net/en/tech_fishb.php
    #[test]
    fn find_xwing_in_columns() {
        let sudoku = r"
            98..62753
            .65..3...
            327.5...6
            79..3.5..
            .5...9...
            832.45..9
            673591428
            249.87..5
            518.2...7
        ";

        let mut sudoku = Sudoku::try_from(sudoku).unwrap();
        sudoku.init_candidates();
        let strategy = XWing::new();

        let step = strategy.find(&sudoku).unwrap();
        assert_eq!(
            &vec![
                (12, 1),
                (15, 1),
                (16, 1),
                (17, 1),
                (38, 1),
                (39, 1),
                (42, 1),
                (43, 1),
                (44, 1),
            ],
            step.eliminated_candidates(),
        );
        assert_eq!(
            &vec![(9, 1), (13, 1), (36, 1), (40, 1)],
            step.locked_candidates(),
        );
    }

    /// A few examples
    #[test]
    fn detect_xwing_in_sudokus() {
        let sudokus = [
            r"5..27134....6.3....3.8.9..7..3..7.....7..8.3.6..31472..8.79...3...3..6.....1..5..",
        ];

        let strategy = XWing::new();
        for sudoku in sudokus.iter() {
            let mut sudoku = Sudoku::try_from(*sudoku).unwrap();
            sudoku.init_candidates();

            assert!(strategy.find(&sudoku).is_some());
        }
    }

    #[test]
    fn ignores_other_fishes() {
        let sudokus = [
            // Swordfish
            r"16.543.7..786.14354358.76.172.458.696..912.57...376..4.16.3..4.3...8..16..71645.3",
        ];

        let strategy = XWing::new();
        for sudoku in sudokus.iter() {
            let mut sudoku = Sudoku::try_from(*sudoku).unwrap();
            sudoku.init_candidates();

            println!("SUDOKU: {}", sudoku);
            let step = strategy.find(&sudoku);
            println!("STEP: {:?}", step);
            assert_eq!(None, step);
        }
    }
}
