use peg::parser;

use crate::sudoku::{GridError, Sudoku};

parser! {
    grammar sudoku_parser() for str {
        rule empty() -> u8
            = $(['-' | '.' ]) { 0 }

        rule number() -> u8
            = n:$(['0'..='9']) { str::parse::<u8>(n).unwrap() }

        rule _ = [' ' | '\n' | '\r' | '\t']*

        rule field() -> u8
            = _ n:number() _ { n }
            / _ e:empty() _ { e }

        pub(crate) rule numbers() -> Vec<u8>
            = n:field() * { n }

        pub(crate) rule parse() -> Sudoku
            = numbers:numbers() { Sudoku::new(numbers).unwrap() }
    }
}

pub fn parse_sudoku(grid: &str) -> Result<Sudoku, GridError> {
    sudoku_parser::parse(grid)
        .map_err(|e| GridError::ParseError(e.to_string()))
}

#[cfg(test)]
mod test {
    use crate::parser::parse_sudoku;
    use crate::parser::sudoku_parser;

    #[test]
    fn test_parse_sudoku() {
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
        assert!(parse_sudoku(sudoku).is_ok());
    }

    #[test]
    fn test_parse_single_field() {
        let numbers = sudoku_parser::numbers("123 -1- 1-2");
        assert!(numbers.is_ok());

        let numbers = numbers.unwrap();
        assert_eq!(vec![1, 2, 3, 0, 1, 0, 1, 0, 2], numbers);
    }
}
