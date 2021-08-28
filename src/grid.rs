use std::{convert::TryFrom, fmt::{self, Debug, Display}};

use crate::parser::parse_grid;

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

#[derive(Debug)]
pub enum Value {
    /// There is no value set, initial value of a field
    Unset,
    /// Set to a number
    Number(u8),
}

pub struct Grid {
    /// The list of all fields
    fields: Vec<Value>,
}

impl Grid {
    const ROWS: u32 = 9;
    const COLS: u32 = 9;
    const NUM_FIELDS: u32 = 81;

    /// Create a new grid from a list of values
    pub fn new(fields: Vec<u8>) -> Result<Self, GridError> {
        dbg!(fields.len());

        if fields.len() != Self::NUM_FIELDS as usize {
            return Err(GridError::Invalid(format!("Invalid number of fields - found {} elements", fields.len())));
        }

        dbg!(&fields);

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
}

/// Parses the Grid from a layout given as a string.
impl TryFrom<&str> for Grid {
    type Error = GridError;

    fn try_from(grid: &str) -> Result<Self, Self::Error> {
        parse_grid(grid)
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use crate::Grid;

    #[test]
    fn parses_from_string() {
        let grid = r"
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
        assert!(Grid::try_from(grid).is_ok());
    }

    #[test]
    fn creates_grid() {
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
        assert!(Grid::new(numbers).is_ok());
    }

    #[test]
    fn create_grid_fails_with_wrong_numbers() {
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
        assert!(Grid::new(numbers).is_err());
    }

    #[test]
    fn creates_grid_fails_without_numbers() {
        assert!(Grid::new(vec![]).is_err());
    }
}
