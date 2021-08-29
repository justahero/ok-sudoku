pub struct Cell(u8);

impl Cell {
    pub fn new(value: u8) -> Self {
        Cell(value)
    }

    pub fn get(self) -> u8 {
        self.0
    }
}
