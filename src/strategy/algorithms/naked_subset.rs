use itertools::Itertools;

use crate::{
    strategy::{step::Step, Strategy},
    Candidates, Cell, Sudoku,
};

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

                    for cell in &group {
                        step.lock_candidate(cell.index(), cell.candidates());
                    }

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
            if let Some(step) = self.find_tuple(&block) {
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

    use crate::{Candidates, Sudoku, strategy::Strategy};

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
        assert_eq!(
            &vec![
                (65, Candidates::new(&[3, 9])),
                (66, Candidates::new(&[3, 9])),
            ],
            step.locked_candidates(),
        );
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
            &vec![(58, 9), (60, 2), (60, 9), (62, 2), (62, 8), (62, 9)],
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

        let step = strategy.find(&sudoku).unwrap();
        assert_eq!(&vec![(1, 6)], step.eliminated_candidates(),);
        assert_eq!(
            &vec![
                (10, Candidates::new(&[3, 9])),
                (28, Candidates::new(&[6, 9])),
                (37, Candidates::new(&[3, 6, 9])),
            ],
            step.locked_candidates(),
        );
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
        assert_eq!(
            &vec![
                (54, 4),
                (54, 6),
                (55, 4),
                (55, 6),
                (55, 9),
                (63, 4),
                (72, 4),
                (72, 6),
                (73, 4),
                (73, 6)
            ],
            step.eliminated_candidates(),
        );
        assert_eq!(
            &vec![
                (56, Candidates::new(&[4, 6, 7, 9])),
                (64, Candidates::new(&[4, 9])),
                (65, Candidates::new(&[4, 7, 9])),
                (74, Candidates::new(&[4, 6, 7])),
            ],
            step.locked_candidates(),
        );
    }

    #[test]
    fn fix_naked_subset_pair() {
        let sudoku = r"
            ..81.....
            5.392....
            ...78.6..
            145698237
            .3.247.1.
            7823..964
            3.4869.5.
            8..5..4.9
            .5.4..386
        ";

        let mut sudoku = Sudoku::try_from(sudoku).unwrap();
        sudoku.init_candidates();
        sudoku.get_mut(23).unset_candidate(4);
        sudoku.get_mut(26).unset_candidate(1);
        sudoku.get_mut(26).unset_candidate(2);
        sudoku.get_mut(5).unset_candidate(3);
        sudoku.get_mut(5).unset_candidate(5);

        println!("BEFORE: {}", sudoku);
        let strategy = NakedSubset::pair();

        let step = strategy.find(&sudoku);

        dbg!(step);
    }
}
