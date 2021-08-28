use peg::parser;

use crate::{Grid, grid::GridError};

parser! {
    grammar grid_parser() for str {
        rule empty() -> u8
            = $(['-']) { 0 }

        rule number() -> u8
            = n:$(['0'..='9']) { str::parse::<u8>(n).unwrap() }

        rule _ = [' ' | '\n' | '\r' | '\t']*

        rule field() -> u8
            = _ n:number() _ { n }
            / _ e:empty() _ { e }

        pub(crate) rule numbers() -> Vec<u8>
            = n:field() * { n }

        pub(crate) rule parse() -> Grid
            = numbers:numbers() { Grid::new(numbers).unwrap() }
    }
}

pub fn parse_grid(grid: &str) -> Result<Grid, GridError> {
    grid_parser::parse(grid)
        .map_err(|e| GridError::ParseError(e.to_string()))
}

#[cfg(test)]
mod test {
    use crate::parser::parse_grid;
    use crate::parser::grid_parser;

    #[test]
    fn test_parse_grid() {
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
        assert!(parse_grid(grid).is_ok());
    }

    #[test]
    fn test_parse_single_field() {
        let numbers = grid_parser::numbers("123 -1- 1-2");
        assert!(numbers.is_ok());

        let numbers = numbers.unwrap();
        assert_eq!(vec![1, 2, 3, 0, 1, 0, 1, 0, 2], numbers);
    }
}
