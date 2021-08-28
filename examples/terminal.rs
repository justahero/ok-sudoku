use std::convert::{TryFrom, TryInto};

use ok_sudoku::Grid;

fn main() {
    let grid = r"##
    --- --- 984
    4-- 8-- 25-
    -8- -49 --3
    9-6 157 8-2
    --- --- -4-
    --- -8- 196
    -34 928 56-
    6-2 -15 37-
    --5 -6- ---
    ##";

    let _grid = Grid::try_from(grid);
}