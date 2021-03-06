use std::{
    convert::TryFrom,
    fmt::{self, Debug, Display},
};
use itertools::Itertools;

use crate::{
    parser::parse_sudoku,
    types::{BLOCKS, COLS, HOUSES, ROWS},
    Candidates, Cell,
};

#[derive(Debug)]
pub enum GridError {
    /// Error occurred during parsing the grid
    ParseError(String),
    /// Error when initial set of numbers is invalid
    Invalid(String),
}

impl Display for GridError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            GridError::ParseError(err) => format!("Grid parser error: {}", err),
            GridError::Invalid(err) => format!("Grid is invalid: {}", err),
        };
        write!(f, "{}", s)
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct Sudoku {
    /// The list of all cells on the grid
    cells: Vec<Cell>,
}

impl Debug for Sudoku {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl fmt::Display for Sudoku {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Helper function to generate a line to display
        fn line(widths: &[usize], start: &str, middle: &str, end: &str) -> String {
            let line: Vec<String> = widths.iter().chunks(3).into_iter().map(|widths| {
                let total = widths.sum::<usize>();
                String::from("━").repeat(total + 4)
            }).collect::<Vec<_>>();

            format!("{}{}{}", start, line.join(middle), end)
        }

        // for each cell get digit or list of candidatess
        let cells = self
            .iter()
            .map(|cell| match cell.is_digit() {
                true => cell.digit().to_string(),
                false => cell.candidates().iter().join(""),
            })
            .collect::<Vec<_>>();

        // for each list of digits determine the max width to print the longest string in each column
        let widths = (0_usize..9).map(|col| {
            cells.iter().skip(col as usize).step_by(9).map(|cell| cell.len()).max().unwrap_or(1)
        }).collect::<Vec<_>>();

        // print all rows
        write!(f, "\n{}\n", line(&widths, "┏", "┯", "┓"))?;
        for row in 0usize..9 {
            if row == 3 || row == 6 {
                write!(f, "{}\n", line(&widths, "┠", "┼", "┨"))?;
            }

            write!(f, "┃ ")?;
            for col in 0..9 {
                let longest = widths[col];
                let index = col + row * Self::ROWS as usize;

                match (row, col) {
                    (_, 3) | (_, 6) => write!(f, "┃ ")?,
                    _ => {}
                }
                let digits = &cells[index];

                write!(f, "{:<width$} ", digits, width=longest)?;
            }
            write!(f, "┃\n")?;
        }
        write!(f, "{}\n", line(&widths, "┗", "┻", "┛"))?;
        Ok(())
    }
}

