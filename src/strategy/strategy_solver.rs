use crate::{solver::SolverError, Solver, Sudoku};

use super::{
    algorithms::{
        HiddenSingle, HiddenSubset, Jellyfish, LockedCandidate, NakedSingle, NakedSubset,
        PointingTuple, Swordfish, XWing, YWing,
    },
    step::Step,
    Strategy,
};

/// The `StrategySolver` is the struct for solving Sudokus
/// by applying logical strategies that humans can do.
pub struct StrategySolver {
    /// List of all strategies
    strategies: Vec<Box<dyn Strategy>>,
}

impl StrategySolver {
    /// Creates a new Solver with a list of strategies
    pub fn new() -> Self {
        let mut solver = StrategySolver {
            strategies: Vec::new(),
        };
        solver.add_default_strategies();
        solver
    }

    /// Solve the Sudoku by applying solving steps.
    pub fn solve(
        &self,
        sudoku: &Sudoku,
        check_step: bool,
    ) -> Result<(Sudoku, Vec<Step>), SolverError> {
        let mut sudoku = sudoku.clone();
        sudoku.init_candidates();

        println!("SOLVE: {}", sudoku);

        let mut steps = vec![];
        let mut count = 0usize;

        loop {
            if let Some((strategy, step)) = self
                .strategies
                .iter()
                .find_map(|strategy| strategy.find(&sudoku).map(|step| (strategy, step)))
            {
                count += 1;

                println!(
                    "STRATEGY ({:02}): {:?}, STEP: {:?}",
                    count,
                    strategy.name(),
                    step
                );
                steps.push(step.clone());
                self.apply(&step, &mut sudoku);

                if check_step {
                    Solver::find_unique(&sudoku)?;
                }
            } else {
                return Solver::find_unique(&sudoku).and(Err(SolverError::StrategyNotFound));
            }

            if sudoku.is_solved() {
                return Ok((sudoku, steps));
            }
        }
    }

    /// Adds all available default strategies
    fn add_default_strategies(&mut self) {
        self.push_strategy(Box::new(NakedSingle::new()));
        self.push_strategy(Box::new(HiddenSingle::new()));
        self.push_strategy(Box::new(NakedSubset::pair()));
        self.push_strategy(Box::new(NakedSubset::triple()));
        self.push_strategy(Box::new(NakedSubset::quadruple()));
        self.push_strategy(Box::new(HiddenSubset::pair()));
        self.push_strategy(Box::new(HiddenSubset::triple()));
        self.push_strategy(Box::new(HiddenSubset::quadruple()));
        self.push_strategy(Box::new(LockedCandidate::new()));
        self.push_strategy(Box::new(PointingTuple::new()));
        self.push_strategy(Box::new(XWing::new()));
        self.push_strategy(Box::new(Swordfish::new()));
        self.push_strategy(Box::new(Jellyfish::new()));
        self.push_strategy(Box::new(YWing::new()));
    }

    /// Adds a single strategy
    pub fn push_strategy(&mut self, strategy: Box<dyn Strategy>) {
        self.strategies.push(strategy);
    }

    /// Apply the step
    /// TODO return Result with SolverError?
    pub fn apply(&self, step: &Step, sudoku: &mut Sudoku) {
        for (index, candidate) in step.eliminated_candidates() {
            sudoku.get_mut(*index).unset_candidate(*candidate);
        }
        if let Some((index, digit)) = step.digit() {
            sudoku.set_digit(*index, *digit);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use crate::Sudoku;

    use super::StrategySolver;

    #[test]
    fn solve_sudokus() {
        // A few sudokus found here: https://sandiway.arizona.edu/sudoku/examples.html
        #[rustfmt::skip]
        let sudokus = vec![
            // r"...26.7.1 68..7..9. 19...45.. 82.1...4. ..46.29.. .5...3.28 ..93...74 .4..5..36 7.3.18...",
            r"4.....8.5 .3....... ...7..... .2.....6. ....8.4.. ....1.... ...6.3.7. 5..2..... 1.4......",
        ];

        #[rustfmt::skip]
        let solutions = vec![
            // r"435269781 682571493 197834562 826195347 374682915 951743628 519326874 248957136 763418259",
            r"417369825 632158947 958724316 825437169 791586432 346912758 289643571 573291684 164875293",
        ];

        let solver = StrategySolver::new();

        for (&sudoku, solution) in sudokus.iter().zip(solutions) {
            let sudoku = Sudoku::try_from(sudoku).unwrap();
            let solution = Sudoku::try_from(solution).unwrap();

            let actual = solver.solve(&sudoku, false);
            assert!(actual.is_ok());
            let (actual, _) = actual.unwrap();
            assert_eq!(solution, actual);
        }
    }
}
