use crate::buffer::grapheme::{grapheme_at_char_offset, next_grapheme_offset, prev_grapheme_offset};
use crate::buffer::line_ending::detect_line_ending;
use crate::buffer::text_buffer::TextBuffer;
use crate::document::LineEnding;
use ropey::Rope;
use std::ops::Range;

#[derive(Clone)]
pub struct RopeBuffer {
    rope: Rope,
    dirty: bool,
    line_ending: LineEnding,
}

impl RopeBuffer {
    pub fn new(text: &str) -> Self {
        Self {
            rope: Rope::from_str(text),
            dirty: false,
            line_ending: detect_line_ending(text),
        }
    }

    pub fn empty() -> Self {
        Self {
            rope: Rope::new(),
            dirty: false,
            line_ending: LineEnding::default_for_platform(),
        }
    }

    pub fn line_ending(&self) -> LineEnding {
        self.line_ending
    }

    pub fn set_line_ending(&mut self, line_ending: LineEnding) {
        self.line_ending = line_ending;
    }

    fn clamp_offset(&self, offset: usize) -> usize {
        offset.min(self.rope.len_chars())
    }
}

impl TextBuffer for RopeBuffer {
    fn insert(&mut self, position: usize, text: &str) {
        self.rope.insert(self.clamp_offset(position), text);
        self.dirty = true;
    }

    fn delete(&mut self, range: Range<usize>) {
        let start = self.clamp_offset(range.start);
        let end = self.clamp_offset(range.end);
        if start < end {
            self.rope.remove(start..end);
            self.dirty = true;
        }
    }

    fn replace(&mut self, range: Range<usize>, text: &str) {
        self.delete(range.clone());
        self.insert(range.start, text);
    }

    fn text(&self) -> String {
        self.rope.to_string()
    }

    fn text_range(&self, range: Range<usize>) -> String {
        let start = self.clamp_offset(range.start);
        let end = self.clamp_offset(range.end);
        if start >= end {
            return String::new();
        }
        self.rope.slice(start..end).to_string()
    }

    fn line_count(&self) -> usize {
        self.rope.len_lines()
    }

    fn line(&self, line_index: usize) -> Option<String> {
        if line_index < self.rope.len_lines() {
            Some(self.rope.line(line_index).to_string())
        } else {
            None
        }
    }

    fn line_range(&self, line_index: usize) -> Option<Range<usize>> {
        if line_index >= self.rope.len_lines() {
            return None;
        }
        let start = self.rope.line_to_char(line_index);
        let end = if line_index + 1 < self.rope.len_lines() {
            self.rope.line_to_char(line_index + 1)
        } else {
            self.rope.len_chars()
        };
        Some(start..end)
    }

    fn offset_to_line_col(&self, offset: usize) -> (usize, usize) {
        let clamped = self.clamp_offset(offset);
        let line = self.rope.char_to_line(clamped);
        let line_start = self.rope.line_to_char(line);
        (line, clamped - line_start)
    }

    fn line_col_to_offset(&self, line: usize, col: usize) -> usize {
        if line >= self.rope.len_lines() {
            return self.rope.len_chars();
        }
        let start = self.rope.line_to_char(line);
        let end = if line + 1 < self.rope.len_lines() {
            self.rope.line_to_char(line + 1)
        } else {
            self.rope.len_chars()
        };
        (start + col).min(end)
    }

    fn char_at(&self, offset: usize) -> Option<char> {
        let clamped = self.clamp_offset(offset);
        if clamped >= self.rope.len_chars() {
            return None;
        }
        self.rope.get_char(clamped)
    }

    fn grapheme_at(&self, offset: usize) -> Option<String> {
        grapheme_at_char_offset(&self.text(), self.clamp_offset(offset))
    }

    fn next_grapheme(&self, offset: usize) -> usize {
        next_grapheme_offset(&self.text(), self.clamp_offset(offset))
    }

    fn prev_grapheme(&self, offset: usize) -> usize {
        prev_grapheme_offset(&self.text(), self.clamp_offset(offset))
    }

    fn len(&self) -> usize {
        self.rope.len_chars()
    }

    fn is_empty(&self) -> bool {
        self.rope.len_chars() == 0
    }

    fn is_dirty(&self) -> bool {
        self.dirty
    }

    fn set_dirty(&mut self, dirty: bool) {
        self.dirty = dirty;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grapheme_navigation_handles_emoji_cluster() {
        let mut buffer = RopeBuffer::new("a👩🏽‍💻b");
        let start = 1;
        let next = buffer.next_grapheme(start);
        assert!(next > start);
        assert_eq!(buffer.grapheme_at(start), Some("👩🏽‍💻".to_string()));
        assert_eq!(buffer.prev_grapheme(next), start);
        buffer.insert(next, "x");
        assert!(buffer.is_dirty());
    }

    #[test]
    fn line_col_roundtrip() {
        let buffer = RopeBuffer::new("ab\ncd\n");
        let offset = buffer.line_col_to_offset(1, 1);
        assert_eq!(buffer.offset_to_line_col(offset), (1, 1));
    }
}
