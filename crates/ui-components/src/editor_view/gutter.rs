//! Gutter component for line numbers, folding markers, and diagnostics
//!
//! Phase 3.1: Editor View Component Hierarchy

use gpui::*;
use crate::decorations::{ GutterDecoration, GutterDecorationKind, GitDiffKind };

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
    pub fn calculate_width(&self, line_count: usize) -> f32 {
        let mut width = 0.0;

        if self.show_line_numbers {
            // Calculate digits needed for line numbers
            let digits = line_count.to_string().len();

            // Estimate character width (assuming monospace digit width of 6-8px)
            let char_width = 7.0;
            let number_width = (digits as f32) * char_width;

            // Add padding: 8px left + 8px right
            width += number_width + 16.0;
        }

        // Add space for folding markers if enabled
        if self.show_folding {
            width += 16.0;
        }

        // Add space for git diff indicators if enabled
        if self.show_git_diff {
            width += 4.0;
        }

        // Add space for diagnostic icons (error/warning) if enabled
        width += 16.0;

        width.max(self.width)
    }

    /// Render the gutter for a specific line
    pub fn render_line(&self, line: usize, decorations: &[GutterDecoration]) {
        // Calculate Y position: line * line_height (would need to be passed in)
        // This is a placeholder for rendering logic

        // Filter decorations for this line
        let line_decorations: Vec<&GutterDecoration> = decorations
            .iter()
            .filter(|dec| dec.line == line)
            .collect();

        // Render each decoration type
        for decoration in line_decorations {
            match &decoration.kind {
                GutterDecorationKind::LineNumber => {
                    // Format and render line number
                    // Would call LineNumbers::render()
                }
                GutterDecorationKind::FoldingMarker { folded: _ } => {
                    // Render triangle icon: ▼ expanded, ▶ collapsed
                    // Position: to the left of line numbers
                }
                GutterDecorationKind::Diagnostic(_severity) => {
                    // Choose icon and color based on severity
                    // Position: to the left of line numbers
                }
                GutterDecorationKind::GitDiff(_kind) => {
                    // Draw colored bar: green=added, blue=modified, red=deleted
                    // Position: at far left edge
                }
            }
        }
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
    pub fn handle_click(&self, position: Point<Pixels>, line_height: f32) -> Option<GutterAction> {
        let y_pos: f32 = position.y.into();

        // Calculate which line was clicked
        let line = (y_pos / line_height).floor() as usize;

        let x_pos: f32 = position.x.into();

        // Determine which gutter element was clicked based on X coordinate
        let mut offset = 0.0;

        // Check if in folding area (0-16px)
        if self.show_folding {
            if x_pos >= offset && x_pos < offset + 16.0 {
                return Some(GutterAction::ToggleFold(line));
            }
            offset += 16.0;
        }

        // Check if in git diff area (if enabled)
        if self.show_git_diff {
            if x_pos >= offset && x_pos < offset + 4.0 {
                return Some(GutterAction::ShowGitDiff(line));
            }
            offset += 4.0;
        }

        // Check if in line number area
        if self.show_line_numbers {
            // Line numbers take up most of the width
            if x_pos >= offset && x_pos < offset + (self.width - offset) {
                return Some(GutterAction::SelectLine(line));
            }
        }

        None
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
    pub fn render(&self, line: usize, relative_to: Option<usize>) -> String {
        if let Some(relative) = relative_to {
            // Calculate relative line number
            let diff = line.abs_diff(relative);
            if diff == 0 {
                line.to_string() // Current line shows absolute number
            } else {
                diff.to_string()
            }
        } else {
            // Absolute line number
            (line + 1).to_string() // Convert from 0-indexed to 1-indexed
        }
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
mod tests {}
