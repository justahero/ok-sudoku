use itertools::Itertools;

use crate::{Cell, Sudoku, strategy::{Strategy, step::Step}, types::IndexVec};

pub struct Swordfish {
    size: usize,
}

impl Swordfish {
    pub fn new() -> Self {
        Self { size: 3 }
    }

    fn find_fish<F, G>(&self, sudoku: &Sudoku, f: F, g: G) -> Option<Step>
    where
        F: Fn(&Cell) -> usize,
        G: Fn(&Cell) -> usize,
    {
        // get all empty cells first
        let empty_cells = sudoku.iter().filter(|&cell| cell.is_empty()).collect_vec();

        // for each possible candidate
        for candidate in 1..=9 {
            // get all cells with same candidate in
            let cells = &empty_cells
                .iter()
                .filter(|&cell| cell.has_candidate(candidate))
                .collect_vec();

            // get all cells grouped by their lines
            let groups = cells
                .iter()
                .into_group_map_by(|&cell| f(cell))
                .into_iter()
                .map(|(_, line)| line.to_vec())
                .filter(|line| line.len() >= 2)
                .collect_vec();

            if groups.len() < self.size {
                continue;
            }

            // for each tuple of lines check if there is a xwing
            for lines in groups.iter().permutations(self.size) {
                let mut indexes = IndexVec::new();
                for line in &lines {
                    line.iter().for_each(|&cell| indexes.set(g(cell) as u8));
                }

                // at least two lines found, now check if there are any candidates to eliminate
                // along same lines
                if indexes.count() == self.size as u8 {
                    let mut eliminates = Vec::new();
                    let subset = lines.into_iter().flatten().collect_vec();
                    for neighbor in cells {
                        if subset.iter().any(|&cell| {
                            cell.col() == neighbor.col() || cell.row() == neighbor.row()
                        }) {
                            if !subset.iter().any(|&cell| cell.index() == neighbor.index()) {
                                eliminates.push(neighbor);
                            }
                        }
                    }

                    // xwing found, eliminate candidates and lock candidates of subset cells
                    if !eliminates.is_empty() {
                        let mut step = Step::new();
                        subset
                            .iter()
                            .sorted_by(|&l, &r| l.index().cmp(&r.index()))
                            .for_each(|&l| step.lock_candidate(l.index(), candidate));
                        eliminates
                            .iter()
                            .sorted_by(|&l, &r| l.index().cmp(&r.index()))
                            .for_each(|&e| step.eliminate_candidate(e.index(), candidate));
                        return Some(step);
                    }
                }
            }
        }

        None
    }
}

impl Strategy for Swordfish {
    fn find(&self, sudoku: &Sudoku) -> Option<Step> {
        if let Some(step) = self.find_fish(sudoku, |c| c.row(), |c| c.col()) {
            return Some(step);
        }
        if let Some(step) = self.find_fish(sudoku, |c| c.col(), |c| c.row()) {
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

    use crate::{Sudoku, strategy::{Strategy, algorithms::Swordfish}};

    /// Example found here: http://hodoku.sourceforge.net/en/show_example.php?file=bf301&tech=Swordfish
    #[test]
    fn find_swordfish() {
        let sudoku = r"
            16.543.7.
            .786.1435
            4358.76.1
            72.458.69
            6..912.57
            ...376..4
            .16.3..4.
            3...8..16
            ..71645.3
        ";

        let mut sudoku = Sudoku::try_from(sudoku).unwrap();
        sudoku.init_candidates();
        let strategy = Swordfish::new();

        let step = strategy.find(&sudoku).unwrap();

        assert_eq!(&vec![(52, 2), (54, 2)], step.eliminated_candidates());
        assert_eq!(
            &vec![
                (9, 2),
                (13, 2),
                (22, 2),
                (25, 2),
                (72, 2),
                (79, 2),
            ],
            step.locked_candidates(),
        );
    }

    /// See example: https://www.sudokuwiki.org/X_Wing_Strategy
    #[test]
    fn ignores_xwing() {
        let sudoku = r"
            1.....569
            492.561.8
            .561.924.
            ..964.8.1
            .64.1....
            218.356.4
            .4.5...16
            9.5.614.2
            621.....5
        ";

        let mut sudoku = Sudoku::try_from(sudoku).unwrap();
        sudoku.init_candidates();
        let strategy = Swordfish::new();

        assert_eq!(None, strategy.find(&sudoku));
    }
}