/// The main Sudoku struct
///
/// The grid is divided into the following rows, columns & indices
///
/// ```text
///                               Blocks
///                   0              1              2
///             ┏━━━━━━━━━━━┓  ┏━━━━━━━━━━━┓  ┏━━━━━━━━━━━┓
///
///              0    1    2    3    4    5    6    7    8   Columns
///           ┏━━━━┯━━━━┯━━━━┳━━━━┯━━━━┯━━━━┳━━━━┯━━━━┯━━━━┓
///    ┏   0  ┃  0 ┃  1 ┃  2 ┃  3 ┃  4 ┃  5 ┃  6 ┃  7 ┃  8 ┃
///    ┃      ┠────┼────┼────╂────┼────┼────╂────┼────┼────┨
///  0 ┨   1  ┃  9 ┃ 10 ┃ 11 ┃ 12 ┃ 13 ┃ 14 ┃ 15 ┃ 16 ┃ 17 ┃
///    ┃      ┠────┼────┼────╂────┼────┼────╂────┼────┼────┨
///    ┗   2  ┃ 18 ┃ 19 ┃ 20 ┃ 21 ┃ 22 ┃ 23 ┃ 24 ┃ 25 ┃ 26 ┃
///           ┠────┼────┼────╂────┼────┼────╂────┼────┼────┨
///    ┏   3  ┃ 27 ┃ 28 ┃ 29 ┃ 30 ┃ 31 ┃ 32 ┃ 33 ┃ 34 ┃ 35 ┃
///    ┃      ┠────┼────┼────╂────┼────┼────╂────┼────┼────┨
///  1 ┨   4  ┃ 36 ┃ 37 ┃ 38 ┃ 39 ┃ 40 ┃ 41 ┃ 42 ┃ 43 ┃ 44 ┃
///    ┃      ┠────┼────┼────╂────┼────┼────╂────┼────┼────┨
///    ┗   5  ┃ 45 ┃ 46 ┃ 47 ┃ 48 ┃ 49 ┃ 50 ┃ 51 ┃ 52 ┃ 53 ┃
///           ┠────┼────┼────╂────┼────┼────╂────┼────┼────┨
///    ┏   6  ┃ 54 ┃ 55 ┃ 56 ┃ 57 ┃ 58 ┃ 59 ┃ 60 ┃ 61 ┃ 62 ┃
///    ┃      ┠────┼────┼────╂────┼────┼────╂────┼────┼────┨
///  2 ┨   7  ┃ 63 ┃ 64 ┃ 65 ┃ 66 ┃ 67 ┃ 68 ┃ 69 ┃ 70 ┃ 71 ┃
///    ┃      ┠────┼────┼────╂────┼────┼────╂────┼────┼────┨
///    ┗   8  ┃ 72 ┃ 73 ┃ 74 ┃ 75 ┃ 76 ┃ 77 ┃ 78 ┃ 79 ┃ 80 ┃
///           ┗━━━━┷━━━━┷━━━━┻━━━━┷━━━━┷━━━━┻━━━━┷━━━━┷━━━━┛
///      Rows
/// ```
///
/// The Block grid looks as follows:
///
/// ```text
///       ┏━━━━┯━━━━┯━━━━┓
///       ┃  0 ┃  1 ┃  2 ┃
///       ┠━━━━┼━━━━┼━━━━┨
///       ┃  3 ┃  4 ┃  5 ┃
///       ┠━━━━┼━━━━┼━━━━┨
///       ┃  6 ┃  7 ┃  8 ┃
///       ┗━━━━┻━━━━┻━━━━┛
/// ```
///
impl Sudoku {
    pub const BLOCK_SIZE: u8 = 3;
    pub const ROWS: u8 = 9;
    pub const COLS: u8 = 9;
    pub const NUM_FIELDS: usize = 81;

    /// Creates a new empty Sudoku
    pub fn empty() -> Self {
        Self::new([0_u8; Self::NUM_FIELDS].to_vec()).expect("Failed to create empty Sudoku")
    }

    /// Create a new grid from a list of values
    pub fn new(fields: Vec<u8>) -> Result<Self, GridError> {
        if fields.len() != Self::NUM_FIELDS as usize {
            return Err(GridError::Invalid(format!(
                "Invalid number of fields - found {} elements",
                fields.len()
            )));
        }

        let cells: Result<Vec<_>, _> = fields
            .iter()
            .enumerate()
            .map(|(index, value)| match value {
                1..=9 => Ok(Cell::number(index, *value)),
                0 => Ok(Cell::empty(index)),
                v => Err(GridError::Invalid(format!(
                    "Digit must be between 0..=9, was {}",
                    v
                ))),
            })
            .collect();

        let mut sudoku = Sudoku { cells: cells? };
        sudoku.init_candidates();
        Ok(sudoku)
    }

    /// Initializes all empty fields with candidates.
    ///
    /// **Note** this will not check or validate the candidates
    ///
    pub fn init_candidates(&mut self) {
        for index in 0..Self::NUM_FIELDS {
            if self.cells[index].is_empty() {
                let neighbors = self.get_house(index);
                let candidates = neighbors.fold(
                    Candidates::all(),
                    |mut candidates, neighbor| {
                        if neighbor.is_digit() {
                            candidates.unset(neighbor.digit());
                        }
                        candidates
                    },
                );

                let cell = &mut self.cells[index];
                cell.set_candidates(candidates);
            }
        }
    }

