use std::fmt::Debug;

use itertools::Itertools;

use crate::{Candidates, Cell, Sudoku, strategy::{step::Step, Strategy}};

#[derive(Debug)]
pub struct HiddenSubset {
    count: u8,
}

impl HiddenSubset {
    pub fn pair() -> Self {
        Self { count: 2 }
    }

    pub fn triple() -> Self {
        Self { count: 3 }
    }

    pub fn quadruple() -> Self {
        Self { count: 4 }
    }

    fn find_tuple(&self, cells: &Vec<&Cell>) -> Option<Step> {
        // Group all cells by candidate to their cells that contain them
        let candidates = cells
            .iter()
            .flat_map(|&cell| cell.candidates().iter().map(|c| (c, cell)).collect_vec())
            .into_group_map();

        // Find a group that has the same list of indexes, still contains other candidates to hide within
        let group = candidates
            .iter()
            .filter(|(_index, indexes)| indexes.len() == self.count as usize)
            .permutations(self.count as usize)
            .filter(|entries| entries.len() <= self.count as usize)
            .find(|entries| entries.iter().map(|(_, indexes)| indexes).all_equal());

        if let Some(group) = group {
            let found_cells = group[0].1.iter().collect_vec();

            // get all candidates from neighbors outside the hidden tuple
            let candidates = &cells
                .iter()
                .filter(|&cell| !found_cells.contains(&cell))
                .fold(Candidates::empty(), |mut result, &cell| {
                    result |= cell.candidates();
                    result
                });

            // find all candidates that can be eliminated from hidden tuple
            let mut eliminates: Vec<(usize, Candidates)> = Vec::new();
            for &cell in found_cells {
                let result = Candidates::intersect(candidates, &cell.candidates());
                if result.count() > 0 {
                    eliminates.push((cell.index(), result));
                }
            }

            // there are any candidates to eliminate
            if !eliminates.is_empty() {
                let mut step = Step::new();
                for (index, candidates) in eliminates {
                    candidates.iter().for_each(|candidate| step.eliminate_candidate(index, candidate));
                }
                return Some(step);
            }
        }

        None
    }
}

impl Strategy for HiddenSubset {
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

    use crate::{strategy::Strategy, Sudoku};

    use super::HiddenSubset;

    /// Example of hidden subset: http://hodoku.sourceforge.net/en/show_example.php?file=h202&tech=Hidden+Pair
    #[test]
    fn find_hidden_subset() {
        let sudoku = r"
            ....6....
            ....42736
            ..673..4.
            .94....68
            ....964.7
            6.7.5.923
            1......85
            .6..8.271
            ..5.1..94
        ";

        let sudoku = Sudoku::try_from(sudoku).unwrap();
        let strategy = HiddenSubset::pair();

        let step = strategy.find(&sudoku);
        assert!(step.is_some());

        let step = step.unwrap();
        let expected_eliminated: Vec<(usize, u8)> = vec![
            (0, 2),
            (0, 3),
            (0, 5),
            (0, 8),
            (0, 9),
            (1, 1),
            (1, 2),
            (1, 3),
            (1, 5),
            (1, 8),
        ];
        assert_eq!(&expected_eliminated, step.eliminated_candidates());
    }

    /// Example: http://hodoku.sourceforge.net/en/show_example.php?file=h201&tech=Hidden+Pair
    #[test]
    fn find_hidden_subset_pair() {
        let sudoku = r"
            .49132...
            .81479...
            327685914
            .96.518..
            .75.28...
            .38.46..5
            853267...
            712894563
            964513...
        ";

        let sudoku = Sudoku::try_from(sudoku).unwrap();
        let strategy = HiddenSubset::pair();

        let step = strategy.find(&sudoku);
        assert!(step.is_some());

        let step = step.unwrap();
        assert_eq!(
            &vec![(44, 6)],
            step.eliminated_candidates(),
        );
    }
}
