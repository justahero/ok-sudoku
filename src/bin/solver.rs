use std::convert::TryFrom;

use clap::{App, Arg};
use sudoku::{Sudoku, Solver, StrategySolver};

fn main() {
    let matches = App::new("Sudoku Solver CLI")
        .version("0.1.0")
        .author("Sebastian Ziebell")
        .about("Solve sudokus from the command line")
        .arg(Arg::with_name("grid")
            .short("g")
            .long("grid")
            .value_name("STRING")
            .takes_value(true)
            .help("The 9x9 grid as a single contiguous string"))
        .arg(Arg::with_name("brute")
            .short("b")
            .long("brute")
            .value_name("STRING")
            .takes_value(false)
            .required(false)
        )
        .get_matches();

    let brute = matches.is_present("brute");

    if let Some(grid) = matches.value_of("grid") {
        let sudoku = Sudoku::try_from(grid).unwrap();

        if brute {
            match Solver::find_unique(&sudoku) {
                Ok(result) => println!("Solved: \n{}", result),
                Err(err) => println!("Failed to solve grid: {}", err),
            }
        } else {
            let solver = StrategySolver::new();
            match solver.solve(&sudoku, false) {
                Ok((result, _steps)) => println!("Solved: \n{}", result),
                Err(err) => println!("Failed to solve grid: {}", err),
            }
        }
    }
}
