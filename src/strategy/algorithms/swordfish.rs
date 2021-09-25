
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
    #[test]
    fn find_swordfish() {

    }

    #[test]
    fn ignores_xwing() {
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
        let strategy = Swordfish::new();

        assert_eq!(None, strategy.find(&sudoku));
    }
}
