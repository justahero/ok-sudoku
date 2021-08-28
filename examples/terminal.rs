use std::convert::TryFrom;

use clap::{App, Arg};
use sudoku::Grid;

fn main() {
    let matches = App::new("Sudoku Solver CLI")
        .version("0.1.0")
        .author("Sebastian Ziebell")
        .about("Solve sudokus from the command line")
        .arg(Arg::with_name("grid")
            .short("g")
            .long("grid")
            .help("The 9x9 grid as a single contiguous string"))
        .get_matches();

    if let Some(grid) = matches.value_of("grid") {
        let grid = Grid::try_from(grid).unwrap();
        let solver = Solver::new();
        // println!("Grid: {}", Solver::new(grid).solve());
    }
}
