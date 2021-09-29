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

                if neighbors.len() >= 2 {
                    // find the pincers, the cells that share candidates x, y each
                    for tuple in neighbors.iter().permutations(2) {
                        if let Some((&lhs, &rhs)) = tuple.iter().collect_tuple() {
                            let shared = lhs.candidates() & rhs.candidates();
                            if lhs.has_candidate(x) && rhs.has_candidate(y) && shared.count() == 1 {
                                if let Some(candidate) = shared.iter().nth(0) {
                                    let eliminates: Vec<&&Cell> = empty_cells
                                        .iter()
                                        .filter(|&cell| cell.index() != pivot.index())
                                        .filter(|&cell| !cell.sees(&pivot))
                                        .filter(|&cell| cell.sees(lhs) && cell.sees(rhs))
                                        .filter(|&cell| cell.has_candidate(candidate))
                                        .collect_vec();

                                    if !eliminates.is_empty() {
                                        let mut step = Step::new();

                                        eliminates.iter().for_each(|&cell| step.eliminate_candidate(cell.index(), candidate));
                                        [pivot, lhs, rhs].iter().for_each(|&cell| {
                                            cell.candidates().iter().for_each(|candidate| step.lock_candidate(cell.index(), candidate));
                                        });

                                        return Some(step);
                                    }
                                }
                            }
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

    use crate::{Sudoku, strategy::Strategy};

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

        let mut sudoku = Sudoku::try_from(sudoku).unwrap();
        sudoku.init_candidates();
        let strategy = YWing::new();

        let step = strategy.find(&sudoku).unwrap();
        assert_eq!(&vec![(65, 4)], step.eliminated_candidates());
        assert_eq!(
            &vec![
                (1, 3),
                (1, 8),
                (73, 3),
                (73, 4),
                (11, 4),
                (11, 8),
            ],
            step.locked_candidates(),
        );
    }
}
