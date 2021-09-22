use std::collections::HashMap;

use itertools::Itertools;

use crate::{
    strategy::{step::Step, Strategy},
    types::IndexVec,
    Sudoku,
};

pub struct XWing {}

impl XWing {
    pub fn new() -> Self {
        Self {}
    }
}

impl Strategy for XWing {
    fn find(&self, sudoku: &Sudoku) -> Option<Step> {
        // get all empty cells first
        let empty_cells = sudoku.iter().filter(|&cell| cell.is_empty()).collect_vec();

        // for each possible candidate
        //for candidate in 1..=9 {
        for candidate in 1..=9 {
            // get all cells with same candidate in
            let cells = empty_cells
                .iter()
                .filter(|&cell| cell.has_candidate(candidate))
                .collect_vec();

            // get all combinations of rows / cols
            let groups = cells.iter().fold(HashMap::new(), |mut result, &cell| {
                if result.get(&cell.row()).is_none() {
                    result.insert(cell.row(), vec![]);
                }
                if let Some(entries) = result.get_mut(&cell.row()) {
                    entries.push(*cell);
                }
                result
            });
            println!(">> CANDIDATE: {}, GROUPS: {:?}", candidate, groups);

            // check if there are multiple rows with the same set of candidates
            let subset = groups
                .iter()
                .permutations(2)
                .find(|rows| {
                    let candidates =
                        rows.iter()
                            .fold(IndexVec::new(), |mut result, (_row, list)| {
                                list.iter().for_each(|&cell| result.set(cell.col() as u8));
                                result
                            });
                    rows.len() >= 2 && candidates.count() == 2
                })
                .map(|rows| rows.iter().map(|x| x.1).flatten().collect_vec());

            println!(":: SUBSET {:?}", subset);

            // in case there is one wing, check if there are other candidates in these columns
            if let Some(subset) = subset {
                let eliminates = cells
                    .iter()
                    .filter(|&neighbor| {
                        println!("----- CELL: {:?}", neighbor);

                        // find all cells that are shared by same column or row
                        subset.iter().any(|&cell| {
                            cell.col() == neighbor.col() || cell.row() == neighbor.row()
                        })
                    })
                    .filter(|&cell| !subset.iter().any(|&n| n.index() == cell.index()))
                    .collect_vec();

                if !eliminates.is_empty() {
                    println!(":: ELIMINATES: {:?}", eliminates);
                    let mut step = Step::new();
                    subset
                        .iter()
                        .for_each(|&c| step.lock_candidate(c.index(), candidate));
                    eliminates
                        .iter()
                        .for_each(|&c| step.eliminate_candidate(c.index(), candidate));
                    return Some(step);
                }
            }
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

        println!("SUDOKU: {}", sudoku);
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
    }
}
