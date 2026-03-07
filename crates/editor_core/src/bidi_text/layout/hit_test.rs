use crate::bidi_text::layout::visual_line::VisualLine;

pub fn visual_to_logical(line: &VisualLine, visual_x: f32) -> usize {
    line.visual_to_logical(visual_x)
}
