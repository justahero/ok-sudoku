/// A single solving step to either eliminate candidate or find digit
///
/// TODO refine this struct to accomodate most of the strategies
///
#[derive(Debug, Clone, PartialEq)]
pub struct Step {
    /// The cell index to set digit
    digit: Option<(usize, u8)>,
    /// The list of candidates to eliminate for each cell index
    eliminated_candidates: Vec<(usize, u8)>,
    /// The list of constrained candidates
    constrained_candidates: Vec<(usize, u8)>,
}

impl Step {
    pub fn new() -> Self {
        Self {
            digit: None,
            eliminated_candidates: vec![],
            constrained_candidates: vec![],
        }
    }

    /// Sets the digit
    pub fn set_digit(&mut self, index: usize, digit: u8) {
        assert!(digit > 0);
        self.digit = Some((index, digit));
    }

    /// Mark single candidate to be delet / elimination
    /// The candidate will be taken out for the cell with the given index.
    pub fn eliminate_candidate(&mut self, index: usize, candidate: u8) {
        self.eliminated_candidates.push((index, candidate));
    }

    /// Returns the list of eliminated candidates
    pub fn eliminated_candidates(&self) -> &Vec<(usize, u8)> {
        &self.eliminated_candidates
    }

    /// Returns true if this contains a set digit
    pub fn has_digit(&self) -> bool {
        self.digit.is_some()
    }

    /// Returns the digit of this Step
    pub fn digit(&self) -> Option<&(usize, u8)> {
        self.digit.as_ref()
    }
}
