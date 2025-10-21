//! Status bar component showing editor information
//!
//! Phase 3.1: Editor View Component Hierarchy

use gpui::*;
use editor_core::Position;

/// Status bar at bottom of editor
pub struct StatusBar {
    /// Position indicator
    pub position: PositionIndicator,
    /// Selection info
    pub selection: SelectionInfo,
    /// Encoding display
    pub encoding: EncodingDisplay,
    /// Language mode
    pub language: LanguageMode,
}

impl StatusBar {
    pub fn new() -> Self {
        Self {
            position: PositionIndicator::default(),
            selection: SelectionInfo::default(),
            encoding: EncodingDisplay::default(),
            language: LanguageMode::default(),
        }
    }

    /// Update all components
    pub fn update(&mut self, _position: &Position, _selection_size: usize) {
        todo!("Update status bar components")
    }

    /// Render status bar
    pub fn render(&self) {
        todo!("Render status bar")
    }
}

impl Default for StatusBar {
    fn default() -> Self {
        Self::new()
    }
}

/// Position indicator (line:column)
#[derive(Debug, Clone)]
pub struct PositionIndicator {
    /// Current line (1-indexed for display)
    pub line: usize,
    /// Current column (1-indexed for display)
    pub column: usize,
}

impl PositionIndicator {
    pub fn new() -> Self {
        Self { line: 1, column: 1 }
    }

    /// Update position
    pub fn update(&mut self, position: &Position) {
        self.line = position.line + 1; // Convert to 1-indexed
        self.column = position.column + 1;
    }

    /// Format as string (e.g., "Ln 5, Col 12")
    pub fn format(&self) -> String {
        format!("Ln {}, Col {}", self.line, self.column)
    }
}

impl Default for PositionIndicator {
    fn default() -> Self {
        Self::new()
    }
}

/// Selection info (characters selected)
#[derive(Debug, Clone)]
pub struct SelectionInfo {
    /// Number of characters selected
    pub char_count: usize,
    /// Number of lines selected
    pub line_count: usize,
}

impl SelectionInfo {
    pub fn new() -> Self {
        Self {
            char_count: 0,
            line_count: 0,
        }
    }

    /// Update selection info
    pub fn update(&mut self, char_count: usize, line_count: usize) {
        self.char_count = char_count;
        self.line_count = line_count;
    }

    /// Format as string (e.g., "5 chars selected")
    pub fn format(&self) -> Option<String> {
        if self.char_count > 0 {
            if self.line_count > 1 {
                Some(format!("{} chars ({} lines)", self.char_count, self.line_count))
            } else {
                Some(format!("{} chars", self.char_count))
            }
        } else {
            None
        }
    }

    /// Clear selection info
    pub fn clear(&mut self) {
        self.char_count = 0;
        self.line_count = 0;
    }
}

impl Default for SelectionInfo {
    fn default() -> Self {
        Self::new()
    }
}

/// Encoding display
#[derive(Debug, Clone)]
pub struct EncodingDisplay {
    /// Current encoding (e.g., "UTF-8")
    pub encoding: String,
}

impl EncodingDisplay {
    pub fn new() -> Self {
        Self {
            encoding: "UTF-8".to_string(),
        }
    }

    /// Set encoding
    pub fn set_encoding(&mut self, encoding: String) {
        self.encoding = encoding;
    }

    /// Format as string
    pub fn format(&self) -> String {
        self.encoding.clone()
    }
}

impl Default for EncodingDisplay {
    fn default() -> Self {
        Self::new()
    }
}

/// Language mode indicator
#[derive(Debug, Clone)]
pub struct LanguageMode {
    /// Current language (e.g., "Typst")
    pub language: String,
}

impl LanguageMode {
    pub fn new() -> Self {
        Self {
            language: "Plain Text".to_string(),
        }
    }

    /// Set language
    pub fn set_language(&mut self, language: String) {
        self.language = language;
    }

    /// Format as string
    pub fn format(&self) -> String {
        self.language.clone()
    }
}

impl Default for LanguageMode {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status_bar_creation() {
        let status_bar = StatusBar::new();
        assert_eq!(status_bar.position.line, 1);
        assert_eq!(status_bar.position.column, 1);
    }

    #[test]
    fn test_position_indicator() {
        let mut indicator = PositionIndicator::new();
        assert_eq!(indicator.format(), "Ln 1, Col 1");
        
        indicator.update(&Position::new(4, 9));
        assert_eq!(indicator.format(), "Ln 5, Col 10");
    }

    #[test]
    fn test_selection_info() {
        let mut info = SelectionInfo::new();
        assert_eq!(info.format(), None);
        
        info.update(10, 1);
        assert_eq!(info.format(), Some("10 chars".to_string()));
        
        info.update(50, 3);
        assert_eq!(info.format(), Some("50 chars (3 lines)".to_string()));
        
        info.clear();
        assert_eq!(info.format(), None);
    }

    #[test]
    fn test_encoding_display() {
        let mut encoding = EncodingDisplay::new();
        assert_eq!(encoding.format(), "UTF-8");
        
        encoding.set_encoding("UTF-16".to_string());
        assert_eq!(encoding.format(), "UTF-16");
    }

    #[test]
    fn test_language_mode() {
        let mut language = LanguageMode::new();
        assert_eq!(language.format(), "Plain Text");
        
        language.set_language("Typst".to_string());
        assert_eq!(language.format(), "Typst");
    }
}
