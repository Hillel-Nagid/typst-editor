use crate::selection::cursor::Cursor;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MultiCursor {
    cursors: Vec<Cursor>,
    primary: usize,
}

impl MultiCursor {
    pub fn new(position: usize) -> Self {
        Self {
            cursors: vec![Cursor::new(position)],
            primary: 0,
        }
    }

    pub fn from_cursors(cursors: Vec<Cursor>) -> Self {
        let mut this = Self {
            cursors: if cursors.is_empty() {
                vec![Cursor::new(0)]
            } else {
                cursors
            },
            primary: 0,
        };
        this.normalize();
        this
    }

    pub fn cursors(&self) -> &[Cursor] {
        &self.cursors
    }

    pub fn cursors_mut(&mut self) -> &mut [Cursor] {
        &mut self.cursors
    }

    pub fn primary_index(&self) -> usize {
        self.primary
    }

    pub fn primary_cursor(&self) -> &Cursor {
        &self.cursors[self.primary]
    }

    pub fn set_single(&mut self, position: usize) {
        self.cursors.clear();
        self.cursors.push(Cursor::new(position));
        self.primary = 0;
    }

    pub fn set_cursors(&mut self, cursors: Vec<Cursor>) {
        self.cursors = if cursors.is_empty() {
            vec![Cursor::new(0)]
        } else {
            cursors
        };
        self.normalize();
    }

    pub fn add_cursor(&mut self, cursor: Cursor) {
        self.cursors.push(cursor);
        self.normalize();
    }

    pub fn clear_secondary(&mut self) {
        let primary = self.primary_cursor().to_owned();
        self.cursors = vec![primary];
        self.primary = 0;
    }

    pub fn remove_last_secondary(&mut self) {
        if self.cursors.len() > 1 {
            self.cursors.pop();
            self.primary = self.primary.min(self.cursors.len() - 1);
        }
    }

    pub fn ordered_desc(&self) -> Vec<Cursor> {
        let mut cursors = self.cursors.clone();
        cursors.sort_by(|a, b| b.range().start.cmp(&a.range().start));
        cursors
    }

    fn normalize(&mut self) {
        let primary_head = self.cursors[self.primary.min(self.cursors.len() - 1)].head;
        self.cursors.sort_by_key(|c| c.range().start);

        let mut merged: Vec<Cursor> = Vec::new();
        for cursor in self.cursors.iter().copied() {
            if let Some(last) = merged.last_mut() {
                let last_range = last.range();
                let range = cursor.range();
                if range.start <= last_range.end {
                    let start = last_range.start.min(range.start);
                    let end = last_range.end.max(range.end);
                    *last = Cursor::with_selection(start, end);
                    continue;
                }
            }
            merged.push(cursor);
        }

        if merged.is_empty() {
            merged.push(Cursor::new(0));
        }
        self.cursors = merged;

        self.primary = self
            .cursors
            .iter()
            .enumerate()
            .min_by_key(|(_, c)| c.head.abs_diff(primary_head))
            .map(|(idx, _)| idx)
            .unwrap_or(0);
    }
}

impl Default for MultiCursor {
    fn default() -> Self {
        Self::new(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn merges_overlapping_ranges() {
        let cursors = vec![
            Cursor::with_selection(2, 6),
            Cursor::with_selection(4, 8),
            Cursor::new(12),
        ];
        let multi = MultiCursor::from_cursors(cursors);
        assert_eq!(multi.cursors().len(), 2);
        assert_eq!(multi.cursors()[0].range(), 2..8);
        assert_eq!(multi.cursors()[1].range(), 12..12);
    }

    #[test]
    fn returns_descending_by_edit_start() {
        let multi = MultiCursor::from_cursors(vec![Cursor::new(2), Cursor::new(10), Cursor::new(6)]);
        let ordered = multi.ordered_desc();
        assert_eq!(ordered[0].position(), 10);
        assert_eq!(ordered[1].position(), 6);
        assert_eq!(ordered[2].position(), 2);
    }
}
