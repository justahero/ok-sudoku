use itertools::Itertools;

use crate::{Sudoku, strategy::{step::Step, Strategy}, types::Index};

pub struct PointingTuple {}

impl PointingTuple {
    pub fn new() -> Self {
        Self {}
    }
}

impl Strategy for PointingTuple {
    fn find(&self, sudoku: &Sudoku) -> Option<Step> {
        // find single mini row with same candidate in block
        // then check row for candidates, eliminate these
        for cells in sudoku.get_blocks() {
            let lines = [&cells[0..3], &cells[3..6], &cells[6..9]];

            for candidate in 1_u8..=9 {
                // filter cells with the candidate in same group
                let lines = lines
                    .iter()
                    .map(|&group| {
                        group
                            .iter()
                            .filter(|&&cell| cell.has_candidate(candidate))
                            .map(|&cell| cell.index())
                            .collect_vec()
                    })
                    .filter(|group| !group.is_empty())
                    .collect_vec();

                for i in 0..=lines.len() {
                    if let Some((&indexes, others)) = lines
                        .iter()
                        .cycle()
                        .skip(i)
                        .take(lines.len())
                        .collect_vec()
                        .split_first()
                    {
                        if indexes.len() >= 2 && others.len() == 0_usize {
                            // get remaining cells from same row
                            let row = Index::from(indexes[0]).row();
                            let eliminates = sudoku
                                .get_row(row)
                                .filter(|&cell| cell.has_candidate(candidate) && !indexes.contains(&cell.index()))
                                .collect_vec();

                            if eliminates.len() > 0 {
                                let mut step = Step::new();

                                for index in indexes {
                                    step.lock_candidate(*index, candidate)
                                }
                                for neighbor in eliminates {
                                    step.eliminate_candidate(neighbor.index(), candidate)
                                }

                                return Some(step);
                            }
                        }
                    }
                }
            }
        }

        None
    }

    fn name(&self) -> String {
        String::from("Pointing Tuple")
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use crate::{strategy::Strategy, Sudoku};

    use super::PointingTuple;

    // Example: http://hodoku.sourceforge.net/en/show_example.php?file=lc101&tech=Locked+Candidates+Type+1+%28Pointing%29
    #[test]
    fn pointing_tuple_in_row_found() {
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

    #[test]
    fn pointing_tuple_in_col_found() {
        todo!();
    }
}
