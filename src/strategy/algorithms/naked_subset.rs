use itertools::Itertools;

use crate::{Sudoku, strategy::{Strategy, step::Step}};

#[derive(Debug)]
pub struct NakedSubset {
    count: usize,
}

impl NakedSubset {
    /// Create a new Naked Subset for pairs of 2
    pub fn pair() -> Self {
        Self { count: 2 }
    }
}

impl Strategy for NakedSubset {
    fn find(&self, sudoku: &Sudoku) -> Option<Step> {
        for row in 0..9 {
            for indices in (0u8..9).permutations(self.count) {
                let cells = indices
                    .iter()
                    .map(|&col| sudoku.get(row, col).candidates_vec())
                    .collect::<Vec<_>>();

                if cells.iter().all(|candidates| candidates.len() == self.count) {
                    let first = &cells[0];
                    if cells.iter().skip(1).all(|candidates| first == candidates) {
                        let step = Step::new();
                        return Some(step);
                    }
                }
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use crate::{Sudoku, strategy::Strategy};

    use super::NakedSubset;

    /// Example of naked pair: http://hodoku.sourceforge.net/en/show_example.php?file=n201&tech=Naked+Pair
    #[test]
    fn find_naked_pairs() {
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
    }
}
