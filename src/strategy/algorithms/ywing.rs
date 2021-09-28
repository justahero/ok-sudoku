use itertools::Itertools;

use crate::{Sudoku, strategy::{Strategy, step::Step}};

pub struct YWing {}

impl YWing {
    pub fn new() -> Self {
        Self {}
    }
}

impl Strategy for YWing {    
    fn find(&self, sudoku: &Sudoku) -> Option<Step> {
        // get all cells that contain exactly 2 candidates
        let cells = sudoku.iter().filter(|&cell| cell.candidates().count() == 2).collect_vec();
        println!(":: CELLS: {:?}", cells);

        // The pivot is the bivalue cell with candidates X, Y
        for pivot in cells.iter() {
            println!(":: PIVOT: {:?}", pivot);

            println!("-- BLOCK {}", pivot.block());

            // find all neighbors that see the given pivot cell
            let neighbors = cells
                .iter()
                .filter(|&cell| cell.index() != pivot.index())
                .filter(|&cell| {
                    cell.row() == pivot.row() || cell.col() == pivot.col() || cell.block() == pivot.block()
                });

            println!(":: NEIGHBORS: {:?}", neighbors);

            if let Some((x, y)) = pivot.candidates().iter().collect_tuple() {
                // println!(":: PIVOT: {:?}, CANDIDATES: {}, {}, neighbors: {:?}", pivot, x, y, neighbors);
            }
        }

        None
    }

    fn name(&self) -> String {
        String::from("YWing")
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use crate::{Sudoku, strategy::Strategy};

    use super::YWing;

    /// See example: https://www.sudokuwiki.org/Y_Wing_Strategy
    #[test]
    fn find_ywing() {
        let sudoku = r"
            9..24....
            .5.69.231
            .2..5..9.
            .9.7..32.
            ..29356.7
            .7...29..
            .69.2..73
            51..79.62
            2.7.86..9
        ";

        let mut sudoku = Sudoku::try_from(sudoku).unwrap();
        sudoku.init_candidates();
        let strategy = YWing::new();

        let step = strategy.find(&sudoku).unwrap();
        assert_eq!(&vec![(65, 4)], step.eliminated_candidates());
        assert_eq!(
            &vec![
                (1, 3),
                (1, 8),
                (11, 3),
                (11, 8),
                (73, 3),
                (73, 8),
            ],
            step.locked_candidates(),
        );
    }
}
