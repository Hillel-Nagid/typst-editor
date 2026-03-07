use std::ops::Range;

pub trait TextBuffer: Send + Sync {
    fn insert(&mut self, position: usize, text: &str);
    fn delete(&mut self, range: Range<usize>);
    fn replace(&mut self, range: Range<usize>, text: &str);
    fn text(&self) -> String;
    fn text_range(&self, range: Range<usize>) -> String;

    fn line_count(&self) -> usize;
    fn line(&self, line_index: usize) -> Option<String>;
    fn line_range(&self, line_index: usize) -> Option<Range<usize>>;
    fn offset_to_line_col(&self, offset: usize) -> (usize, usize);
    fn line_col_to_offset(&self, line: usize, col: usize) -> usize;

    fn char_at(&self, offset: usize) -> Option<char>;
    fn grapheme_at(&self, offset: usize) -> Option<String>;
    fn next_grapheme(&self, offset: usize) -> usize;
    fn prev_grapheme(&self, offset: usize) -> usize;

    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
    fn is_dirty(&self) -> bool;
    fn set_dirty(&mut self, dirty: bool);
}
