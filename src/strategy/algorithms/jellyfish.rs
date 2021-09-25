use crate::{Sudoku, strategy::{step::Step, Strategy}};

use super::find_fish;

pub struct Jellyfish {}

impl Jellyfish {
    pub fn new() -> Self {
        Self {}
    }
}

impl Strategy for Jellyfish {
    fn find(&self, sudoku: &Sudoku) -> Option<Step> {
        if let Some(step) = find_fish(4, sudoku, |c| c.row(), |c| c.col()) {
            return Some(step);
        }
        if let Some(step) = find_fish(4, sudoku, |c| c.col(), |c| c.row()) {
            return Some(step);
        }

        None
    }

    fn name(&self) -> String {
        String::from("XWing (Fish)")
    }
}

#[cfg(test)]
mod tests {
}
