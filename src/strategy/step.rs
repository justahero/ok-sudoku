/// A single solving step to either eliminate candidate or find digit
#[derive(Debug)]
pub struct Step {
    /// The cell index to set digit
    digit: Option<(usize, u8)>,
    /// The list of candidates to eliminate
    eliminated_candidates: Vec<(usize, Vec<u8>)>,
}

impl Step {
    pub fn new() -> Self {
        Self {
            digit: None,
            eliminated_candidates: vec![],
        }
    }
}
