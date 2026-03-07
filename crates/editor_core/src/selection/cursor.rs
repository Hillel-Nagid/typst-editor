use std::ops::Range;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cursor {
    pub anchor: usize,
    pub head: usize,
}

impl Cursor {
    pub fn new(position: usize) -> Self {
        Self {
            anchor: position,
            head: position,
        }
    }

    pub fn with_selection(anchor: usize, head: usize) -> Self {
        Self { anchor, head }
    }

    pub fn is_forward(&self) -> bool {
        self.head >= self.anchor
    }

    pub fn range(&self) -> Range<usize> {
        if self.is_forward() {
            self.anchor..self.head
        } else {
            self.head..self.anchor
        }
    }

    pub fn has_selection(&self) -> bool {
        self.anchor != self.head
    }

    pub fn position(&self) -> usize {
        self.head
    }

    pub fn collapse_to(&mut self, position: usize) {
        self.anchor = position;
        self.head = position;
    }
}
