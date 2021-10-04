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

        let sudoku = Sudoku::try_from(sudoku).unwrap();
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
                (0, 2.into()),
                (7, 2.into()),
                (8, 2.into()),
                (31, 2.into()),
                (35, 2.into()),
                (49, 2.into()),
                (52, 2.into()),
                (53, 2.into()),
                (72, 2.into()),
                (76, 2.into()),
                (79, 2.into()),
                (80, 2.into()),
            ],
            step.locked_candidates(),
        );
    }

    #[test]
    fn ignores_other_fishes() {
        let sudokus = [
            // Swordfish
            r"16.543.7. .786.1435 4358.76.1 72.458.69 6..912.57 ...376..4 .16.3..4. 3...8..16 ..71645.3",
        ];

        let strategy = Jellyfish::new();
        for sudoku in sudokus.iter() {
            let sudoku = Sudoku::try_from(*sudoku).unwrap();

            println!("SUDOKU: {}", sudoku);
            let step = strategy.find(&sudoku);
            assert_eq!(None, step);
        }
    }
}
