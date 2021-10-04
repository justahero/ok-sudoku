use itertools::Itertools;

use crate::{Cell, Sudoku, strategy::{Strategy, step::Step}};

pub struct YWing {}

impl YWing {
    pub fn new() -> Self {
        Self {}
    }
}

impl Strategy for YWing {    
    fn find(&self, sudoku: &Sudoku) -> Option<Step> {
        // get all cells that contain exactly 2 candidates
        let empty_cells = sudoku.iter().filter(|&cell| cell.is_empty()).collect_vec();
        let pivots = empty_cells.iter().filter(|&cell| cell.candidates().count() == 2).collect_vec();

        // The pivot is the bivalue cell with candidates X, Y
        for pivot in pivots.iter() {
            if let Some((x, y)) = pivot.candidates().iter().collect_tuple() {
                // find all neighbors that see the given pivot cell
                let neighbors = pivots
                    .iter()
                    .filter(|&cell| cell.index() != pivot.index())
                    .filter(|&cell| cell.sees(&pivot))
                    .collect_vec();

                if neighbors.len() < 2 {
                    continue;
                }

                // find the pincers, the cells that share candidates x, y each
                for tuple in neighbors.iter().permutations(2) {
                    if let Some((&lhs, &rhs)) = tuple.iter().collect_tuple() {
                        if lhs.sees(rhs) {
                            continue;
                        }

                        if !(lhs.has_candidate(x) && rhs.has_candidate(y)) {
                            continue;
                        }

                        // ensure there is the same remaining candidate in both cells
                        let shared = lhs.candidates() & rhs.candidates();
                        if shared.count() != 1 {
                            continue;
                        }

                        let z = shared.iter().next().expect("Failed to get first item");
                        if pivot.has_candidate(z) {
                            continue;
                        }

                        let eliminates: Vec<&&Cell> = empty_cells
                            .iter()
                            .filter(|&cell| cell.index() != pivot.index())
                            .filter(|&cell| cell.sees(lhs) && cell.sees(rhs))
                            .filter(|&cell| cell.has_candidate(z))
                            .collect_vec();

                        if !eliminates.is_empty() {
                            let mut step = Step::new();

                            eliminates.iter().for_each(|&cell| step.eliminate_candidate(cell.index(), z));
                            [pivot, lhs, rhs].iter().for_each(|&cell| {
                                step.lock_candidate(cell.index(), cell.candidates());
                            });

                            return Some(step);
                        }
                    }
                }
            }
        }

        None
    }

    fn name(&self) -> String {
        String::from("YWing")
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use crate::{Candidates, Sudoku, strategy::Strategy};

    use super::YWing;

    /// See example: https://www.sudokuwiki.org/Y_Wing_Strategy
    #[test]
    fn find_ywing() {
        let sudoku = r"
            9..24....
            .5.69.231
            .2..5..9.
            .9.7..32.
            ..29356.7
            .7...29..
            .69.2..73
            51..79.62
            2.7.86..9
        ";

        let sudoku = Sudoku::try_from(sudoku).unwrap();
        let strategy = YWing::new();

        let step = strategy.find(&sudoku).unwrap();
        assert_eq!(&vec![(65, 4)], step.eliminated_candidates());
        assert_eq!(
            &vec![
                (1, Candidates::new(&[3, 8])),
                (11, Candidates::new(&[4, 8])),
                (73, Candidates::new(&[3, 4])),
            ],
            step.locked_candidates(),
        );
    }

    /// A more elaborate YWing from "Good Sudoku Daily Arcade Challenge" (3th October 2021)
    #[test]
    fn find_ywing_eliminate_in_pivot_box() {
        let sudoku = r"
            .6..8..3.
            895...7.2
            .........
            179......
            246851.7.
            ..89.712.
            .....8..7
            617.3.58.
            98..7..6.
        ";

        let mut sudoku = Sudoku::try_from(sudoku).unwrap();
        // naked subset (2)
        sudoku.get_mut(53).unset_candidates(&[3, 5]);
        // naked subset (2)
        sudoku.get_mut(33).unset_candidate(3);
        sudoku.get_mut(35).unset_candidate(3);
        // naked subset (4)
        sudoku.get_mut(24).unset_candidates(&[4, 9]);
        sudoku.get_mut(33).unset_candidate(4);
        // naked subset (4)
        sudoku.get_mut(26).unset_candidates(&[1, 4, 5, 9]);
        // naked subset (2)
        sudoku.get_mut(21).unset_candidate(6);
        sudoku.get_mut(22).unset_candidate(6);
        sudoku.get_mut(23).unset_candidate(6);
        // locked candidate
        sudoku.get_mut(21).unset_candidate(3);
        sudoku.get_mut(23).unset_candidate(3);
        // naked subset (4)
        sudoku.get_mut(14).unset_candidate(4);
        sudoku.get_mut(32).unset_candidates(&[2, 4]);
        // locked candidate
        sudoku.get_mut(57).unset_candidate(2);
        sudoku.get_mut(58).unset_candidate(2);
        sudoku.get_mut(75).unset_candidate(2);
        sudoku.get_mut(77).unset_candidate(2);
        // locked candidate
        sudoku.get_mut(57).unset_candidate(5);
        // XWing
        sudoku.get_mut(23).unset_candidate(9);
        sudoku.get_mut(60).unset_candidate(9);
        // naked subset (4)
        sudoku.get_mut(57).unset_candidate(4);
        sudoku.get_mut(58).unset_candidate(4);
        sudoku.get_mut(61).unset_candidate(4);
        let strategy = YWing::new();

        let step = strategy.find(&sudoku).unwrap();
        assert_eq!(&vec![(25, 9)], step.eliminated_candidates());
        assert_eq!(
            &vec![
                (6, Candidates::new(&[4, 9])),
                (16, Candidates::new(&[1, 4])),
                (61, Candidates::new(&[1, 9])),
            ],
            step.locked_candidates(),
        );
    }

    #[test]
    fn find_ywing_in_same_box() {
        let sudoku = r"
            .67918.2.
            ...63..7.
            ..32.7618
            8..1.37..
            3197..862
            7..8.91..
            6.53724.1
            .7..81..6
            .3..962.7
        ";

        let mut sudoku = Sudoku::try_from(sudoku).unwrap();
        sudoku.get_mut(9).unset_candidates(&[4, 5, 9]);
        sudoku.get_mut(10).unset_candidates(&[4, 5, 9]);
        sudoku.get_mut(11).unset_candidates(&[2, 4]);
        sudoku.get_mut(19).unset_candidate(5);

        let strategy = YWing::new();

        let step = strategy.find(&sudoku).unwrap();
        assert_eq!(&vec![(63, 2)], step.eliminated_candidates());
        assert_eq!(
            &vec![
                (9, Candidates::new(&[1, 2])),
                (65, Candidates::new(&[2, 4])),
                (72, Candidates::new(&[1, 4])),
            ],
            step.locked_candidates(),
        );
    }

    #[test]
    fn detect_no_ywing() {
        let sudoku = r"
            ...5...29
            ....821..
            8..9....3
            .512.7.4.
            .........
            .6...125.
            1....8...
            ..514....
            67...5...
        ";

        let sudoku = Sudoku::try_from(sudoku).unwrap();
        let strategy = YWing::new();

        assert_eq!(None, strategy.find(&sudoku));
    }
}