    /// Returns the cell given by index
    #[inline(always)]
    pub fn get(&self, index: usize) -> &Cell {
        &self.cells[index]
    }

    /// Returns a mutable reference to the cell given by index
    #[inline(always)]
    pub fn get_mut(&mut self, index: usize) -> &mut Cell {
        &mut self.cells[index]
    }

    /// Sets the value of a specifc cell given by index
    pub fn set_digit(&mut self, index: usize, digit: u8) {
        self.cells[index].set_digit(digit);
    }

    /// Unsets the cell at given coordinates
    pub fn unset(&mut self, index: usize) {
        self.cells[index].unset();
    }

    /// Naive version to check if Sudoku is solved
    /// **Note** ignores any checks that each line, row and block contains of digits 1..9
    pub fn is_solved(&self) -> bool {
        self.cells.iter().all(|f| f.is_digit())
    }

    /// Returns all fields for the given row
    pub fn get_row(&self, row: u8) -> impl Iterator<Item = &Cell> + '_ {
        let indices = &ROWS[row as usize];
        indices
            .iter()
            .map(move |&index| &self.cells[index as usize])
    }

    /// Returns an iterator over all rows
    pub fn get_rows(&self) -> impl Iterator<Item = Vec<&Cell>> + '_ {
        ROWS.iter().map(move |row| {
            row.iter()
                .map(|&index| self.get(index as usize))
                .collect::<Vec<_>>()
        })
    }

    /// Returns all fields for the given column
    pub fn get_col(&self, col: u8) -> impl Iterator<Item = &Cell> + '_ {
        let indices = &COLS[col as usize];
        indices
            .iter()
            .map(move |&index| &self.cells[index as usize])
    }

    /// Returns an iterator over all columns
    pub fn get_cols(&self) -> impl Iterator<Item = Vec<&Cell>> + '_ {
        COLS.iter().map(move |col| {
            col.iter()
                .map(|&index| &self.cells[index as usize])
                .collect::<Vec<_>>()
        })
    }

    /// Returns all fields from the given block
    pub fn get_block(&self, index: u8) -> impl Iterator<Item = &Cell> + '_ {
        let indices = &BLOCKS[index as usize];
        indices
            .iter()
            .map(move |&index| &self.cells[index as usize])
    }

    /// Returns an iterator over all blocks
    pub fn get_blocks(&self) -> impl Iterator<Item = Vec<&Cell>> + '_ {
        BLOCKS.iter().map(move |block| {
            block
                .iter()
                .map(|&index| &self.cells[index as usize])
                .collect::<Vec<_>>()
        })
    }

    /// Returns the house, all fields from same row, col and block
    pub fn get_house(&self, index: usize) -> impl Iterator<Item = &Cell> + '_ {
        let indices = &HOUSES[index];
        indices
            .iter()
            .map(move |&index| &self.cells[index as usize])
    }

    /// Returns the list of all field values with index
    pub fn iter(&self) -> impl Iterator<Item = &Cell> + '_ {
        self.cells.iter()
    }
}

/// Parses the Grid from a layout given as a string.
impl TryFrom<&str> for Sudoku {
    type Error = GridError;

