use crate::{Sudoku, strategy::{step::Step, Strategy}};

use super::find_fish;

pub struct Jellyfish {}

impl Jellyfish {
    pub fn new() -> Self {
        Self {}
    }
}

impl Strategy for Jellyfish {
    fn find(&self, sudoku: &Sudoku) -> Option<Step> {
        if let Some(step) = find_fish(4, sudoku, |c| c.row(), |c| c.col()) {
            return Some(step);
        }
        if let Some(step) = find_fish(4, sudoku, |c| c.col(), |c| c.row()) {
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

    use crate::{Sudoku, strategy::{Strategy, algorithms::Jellyfish}};

    /// See https://www.sudokuwiki.org/Jelly_Fish_Strategy
    #[test]
    fn find_jellyfish() {
        let sudoku = r"
            ..17538..
            .5......7
            7..89.1..
            ...6.157.
            625478931
            .179.54..
            ....67..4
            .7.....1.
            ..63.97..
        ";

        let mut sudoku = Sudoku::try_from(sudoku).unwrap();
        sudoku.init_candidates();
        let strategy = Jellyfish::new();

        let step = strategy.find(&sudoku).unwrap();
        assert_eq!(
            &vec![
                (9, 2),
                (13, 2),
                (16, 2),
                (25, 2),
                (26, 2),
                (54, 2),
                (61, 2),
                (63, 2),
                (67, 2),
                (71, 2),
            ],
            step.eliminated_candidates()
        );
        assert_eq!(
            &vec![
                (0, 2),
                (7, 2),
                (8, 2),
                (31, 2),
                (35, 2),
                (49, 2),
                (52, 2),
                (53, 2),
                (72, 2),
                (76, 2),
                (79, 2),
                (80, 2),
            ],
            step.locked_candidates(),
        );
    }
}
