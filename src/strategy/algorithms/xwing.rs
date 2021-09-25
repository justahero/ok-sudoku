use itertools::Itertools;

use crate::{
    strategy::{step::Step, Strategy},
    types::IndexVec,
    Cell, Sudoku,
};
pub struct XWing {
    size: usize,
}

impl XWing {
    pub fn new() -> Self {
        Self { size: 2 }
    }

    /// TODO move this function, use it in different fish subsets
    fn find_xwing<F, G>(&self, sudoku: &Sudoku, f: F, g: G) -> Option<Step>
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
                            .for_each(|&e| step.eliminate_candidate(e.index(), candidate));
                        return Some(step);
                    }
                }
            }
        }

        None
    }
}

impl Strategy for XWing {
    fn find(&self, sudoku: &Sudoku) -> Option<Step> {
        if let Some(step) = self.find_xwing(sudoku, |c| c.row(), |c| c.col()) {
            return Some(step);
        }
        if let Some(step) = self.find_xwing(sudoku, |c| c.col(), |c| c.row()) {
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

    use crate::{
        strategy::{algorithms::xwing::XWing, Strategy},
        Sudoku,
    };

    /// See example: http://hodoku.sourceforge.net/en/tech_fishb.php
    #[test]
    fn find_xwing_in_rows() {
        let sudoku = r"
            .41729.3.
            760..34.2
            .3264.719
            4.39..17.
            6.7..49.3
            19537..24
            214567398
            376.9.541
            958431267
        ";

        let mut sudoku = Sudoku::try_from(sudoku).unwrap();
        sudoku.init_candidates();
        let strategy = XWing::new();

        let step = strategy.find(&sudoku).unwrap();

        assert_eq!(&vec![(31, 5)], step.eliminated_candidates());
        assert_eq!(
            &vec![(13, 5), (16, 5), (40, 5), (43, 5)],
            step.locked_candidates(),
        );
    }

    /// See example: http://hodoku.sourceforge.net/en/tech_fishb.php
    #[test]
    fn find_xwing_in_columns() {
        let sudoku = r"
            98..62753
            .65..3...
            327.5...6
            79..3.5..
            .5...9...
            832.45..9
            673591428
            249.87..5
            518.2...7
        ";

        let mut sudoku = Sudoku::try_from(sudoku).unwrap();
        sudoku.init_candidates();
        let strategy = XWing::new();

        let step = strategy.find(&sudoku).unwrap();
        assert_eq!(
            &vec![
                (12, 1),
                (15, 1),
                (16, 1),
                (17, 1),
                (38, 1),
                (39, 1),
                (42, 1),
                (43, 1),
                (44, 1),
            ],
            step.eliminated_candidates(),
        );
        assert_eq!(
            &vec![(9, 1), (13, 1), (36, 1), (40, 1)],
            step.locked_candidates(),
        );
    }

    #[test]
    fn does_not_find_xwing() {
        // It's a naked subset example
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
        let strategy = XWing::new();

        assert_eq!(None, strategy.find(&sudoku));
    }
}
