use std::{convert::TryFrom, fmt::{self, Debug, Display}};

use crate::parser::parse_sudoku;

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

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Value {
    /// There is no value set, initial value of a field
    Unset,
    /// Set to a number
    Number(u8),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Unset => write!(f, ".")?,
            Value::Number(n) => write!(f, "{}", n)?,
        }
        Ok(())
    }
}

impl From<&Value> for u8 {
    fn from(v: &Value) -> Self {
        match v {
            Value::Number(v) => *v,
            Value::Unset => 0,
        }
    }
}

#[derive(Clone)]
pub struct Sudoku {
    /// The list of all fields
    fields: Vec<Value>,
}

impl fmt::Display for Sudoku {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..self.num_rows() {
            for x in 0..self.num_cols() {
                match (y, x) {
                    (_, 3) | (_, 6) => write!(f, " ")?,
                    (3, 0) | (6, 0) => write!(f, "\n\n")?,
                    (_, 0)          => write!(f, "\n")?,
                    _ => {}
                }
                write!(f, "{}", self.get(x, y).unwrap())?;
            }
        }
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
impl Sudoku {
    pub const BLOCK_SIZE: u8 = 3;
    pub const ROWS: u8 = 9;
    pub const COLS: u8 = 9;
    pub const NUM_FIELDS: u32 = 81;

    /// Create a new grid from a list of values
    pub fn new(fields: Vec<u8>) -> Result<Self, GridError> {
        if fields.len() != Self::NUM_FIELDS as usize {
            return Err(GridError::Invalid(format!("Invalid number of fields - found {} elements", fields.len())));
        }

        // map numbers to values
        let fields: Result<Vec<_>, _> = fields
            .into_iter()
            .map(|n| match n {
                1..=9 => Ok(Value::Number(n)),
                0 => Ok(Value::Unset),
                v => Err(GridError::Invalid(format!("Invalid value '{}' found", v))),
            })
            .collect();

        match fields {
            Ok(fields) => Ok(Self { fields }),
            Err(err) => Err(err),
        }
    }

    #[inline(always)]
    pub fn num_rows(&self) -> u8 {
        Self::ROWS
    }

    #[inline(always)]
    pub fn num_cols(&self) -> u8 {
        Self::COLS
    }

    pub fn get(&self, row: u8, col: u8) -> Option<&Value> {
        let index = col + row * self.num_rows();
        self.fields.get(index as usize)
    }

    pub fn set(&mut self, col: u8, row: u8, val: Value) {
        let index = col + row * self.num_rows();
        self.fields[index as usize] = val;
    }

    /// Naive version to check if Sudoku is solved
    /// **Note** ignores any checks that each line, row and block contains of digits 1..9
    pub fn is_solved(&self) -> bool {
        self.fields.iter().all(|f| *f != Value::Unset )
    }

    /// Returns all fields for the given row
    pub fn get_row(&self, row: u8) -> impl Iterator<Item = &Value> {
        // refactor to access via indices
        let index = (row * Self::ROWS) as usize;
        self.fields.iter().skip(index).take(9).into_iter()
    }

    /// Returns all fields for the given column
    pub fn get_col(&self, col: u8) -> impl Iterator<Item = &Value> {
        // refactor to access via indices
        self.fields.iter().skip(col as usize).step_by(9).into_iter()
    }

    /// Returns all fields from the given block
    pub fn get_block(&self, row: u8, col: u8) -> impl Iterator<Item = &Value> {
        // get all block indices from given row / col
        // Pos::new(row, col).block()
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
    fn test_get_row_values() {
        let sudoku = Sudoku::new(SUDOKU.to_vec()).unwrap();
        let actual = sudoku.get_row(6).map(|v| v.into()).collect::<Vec<_>>();
        let expected = vec![0u8, 6, 0, 0, 0, 0, 2, 8, 0];
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_get_col_values() {
        let sudoku = Sudoku::new(SUDOKU.to_vec()).unwrap();
        let actual = sudoku.get_col(5).map(|v| v.into()).collect::<Vec<_>>();
        let expected = vec![0u8, 5, 0, 0, 3, 0, 0, 9, 0];
        assert_eq!(expected, actual);
    }
}
