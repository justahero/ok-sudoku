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

    #[test]
    fn find_swordfish_in_multiple_sudokus() {
        let sudokus = [
            r"926...1.. 537.1.42. 841...6.3 259734816 714.6..3. 36812..4. 1.2....82 485.7136. 6.3.....1",
            r".2..43.69 ..38962.. 96..25.3. 89.56..13 6...3.... .3..81.26 3...1..7. ..96743.2 27.358.9.",
        ];

        let strategy = Swordfish::new();
        for sudoku in sudokus.iter() {
            let mut sudoku = Sudoku::try_from(*sudoku).unwrap();
            sudoku.init_candidates();

            assert!(strategy.find(&sudoku).is_some());
        }
    }

    /// This grid is taken from: https://www.youtube.com/watch?v=9m9t8ie9-EE
    #[test]
    fn ignores_xwings() {
        let sudokus = [
            r"5..27134....6.3....3.8.9..7..3..7.....7..8.3.6..31472..8.79...3...3..6.....1..5..",
        ];

        let strategy = Swordfish::new();
        for sudoku in sudokus.iter() {
            let mut sudoku = Sudoku::try_from(*sudoku).unwrap();
            sudoku.init_candidates();

            assert!(strategy.find(&sudoku).is_none());
        }
    }
}
