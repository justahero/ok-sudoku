/// A single solving step to either eliminate candidate or find digit
#[derive(Debug)]
pub(crate) struct Step {

}

/// A `Steps` is a list of steps to find the next digit in the grid.
/// It holds all information to know how to apply the change.
#[derive(Debug)]
pub struct Steps {
    /// The list of all solving steps
    steps: Vec<Step>,
    /// Index for Iterator
    index: u32,
}

impl Default for Steps {
    fn default() -> Self {
        Self {
            steps: Vec::new(),
            index: 0
        }
    }
}
