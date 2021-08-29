use crate::Sudoku;

static BLOCKS: [[u8; 9]; 9] = [
    [00, 01, 02, 09, 10, 11, 18, 19, 20],
    [03, 04, 05, 12, 13, 14, 21, 22, 23],
    [06, 07, 08, 15, 16, 17, 23, 25, 26],
    [27, 28, 29, 36, 37, 38, 45, 46, 47],
    [30, 31, 32, 39, 40, 41, 48, 49, 50],
    [33, 34, 35, 42, 43, 44, 51, 52, 53],
    [54, 55, 56, 63, 64, 65, 72, 73, 74],
    [57, 58, 59, 66, 67, 68, 75, 76, 77],
    [60, 61, 62, 69, 70, 71, 78, 79, 80],
];

static ROWS: [[u8; 9]; 9] = [
    [00, 01, 02, 03, 04, 05, 06, 07, 08],
    [09, 10, 11, 12, 13, 14, 15, 16, 17],
    [18, 19, 20, 21, 22, 23, 24, 25, 26],
    [27, 28, 29, 30, 31, 32, 33, 34, 35],
    [36, 37, 38, 39, 40, 41, 42, 43, 44],
    [45, 46, 47, 48, 49, 50, 51, 52, 53],
    [54, 55, 56, 57, 58, 59, 60, 61, 62],
    [63, 64, 65, 66, 67, 68, 69, 70, 71],
    [72, 73, 74, 75, 76, 77, 78, 79, 80],
];

static COLS: [[u8; 9]; 9] = [
    [00, 09, 18, 27, 36, 45, 54, 63, 72],
    [01, 10, 19, 28, 37, 46, 55, 64, 73],
    [02, 11, 20, 29, 38, 47, 56, 65, 74],
    [03, 12, 21, 30, 39, 48, 57, 66, 75],
    [04, 13, 22, 31, 40, 49, 58, 67, 76],
    [05, 14, 23, 32, 41, 50, 59, 68, 77],
    [06, 15, 24, 33, 42, 51, 60, 69, 78],
    [07, 16, 25, 34, 43, 52, 61, 70, 79],
    [08, 17, 26, 35, 44, 53, 62, 71, 80],
];

/// A block index
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct Block(u8);

/// A single Column value
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct Col(u8);

/// A single Row value
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct Row(u8);

/// Position on the board
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct Pos(u8, u8);

/// One dimensional index on the board
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct Index(u8);

impl Index {
    /// Creates a new index
    pub fn new(index: u8) -> Self {
        Index(index)
    }

    /// Returns the row on the board
    #[inline(always)]
    pub fn row(&self) -> Row {
        Row(self.0 / Sudoku::ROWS)
    }

    /// Returns the column on the board
    #[inline(always)]
    pub fn col(&self) -> Col {
        Col(self.0 % Sudoku::COLS)
    }

    /// Returns the position on the board
    #[inline(always)]
    pub fn pos(&self) -> Pos {
        Pos(self.row().0, self.col().0)
    }

    /// Returns the associated block index.
    ///
    /// Block indices map from the grid as follows.
    ///
    ///   0 0 0 1 1 1 2 2 2
    ///   0 0 0 1 1 1 2 2 2
    ///   0 0 0 1 1 1 2 2 2
    ///   3 3 3 4 4 4 5 5 5
    ///   3 3 3 4 4 4 5 5 5
    ///   3 3 3 4 4 4 5 5 5
    ///   6 6 6 7 7 7 8 8 8
    ///   6 6 6 7 7 7 8 8 8
    ///   6 6 6 7 7 7 8 8 8
    ///
    #[inline(always)]
    pub fn block(&self) -> Block {
        self.pos().block()
    }
}

impl From<Index> for Pos {
    fn from(index: Index) -> Self {
        index.pos()
    }
}

impl Pos {
    pub fn block(&self) -> Block {
        Block::new(self.0 % Sudoku::BLOCK_SIZE + self.1 % Sudoku::BLOCK_SIZE)
    }
}

impl Block {
    pub fn new(block: u8) -> Self {
        Self(block)
    }

    /// Returns all indices for this block
    #[inline(always)]
    pub fn indices(&self) -> &[u8; 9] {
        &BLOCKS[self.0 as usize]
    }
}

impl Row {
    pub fn new(row: u8) -> Self {
        Self(row)
    }

    /// Returns all indices for this row
    pub fn indices(&self) -> &[u8; 9] {
        &ROWS[self.0 as usize]
    }
}

impl Col {
    pub fn new(col: u8) -> Self {
        Self(col)
    }

    pub fn indices(&self) -> &[u8; 9] {
        &COLS[self.0 as usize]
    }
}
