use crate::bidi_text::layout::visual_line::VisualLine;

pub fn selection_ranges(line: &VisualLine, start: usize, end: usize) -> Vec<(f32, f32)> {
    line.selection_ranges(start, end)
}
