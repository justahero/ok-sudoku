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

    /// Sets the digit
    pub fn set_digit(&mut self, index: usize, digit: u8) {
        assert!(digit > 0);
        self.digit = Some((index, digit));
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