    fn try_from(grid: &str) -> Result<Self, Self::Error> {
        parse_sudoku(grid)
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use crate::sudoku::Sudoku;

    #[rustfmt::skip]
    const SUDOKU: [u8; 81] = [
        5, 3, 0, 0, 7, 0, 0, 0, 0,
        6, 0, 0, 1, 9, 5, 0, 0, 0,
        0, 9, 8, 0, 0, 0, 0, 6, 0,
        8, 0, 0, 0, 6, 0, 0, 0, 3,
        4, 0, 0, 8, 0, 3, 0, 0, 1,
        7, 0, 0, 0, 2, 0, 0, 0, 6,
        0, 6, 0, 0, 0, 0, 2, 8, 0,
        0, 0, 0, 4, 1, 9, 0, 0, 5,
        0, 0, 0, 0, 8, 0, 0, 7, 9,
    ];

    #[test]
    fn parses_from_string() {
        let sudoku = r"
            --- --- 984
            4-- 8-- 25-
            -8- -49 --3
            9-6 157 8-2
            --- --- -4-
            --- -8- 196
            -34 928 56-
            6-2 -15 37-
            --5 -6- ---
        ";
        assert!(Sudoku::try_from(sudoku).is_ok());
    }

    #[test]
    fn creates_sudoku() {
        assert!(Sudoku::new(SUDOKU.to_vec()).is_ok());
    }

    #[test]
    fn create_sudoku_fails_with_wrong_numbers() {
        #[rustfmt::skip]
        let numbers = vec![
            0, 0, 0, 0, 0, 0, 9, 8, 4,
            4, 0, 0, 8, 0, 0, 2, 5, 0,
            0, 8, 0, 0, 4, 9, 0, 0, 3,
            9, 0, 6, 1, 5, 7, 8, 0, 2,
            0, 0, 0, 0, 0, 0, 0, 4, 0,
            0, 0, 0, 0, 8, 0, 1, 9, 6,
            0, 3, 4, 9, 2, 8, 5, 6, 0,
            6, 0, 2, 0, 1, 5, 3, 7, 0,
            0, 0, 5, 0, 6, 0, 0, 0, 11,
        ];
        assert!(Sudoku::new(numbers).is_err());
    }

    #[test]
    fn creates_sudoku_fails_without_numbers() {
        assert!(Sudoku::new(vec![]).is_err());
    }

    #[test]
    fn test_init_candidates() {
        #[rustfmt::skip]
        let sudoku: Vec<u8> = vec![
            8, 0, 0, 7, 3, 9, 0, 0, 6,
            3, 7, 0, 4, 6, 5, 0, 0, 0,
            0, 4, 0, 1, 8, 2, 0, 0, 9,
            0, 0, 0, 6, 0, 0, 0, 4, 0,
            0, 5, 4, 3, 0, 0, 6, 1, 0,
            0, 6, 0, 5, 0, 0, 0, 0, 0,
            4, 0, 0, 8, 5, 3, 0, 7, 0,
            0, 0, 0, 2, 7, 1, 0, 6, 4,
            1, 0, 0, 9, 4, 0, 0, 0, 2,
        ];

        let sudoku = Sudoku::new(sudoku).unwrap();
        let candidates = sudoku.get(1).candidates();

        assert_eq!(2, candidates.count());
        assert_eq!(vec![1u8, 2], candidates.iter().collect::<Vec<_>>());
    }

    #[test]
    fn test_get_row_values() {
        let sudoku = Sudoku::new(SUDOKU.to_vec()).unwrap();
        let actual = sudoku.get_row(6).map(|c| c.digit()).collect::<Vec<_>>();
        let expected = vec![0u8, 6, 0, 0, 0, 0, 2, 8, 0];
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_get_col_values() {
        let sudoku = Sudoku::new(SUDOKU.to_vec()).unwrap();
        let actual = sudoku.get_col(5).map(|c| c.digit()).collect::<Vec<_>>();
        let expected = vec![0u8, 5, 0, 0, 3, 0, 0, 9, 0];
        assert_eq!(expected, actual);
    }

    #[test]
    fn sees_other_cells() {
        let sudoku = Sudoku::new(SUDOKU.to_vec()).unwrap();
        assert!(sudoku.get(1).sees(sudoku.get(11)));
        assert!(sudoku.get(1).sees(sudoku.get(73)));
        assert!(sudoku.get(11).sees(sudoku.get(65)));
        assert!(sudoku.get(73).sees(sudoku.get(65)));
        assert!(!sudoku.get(1).sees(sudoku.get(65)));
    }
}
