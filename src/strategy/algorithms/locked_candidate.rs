use itertools::Itertools;

use crate::{
    strategy::{step::Step, Strategy},
    types::Index,
    Cell, Sudoku,
};

pub struct LockedCandidate {}

impl LockedCandidate {
    pub fn new() -> Self {
        Self {}
    }

    /// Finds candidate that is confined to a row or column in a single block.
    ///
    /// The algorithm works as follows
    /// * for a list of cells (a row or column) it is checked that the cells that are shared in the same block (3 contiguous cells)
    ///   contain at least a pair of the same candidate (pointing)
    /// * it is checked no other row or column of the same block contains the candidate
    /// * then it is checked the same candidate is present in the same row / column in other blocks
    /// * when all these conditions are met, the candidates in the other blocks of the same row or column can be eliminated
    ///
    fn find_locked_candidate(&self, sudoku: &Sudoku, cells: &Vec<&Cell>) -> Option<Step> {
        assert!(cells.len() == 9);

        // get all mini rows / mini cols
        let groups = [&cells[0..3], &cells[3..6], &cells[6..9]];

        for candidate in 1_u8..=9 {
            // filter cells with the candidate in same group
            let lines = &groups
                .iter()
                .map(|&group| {
                    group
                        .iter()
                        .filter(|&&cell| cell.has_candidate(candidate))
                        .collect_vec()
                })
                .collect_vec();

            // Find the line that contains at least two candidates
            // then check the other lines do not contain the candidate
            for i in 0_u8..=2 {
                if let Some((&line, others)) = lines
                    .iter()
                    .cycle()
                    .skip(i as usize)
                    .take(3)
                    .collect::<Vec<_>>()
                    .split_first()
                {
                    if line.len() >= 2
                        && others.iter().map(|&line| line.len()).sum::<usize>() == 0_usize
                    {
                        let cell = line.first().expect("Failed to get first cell");
                        let index = Index::new(cell.index() as u8).block();
                        let block = sudoku.get_block(index);

                        // get the block of the match and check there are other cells with the same candidate
                        // then create a new step to eliminate these candidates
                        let line_indexes = line.iter().map(|&c| c.index()).collect_vec();
                        let neighbors = block
                            .filter(|&cell| !cell.is_digit() && !line_indexes.contains(&cell.index()))
                            .collect_vec();

                        let eliminates = neighbors
                            .iter()
                            .filter(|&cell| cell.has_candidate(candidate))
                            .collect_vec();

                        if eliminates.len() > 0 {
                            let mut step = Step::new();

                            line.iter()
                                .for_each(|&cell| step.lock_candidate(cell.index(), candidate));

                            eliminates.iter().for_each(|&cell| {
                                step.eliminate_candidate(cell.index(), candidate)
                            });

                            return Some(step);
                        }
                    }
                }
            }
        }

        None
    }
}

impl Strategy for LockedCandidate {
    fn find(&self, sudoku: &Sudoku) -> Option<Step> {
        for row in sudoku.get_rows() {
            if let Some(step) = self.find_locked_candidate(sudoku, &row) {
                return Some(step);
            }
        }

        for col in sudoku.get_cols() {
            if let Some(step) = self.find_locked_candidate(sudoku, &col) {
                return Some(step);
            }
        }

        None
    }

    fn name(&self) -> String {
        String::from("Locked Candidate")
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use crate::{strategy::Strategy, Sudoku};

    use super::LockedCandidate;

    #[test]
    fn locked_candidate_row_found() {
        let sudoku = r"
            318..54.6
            ...6.381.
            ..6.8.5.3
            864952137
            123476958
            795318264
            .3.5..78.
            .....73.5
            ....39641
        ";

        let mut sudoku = Sudoku::try_from(sudoku).unwrap();
        sudoku.init_candidates();
        let strategy = LockedCandidate::new();

        let step = strategy.find(&sudoku).unwrap();
        assert_eq!(
            &vec![(10_usize, 7), (11_usize, 7)],
            step.locked_candidates()
        );
        assert_eq!(&vec![(19_usize, 7)], step.eliminated_candidates());
    }

    #[test]
    fn locked_candidate_column_found() {
        let sudoku = r"
            762..8..1
            98......6
            15.....87
            478..3169
            526..9873
            3198..425
            835..1692
            297685314
            641932758
        ";

        let mut sudoku = Sudoku::try_from(sudoku).unwrap();
        sudoku.init_candidates();

        let strategy = LockedCandidate::new();

        let step = strategy.find(&sudoku).unwrap();
        assert_eq!(
            &vec![(14_usize, 4), (23_usize, 4)],
            step.locked_candidates()
        );
        assert_eq!(
            &vec![
                (3_usize, 4),
                (4_usize, 4),
                (12_usize, 4),
                (13_usize, 4),
                (21_usize, 4),
                (22_usize, 4)
            ],
            step.eliminated_candidates(),
        );
    }

    #[test]
    fn locked_candidate_not_in_grid() {}
}
