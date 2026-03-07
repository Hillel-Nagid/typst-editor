#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Ltr,
    Rtl,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BidiRun {
    pub start: usize,
    pub end: usize,
    pub level: u8,
    pub direction: Direction,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BidiInfo {
    pub levels: Vec<u8>,
    pub paragraph_level: u8,
    pub runs: Vec<BidiRun>,
}
