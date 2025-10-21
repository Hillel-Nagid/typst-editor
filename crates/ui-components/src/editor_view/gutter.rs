//! Gutter component for line numbers, folding markers, and diagnostics
//!
//! Phase 3.1: Editor View Component Hierarchy

use gpui::*;
use editor_core::Position;
use crate::decorations::{ GutterDecoration, GitDiffKind };

/// Gutter component showing line numbers and decorations
pub struct Gutter {
    /// Width of the gutter in pixels
    pub width: f32,
    /// Show line numbers
    pub show_line_numbers: bool,
    /// Show folding markers
    pub show_folding: bool,
    /// Show git diff indicators
    pub show_git_diff: bool,
    /// Relative line numbers (Vim-style)
    pub relative_line_numbers: bool,
}

impl Gutter {
    pub fn new() -> Self {
        Self {
            width: 50.0,
            show_line_numbers: true,
            show_folding: true,
            show_git_diff: true,
            relative_line_numbers: false,
        }
    }

    /// Calculate gutter width based on line count and decorations
    pub fn calculate_width(&self, _line_count: usize) -> f32 {
        todo!("Calculate gutter width based on max line number and decorations")
    }

    /// Render the gutter for a specific line
    pub fn render_line(&self, _line: usize, _decorations: &[GutterDecoration]) {
        todo!("Render gutter decorations for a line")
    }

    /// Set gutter width
    pub fn set_width(&mut self, width: f32) {
        self.width = width;
    }

    /// Toggle line numbers
    pub fn toggle_line_numbers(&mut self) {
        self.show_line_numbers = !self.show_line_numbers;
    }

    /// Toggle relative line numbers
    pub fn toggle_relative_line_numbers(&mut self) {
        self.relative_line_numbers = !self.relative_line_numbers;
    }

    /// Handle click in gutter (for folding etc.)
    pub fn handle_click(
        &self,
        _position: Point<Pixels>,
        _line_height: f32
    ) -> Option<GutterAction> {
        todo!("Handle mouse clicks in gutter")
    }
}

impl Default for Gutter {
    fn default() -> Self {
        Self::new()
    }
}

/// Action triggered by gutter interaction
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GutterAction {
    /// Toggle fold at line
    ToggleFold(usize),
    /// Show git diff details
    ShowGitDiff(usize),
    /// Select line
    SelectLine(usize),
}

/// Line number display
pub struct LineNumbers {
    /// Current line (for highlighting)
    pub current_line: Option<usize>,
}

impl LineNumbers {
    pub fn new() -> Self {
        Self {
            current_line: None,
        }
    }

    /// Render line number for a specific line
    pub fn render(&self, _line: usize, _relative_to: Option<usize>) -> String {
        todo!("Format line number for display")
    }

    /// Set current line for highlighting
    pub fn set_current_line(&mut self, line: usize) {
        self.current_line = Some(line);
    }
}

impl Default for LineNumbers {
    fn default() -> Self {
        Self::new()
    }
}

/// Folding markers (expand/collapse indicators)
pub struct FoldingMarkers {
    /// Folded line ranges
    pub folded_ranges: Vec<(usize, usize)>,
}

impl FoldingMarkers {
    pub fn new() -> Self {
        Self {
            folded_ranges: Vec::new(),
        }
    }

    /// Check if a line can be folded
    pub fn can_fold(&self, _line: usize) -> bool {
        todo!("Check if line has foldable content")
    }

    /// Check if a line is folded
    pub fn is_folded(&self, line: usize) -> bool {
        self.folded_ranges.iter().any(|(start, end)| line >= *start && line <= *end)
    }

    /// Toggle fold at line
    pub fn toggle_fold(&mut self, _line: usize) {
        todo!("Toggle fold state at line")
    }

    /// Fold all foldable regions
    pub fn fold_all(&mut self) {
        todo!("Fold all foldable regions")
    }

    /// Unfold all regions
    pub fn unfold_all(&mut self) {
        self.folded_ranges.clear();
    }
}

impl Default for FoldingMarkers {
    fn default() -> Self {
        Self::new()
    }
}

/// marker indicators
pub struct Markers {
    pub markers: Vec<(usize, MarkerKind)>,
}

impl Markers {
    pub fn new() -> Self {
        Self {
            markers: Vec::new(),
        }
    }
}

impl Default for Markers {
    fn default() -> Self {
        Self::new()
    }
}

/// Marker kind
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MarkerKind {
    Bookmark,
    Error,
    Warning,
}

/// Git diff indicators (added, modified, deleted lines)
pub struct GitDiffIndicators {
    /// Git diff information per line
    pub diffs: Vec<(usize, GitDiffKind)>,
}

impl GitDiffIndicators {
    pub fn new() -> Self {
        Self {
            diffs: Vec::new(),
        }
    }

    /// Set git diff information
    pub fn set_diffs(&mut self, diffs: Vec<(usize, GitDiffKind)>) {
        self.diffs = diffs;
    }

    /// Get diff kind for line
    pub fn get_diff(&self, line: usize) -> Option<GitDiffKind> {
        self.diffs
            .iter()
            .find(|(l, _)| *l == line)
            .map(|(_, kind)| *kind)
    }

    /// Clear all diff information
    pub fn clear(&mut self) {
        self.diffs.clear();
    }

    /// Render diff indicator for line
    pub fn render(&self, _line: usize) {
        todo!("Render git diff indicator")
    }
}

impl Default for GitDiffIndicators {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gutter_creation() {
        let gutter = Gutter::new();
        assert!(gutter.show_line_numbers);
        assert!(gutter.show_folding);
        assert!(gutter.show_git_diff);
        assert!(!gutter.relative_line_numbers);
    }

    #[test]
    fn test_gutter_toggle_line_numbers() {
        let mut gutter = Gutter::new();
        gutter.toggle_line_numbers();
        assert!(!gutter.show_line_numbers);
        gutter.toggle_line_numbers();
        assert!(gutter.show_line_numbers);
    }

    #[test]
    fn test_line_numbers_current_line() {
        let mut line_numbers = LineNumbers::new();
        assert_eq!(line_numbers.current_line, None);
        line_numbers.set_current_line(5);
        assert_eq!(line_numbers.current_line, Some(5));
    }

    #[test]
    fn test_folding_markers() {
        let mut markers = FoldingMarkers::new();
        assert!(!markers.is_folded(5));
        markers.folded_ranges.push((5, 10));
        assert!(markers.is_folded(7));
        assert!(!markers.is_folded(11));
        markers.unfold_all();
        assert!(markers.folded_ranges.is_empty());
    }

    #[test]
    fn test_git_diff_indicators() {
        let mut indicators = GitDiffIndicators::new();
        indicators.set_diffs(
            vec![(1, GitDiffKind::Added), (2, GitDiffKind::Modified), (3, GitDiffKind::Deleted)]
        );
        assert_eq!(indicators.get_diff(1), Some(GitDiffKind::Added));
        assert_eq!(indicators.get_diff(2), Some(GitDiffKind::Modified));
        assert_eq!(indicators.get_diff(3), Some(GitDiffKind::Deleted));
        assert_eq!(indicators.get_diff(4), None);
        indicators.clear();
        assert!(indicators.diffs.is_empty());
    }
}
