use itertools::Itertools;

use crate::{Candidates, Cell, Sudoku, strategy::{step::Step, Strategy}};

#[derive(Debug)]
pub struct NakedSubset {
    count: usize,
}

impl NakedSubset {
    /// Create a new Naked Subset for pairs of 2
    pub fn pair() -> Self {
        Self { count: 2 }
    }

    /// Create a new Naked Subset for triples
    pub fn triple() -> Self {
        Self { count: 3 }
    }

    /// Create a new Naked Subset for quadruplets
    pub fn quadruple() -> Self {
        Self { count: 4 }
    }

    fn find_tuple(&self, cells: &Vec<&Cell>) -> Option<Step> {
        // get all neighbors, cells that have only candidates
        let neighbors = cells
            .iter()
            .filter(|&cell| cell.is_empty())
            .collect::<Vec<_>>();

        // for all available neighbors check if there is a naked subset
        if neighbors.len() > self.count {
            for group in neighbors.iter().permutations(self.count) {
                let subset = group
                    .iter()
                    .fold(Candidates::empty(), |mut candidates, &&cell| {
                        candidates |= cell.candidates();
                        candidates
                    });

                // In case the subset contains the same number of candidates
                if subset.count() == self.count {
                    let mut step = Step::new();

                    // TODO put the naked tuple as locked candidates in step!

                    // for all cells outside the naked subset eliminate these candidates
                    neighbors
                        .iter()
                        .filter(|index| !group.contains(index))
                        .for_each(|&&cell| {
                            for c in subset.iter() {
                                if cell.candidates().get(c) {
                                    step.eliminate_candidate(cell.index(), c);
                                }
                            }
                        });

                    if !step.eliminated_candidates().is_empty() {
                        return Some(step);
                    }
                }
            }
        }

        None
    }
}

impl Strategy for NakedSubset {
    fn find(&self, sudoku: &Sudoku) -> Option<Step> {
        for row in sudoku.get_rows() {
            if let Some(step) = self.find_tuple(&row) {
                return Some(step);
            }
        }

        for col in sudoku.get_cols() {
            if let Some(step) = self.find_tuple(&col) {
                return Some(step);
            }
        }

        for block in sudoku.get_blocks() {
            if let Some(step) = self.find_tuple( &block) {
                return Some(step);
            }
        }

        None
    }

    fn name(&self) -> String {
        format!("Naked Subset ({})", self.count)
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use crate::{strategy::Strategy, Sudoku};

    use super::NakedSubset;

    /// Example of naked pair: http://hodoku.sourceforge.net/en/show_example.php?file=n201&tech=Naked+Pair
    #[test]
    fn find_naked_pair() {
        let sudoku = r"
            7..849.3.
            928135..6
            4..267.89
            642783951
            397451628
            8156923..
            2.4516.93
            1....8.6.
            5....4.1.
        ";

        let mut sudoku = Sudoku::try_from(sudoku).unwrap();
        sudoku.init_candidates();
        let strategy = NakedSubset::pair();

        let step = strategy.find(&sudoku).unwrap();
        println!("SUDOKU: {}", sudoku);
        assert_eq!(&vec![(64, 3)], step.eliminated_candidates());
    }

    /// Example: https://github.com/dimitri/sudoku/blob/master/top95.txt
    #[test]
    fn find_naked_subset_triple_with_issue() {
        let sudoku = r"
            4.....8.5
            .3.......
            ...7.....
            .2.....6.
            ....8.4..
            ....1....
            ...6.3.7.
            5..2.....
            1.4......
        ";

        let mut sudoku = Sudoku::try_from(sudoku).unwrap();
        sudoku.init_candidates();
        let strategy = NakedSubset::triple();

        let step = strategy.find(&sudoku).unwrap();
        assert_eq!(
            &vec![
                (58, 9),
                (60, 2),
                (60, 9),
                (62, 2),
                (62, 8),
                (62, 9)
            ],
            step.eliminated_candidates(),
        );
    }

    /// Example: http://hodoku.sourceforge.net/en/show_example.php?file=n301&tech=Naked+Triple
    #[test]
    fn find_naked_triple() {
        let sudoku = r"
            ...29438.
            ...17864.
            48.3561..
            ..48375.1
            ...4157..
            5..629834
            953782416
            126543978
            .4.961253
        ";

        let mut sudoku = Sudoku::try_from(sudoku).unwrap();
        sudoku.init_candidates();
        let strategy = NakedSubset::triple();

        // TODO test specific pair of candidates and cells
        let _step = strategy.find(&sudoku).unwrap();
    }

    /// Example: http://hodoku.sourceforge.net/en/show_example.php?file=n402&tech=Naked+Quadruple
    #[test]
    fn find_naked_quadruple() {
        let sudoku = r"
            532786...
            978241.6.
            ..1953287
            .254..67.
            ..3617.52
            7..5.....
            ...1.....
            ...8.51.6
            ...3...98
        ";

        let mut sudoku = Sudoku::try_from(sudoku).unwrap();
        sudoku.init_candidates();
        let strategy = NakedSubset::quadruple();

        let step = strategy.find(&sudoku).unwrap();
        assert_eq!(10, step.eliminated_candidates().len());
    }
}
