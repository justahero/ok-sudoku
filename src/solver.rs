use crate::{Grid, grid::Value};

pub enum SolverError {

}

pub struct Solver {}

impl Solver {
    /// 
    pub fn solve(grid: &Grid) -> Result<Grid, SolverError> {
        todo!();
    }

    fn solve_grid(grid: Grid) {
        for y in 0..grid.num_rows() {
            for x in 0..grid.num_cols() {
                if grid.get(x, y) == Value::Unset {
                    
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {

}
