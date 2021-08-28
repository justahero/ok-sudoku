use std::convert::TryFrom;

pub enum GridError {
    GridParseError(String),
}

pub enum Value {
    /// There is no value set, initial value of a field
    Unset,
    /// Set to a number
    Number(u8),
}

pub struct Grid {
    fields: Vec<Value>,
}

impl Grid {

}

/// Parses the Grid from a layout given as a string.
impl TryFrom<&str> for Grid {
    type Error = GridError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        // skip all empty lines
        //
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use crate::Grid;

    #[test]
    fn parses_from_string() {
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

        assert!(Grid::try_from(grid).is_ok());
    }

    #[test]
    fn parse_fails_missing_row() {

    }
}
