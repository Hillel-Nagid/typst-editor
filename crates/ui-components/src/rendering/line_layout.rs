//! Line layout calculation and wrapping
//!
//! Phase 3.2: Text Rendering Pipeline

use bidi_text::*;
use std::ops::Range;

/// Line layout calculator
pub struct LineLayout {
    // TODO: Implement line layout state
}

impl LineLayout {
    pub fn new() -> Self {
        Self {}
    }

    /// Calculate visual lines from a logical line
    pub fn calculate_visual_lines(&self, _text: &str, _max_width: f32) -> Vec<VisualLine> {
        todo!("Implement line layout calculation")
    }
}

impl Default for LineLayout {
    fn default() -> Self {
        Self::new()
    }
}

/// A visual line (may be part of a wrapped logical line)
pub struct VisualLine {
    pub logical_line: usize,
    pub visual_line_index: usize,
    pub char_range: Range<usize>,
    pub pixel_width: f32,
    pub baseline_y: f32,
    pub bidi_runs: Vec<VisualTextRun>,
}

/// A visual text run with direction and styling
pub struct VisualTextRun {
    pub text: String,
    pub direction: Direction,
    pub x_offset: f32,
    pub glyphs: Vec<super::text_shaping::ShapedGlyph>,
    pub style: TextStyle,
}

/// Text direction
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    LTR,
    RTL,
}

/// Text style for a run
pub struct TextStyle {
    pub color: palette::Srgb,
    pub font_family: String,
    pub font_size: f32,
}
