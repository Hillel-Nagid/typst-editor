use crate::buffer::TextBuffer;
use std::ops::Range;

#[derive(Debug, Clone)]
pub struct TextEdit {
    pub range: Range<usize>,
    pub insert_text: String,
}

pub fn apply_edits_descending(buffer: &mut dyn TextBuffer, edits: &mut [TextEdit]) {
    edits.sort_by(|a, b| b.range.start.cmp(&a.range.start));
    for edit in edits {
        if !edit.range.is_empty() {
            buffer.delete(edit.range.clone());
        }
        if !edit.insert_text.is_empty() {
            buffer.insert(edit.range.start, &edit.insert_text);
        }
    }
}
