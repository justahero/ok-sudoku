use crate::{Sudoku, strategy::{step::Step, Strategy}};

use super::find_fish;

pub struct Swordfish {}

impl Swordfish {
    pub fn new() -> Self {
        Self {}
    }
}

impl Strategy for Swordfish {
    fn find(&self, sudoku: &Sudoku) -> Option<Step> {
        if let Some(step) = find_fish(3, sudoku, |c| c.row(), |c| c.col()) {
            return Some(step);
        }
        if let Some(step) = find_fish(3, sudoku, |c| c.col(), |c| c.row()) {
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

    use crate::{
        strategy::{algorithms::Swordfish, Strategy},
        Sudoku,
    };

    /// Example found here: http://hodoku.sourceforge.net/en/show_example.php?file=bf301&tech=Swordfish
    #[test]
    fn find_swordfish() {
        let sudoku = r"
            16.543.7.
            .786.1435
            4358.76.1
            72.458.69
            6..912.57
            ...376..4
            .16.3..4.
            3...8..16
            ..71645.3
        ";

        let mut sudoku = Sudoku::try_from(sudoku).unwrap();
        sudoku.init_candidates();
        let strategy = Swordfish::new();

        let step = strategy.find(&sudoku).unwrap();

        assert_eq!(&vec![(52, 2), (54, 2)], step.eliminated_candidates());
        assert_eq!(
            &vec![(9, 2), (13, 2), (22, 2), (25, 2), (72, 2), (79, 2),],
            step.locked_candidates(),
        );
    }

    #[test]
    fn find_swordfish_in_multiple_sudokus() {
        let sudokus = [
            r"926...1.. 537.1.42. 841...6.3 259734816 714.6..3. 36812..4. 1.2....82 485.7136. 6.3.....1",
            r".2..43.69 ..38962.. 96..25.3. 89.56..13 6...3.... .3..81.26 3...1..7. ..96743.2 27.358.9.",
        ];

        let strategy = Swordfish::new();
        for sudoku in sudokus.iter() {
            let mut sudoku = Sudoku::try_from(*sudoku).unwrap();
            sudoku.init_candidates();

            assert!(strategy.find(&sudoku).is_some());
        }
    }

    /// This grid is taken from: https://www.youtube.com/watch?v=9m9t8ie9-EE
    #[test]
    fn ignores_xwings() {
        let sudokus =
            [r"5..27134....6.3....3.8.9..7..3..7.....7..8.3.6..31472..8.79...3...3..6.....1..5.."];

        let strategy = Swordfish::new();
        for sudoku in sudokus.iter() {
            let mut sudoku = Sudoku::try_from(*sudoku).unwrap();
            sudoku.init_candidates();

            assert!(strategy.find(&sudoku).is_none());
        }
    }
}
