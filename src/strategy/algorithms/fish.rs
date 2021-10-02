use itertools::Itertools;

use crate::{Cell, Sudoku, strategy::step::Step};

/// Finds a fish in the Sudoku
pub(crate) fn find_fish<F, G>(size: usize, sudoku: &Sudoku, f: F, g: G) -> Option<Step>
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

        if groups.len() < size {
            continue;
        }

        // for each tuple of lines check if there is a xwing
        for lines in groups.iter().permutations(size) {
            let mut indexes = [0u8; 9];
            for line in &lines {
                line.iter().for_each(|&cell| indexes[g(cell)] += 1);
            }

            let indexes = indexes.iter().filter(|&index| *index > 0).collect_vec();
            if indexes.len() > size {
                continue;
            }

            // Check that each intersecting line contains at least 2 overlapping entries
            let result = indexes
                .iter()
                .filter(|&&index| *index >= 2)
                .count();

            if result == size {
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
                        .for_each(|&l| step.lock_candidate(l.index(), candidate.into()));
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
