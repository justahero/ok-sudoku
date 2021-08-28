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

impl Sudoku {
    const ROWS: u32 = 9;
    const COLS: u32 = 9;
    const NUM_FIELDS: u32 = 81;

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
    pub fn num_rows(&self) -> u32 {
        Self::ROWS
    }

    #[inline(always)]
    pub fn num_cols(&self) -> u32 {
        Self::COLS
    }

    pub fn get(&self, x: u32, y: u32) -> Option<&Value> {
        let index = x + y * self.num_rows();
        self.fields.get(index as usize)
    }

    pub fn set(&mut self, x: u32, y: u32, val: Value) {
        let index = x + y * self.num_rows();
        self.fields[index as usize] = val;
    }

    /// Naive version to check if Sudoku is solved
    /// **Note** ignores any checks that each line, row and block contains of digits 1..9
    pub fn is_solved(&self) -> bool {
        self.fields.iter().all(|f| *f != Value::Unset )
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

    use crate::Sudoku;

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
        let numbers = vec![
            0, 0, 0, 0, 0, 0, 9, 8, 4,
            4, 0, 0, 8, 0, 0, 2, 5, 0,
            0, 8, 0, 0, 4, 9, 0, 0, 3,
            9, 0, 6, 1, 5, 7, 8, 0, 2,
            0, 0, 0, 0, 0, 0, 0, 4, 0,
            0, 0, 0, 0, 8, 0, 1, 9, 6,
            0, 3, 4, 9, 2, 8, 5, 6, 0,
            6, 0, 2, 0, 1, 5, 3, 7, 0,
            0, 0, 5, 0, 6, 0, 0, 0, 0,
        ];
        assert!(Sudoku::new(numbers).is_ok());
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
}
