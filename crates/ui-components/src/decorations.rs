//! Decorations and annotations for the editor
//!
//! Phase 3.5: Decorations and Annotations

use palette::Srgb;
use std::ops::Range;

/// Decoration manager
pub struct DecorationManager {
    inline_decorations: Vec<InlineDecoration>,
    gutter_decorations: Vec<GutterDecoration>,
    highlight_ranges: Vec<HighlightRange>,
}

impl DecorationManager {
    pub fn new() -> Self {
        Self {
            inline_decorations: Vec::new(),
            gutter_decorations: Vec::new(),
            highlight_ranges: Vec::new(),
        }
    }

    /// Add an inline decoration
    pub fn add_inline(&mut self, decoration: InlineDecoration) {
        self.inline_decorations.push(decoration);
    }

    /// Add a gutter decoration
    pub fn add_gutter(&mut self, decoration: GutterDecoration) {
        self.gutter_decorations.push(decoration);
    }

    /// Add a highlight range
    pub fn add_highlight(&mut self, range: HighlightRange) {
        self.highlight_ranges.push(range);
    }

    /// Clear all decorations
    pub fn clear(&mut self) {
        self.inline_decorations.clear();
        self.gutter_decorations.clear();
        self.highlight_ranges.clear();
    }
}

impl Default for DecorationManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Inline decoration (squiggles, code lens, hints)
#[derive(Debug, Clone)]
pub struct InlineDecoration {
    pub range: Range<usize>,
    pub kind: InlineDecorationKind,
}

/// Type of inline decoration
#[derive(Debug, Clone)]
pub enum InlineDecorationKind {
    /// Error squiggle (red wavy underline)
    ErrorSquiggle,
    /// Warning squiggle (yellow wavy underline)
    WarningSquiggle,
    /// Info squiggle (blue wavy underline)
    InfoSquiggle,
    /// Hint squiggle (gray wavy underline)
    HintSquiggle,
    /// Code lens above the line
    CodeLens(String),
    /// Inline hint (parameter name, type hint)
    InlineHint(String),
    /// Matching bracket highlight
    MatchingBracket,
}

/// Gutter decoration (line numbers, icons, etc.)
#[derive(Debug, Clone)]
pub struct GutterDecoration {
    pub line: usize,
    pub kind: GutterDecorationKind,
}

/// Type of gutter decoration
#[derive(Debug, Clone)]
pub enum GutterDecorationKind {
    /// Line number (always present)
    LineNumber,
    /// Folding marker (expand/collapse)
    FoldingMarker {
        folded: bool,
    },
    /// Diagnostic marker
    Diagnostic(DiagnosticSeverity),
    /// Git diff indicator
    GitDiff(GitDiffKind),
}

/// Diagnostic severity level
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticSeverity {
    Error,
    Warning,
    Info,
    Hint,
}

/// Git diff kind
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GitDiffKind {
    Added,
    Modified,
    Deleted,
}

/// Highlight range for selections, search results, etc.
#[derive(Debug, Clone)]
pub struct HighlightRange {
    pub range: Range<usize>,
    pub kind: HighlightKind,
}

/// Type of highlight
#[derive(Debug, Clone)]
pub enum HighlightKind {
    /// Current line highlight
    CurrentLine,
    /// Selection highlight
    Selection,
    /// Search result highlight
    SearchResult,
    /// Write occurrence (when cursor on symbol)
    WriteOccurrence,
    /// Read occurrence (when cursor on symbol)
    ReadOccurrence,
    /// Custom highlight with color
    Custom(Srgb),
}
