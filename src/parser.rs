use peg::parser;

use crate::sudoku::{GridError, Sudoku};

parser! {
    grammar sudoku_parser() for str {
        rule empty() -> u8
            = $(['-' | '.' | '0' ]) { 0 }

        rule number() -> u8
            = n:$(['0'..='9']) { str::parse::<u8>(n).unwrap() }

        rule _ = [' ' | '\n' | '\r' | '\t']*

        rule field() -> u8
            = _ n:number() _ { n }
            / _ e:empty() _ { e }

        rule numbers() -> Vec<u8>
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
    use crate::Sudoku;
    use crate::parser::parse_sudoku;

    #[test]
    fn test_parse_sudoku() {
        let input = r"
            000 --- 984
            4.. 8.. 25.
            .8. .49 ..3
            9.6 157 8.2
            ... ... .4.
            ... .8. 196
            .34 928 56.
            6.2 .15 37.
            ..5 .6. ...
        ";
        let expected = vec![
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
        let expected = Sudoku::new(expected).unwrap();

        let result = parse_sudoku(input);
        assert!(result.is_ok());

        let actual = result.unwrap();
        assert_eq!(expected, actual);
    }
}
