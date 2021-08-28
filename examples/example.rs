use std::convert::TryFrom;

use sudoku::{Solver, Sudoku};

fn main() {
    let sudoku = r"
        --- --- ---
        --- --3 -85
        --1 -2- ---
        --- 5-7 ---
        --4 --- 1--
        -9- --- ---
        5-- --- -73
        --2 -1- ---
        --- -4- --9
    ";

    let sudoku = Sudoku::try_from(sudoku).unwrap();
    match Solver::solve(&sudoku) {
        Ok(result) => println!("SOLVED: {}", result),
        Err(err) => println!("Failed to solve Sudoku: {}", err),
    }
}