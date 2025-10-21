//! Text buffer implementation using rope data structure

use crate::{ EditorError, Result, Version };
use crate::selection::Position;
use ropey::Rope;
use serde::{ Deserialize, Serialize };
use std::path::PathBuf;
use unicode_segmentation::UnicodeSegmentation;

/// Unique identifier for a buffer
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BufferId(u64);

impl BufferId {
    pub fn new(id: u64) -> Self {
        Self(id)
    }

    pub fn next(&self) -> Self {
        Self(self.0 + 1)
    }
}

/// Line ending style
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LineEnding {
    /// Unix-style (LF)
    Lf,
    /// Windows-style (CRLF)
    Crlf,
    /// Classic Mac (CR) - rare
    Cr,
}

impl LineEnding {
    pub fn as_str(&self) -> &str {
        match self {
            LineEnding::Lf => "\n",
            LineEnding::Crlf => "\r\n",
            LineEnding::Cr => "\r",
        }
    }

    /// Detect line ending from text content
    pub fn detect(text: &str) -> Self {
        if text.contains("\r\n") {
            LineEnding::Crlf
        } else if text.contains('\n') {
            LineEnding::Lf
        } else if text.contains('\r') {
            LineEnding::Cr
        } else {
            // Default to platform-specific
            #[cfg(windows)]
            return LineEnding::Crlf;
            #[cfg(not(windows))]
            return LineEnding::Lf;
        }
    }
}

/// Immutable snapshot of a buffer at a point in time
#[derive(Clone)]
pub struct BufferSnapshot {
    rope: Rope,
    version: Version,
}

impl BufferSnapshot {
    pub fn text(&self) -> String {
        self.rope.to_string()
    }

    pub fn version(&self) -> Version {
        self.version
    }

    pub fn len_lines(&self) -> usize {
        self.rope.len_lines()
    }

    pub fn len_chars(&self) -> usize {
        self.rope.len_chars()
    }

    pub fn line(&self, line_idx: usize) -> Option<String> {
        if line_idx < self.len_lines() { Some(self.rope.line(line_idx).to_string()) } else { None }
    }
}

/// Metrics about the buffer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BufferMetrics {
    pub total_lines: usize,
    pub total_chars: usize,
    pub total_bytes: usize,
    pub longest_line_length: usize,
}

/// The main text buffer
pub struct Buffer {
    id: BufferId,
    rope: Rope,
    version: Version,
    file_path: Option<PathBuf>,
    line_ending: LineEnding,
    dirty: bool,
    read_only: bool,
}

impl Buffer {
    /// Create a new empty buffer
    pub fn new(id: BufferId) -> Self {
        Self {
            id,
            rope: Rope::new(),
            version: Version::new(),
            file_path: None,
            line_ending: LineEnding::Lf,
            dirty: false,
            read_only: false,
        }
    }

    /// Create a buffer from text content
    pub fn from_text(id: BufferId, text: &str) -> Self {
        let line_ending = LineEnding::detect(text);
        Self {
            id,
            rope: Rope::from_str(text),
            version: Version::new(),
            file_path: None,
            line_ending,
            dirty: false,
            read_only: false,
        }
    }

    /// Create a buffer from a file path
    pub fn from_file(id: BufferId, path: PathBuf) -> std::io::Result<Self> {
        let content = std::fs::read_to_string(&path)?;
        let line_ending = LineEnding::detect(&content);
        Ok(Self {
            id,
            rope: Rope::from_str(&content),
            version: Version::new(),
            file_path: Some(path),
            line_ending,
            dirty: false,
            read_only: false,
        })
    }

    /// Get buffer ID
    pub fn id(&self) -> BufferId {
        self.id
    }

    /// Get current version
    pub fn version(&self) -> Version {
        self.version
    }

    /// Get file path if any
    pub fn file_path(&self) -> Option<&PathBuf> {
        self.file_path.as_ref()
    }

    /// Set file path
    pub fn set_file_path(&mut self, path: PathBuf) {
        self.file_path = Some(path);
    }

    /// Check if buffer has unsaved changes
    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    /// Check if buffer is read-only
    pub fn is_read_only(&self) -> bool {
        self.read_only
    }

    /// Set read-only status
    pub fn set_read_only(&mut self, read_only: bool) {
        self.read_only = read_only;
    }

    /// Get the entire text content
    pub fn text(&self) -> String {
        self.rope.to_string()
    }

    /// Get number of lines
    pub fn len_lines(&self) -> usize {
        self.rope.len_lines()
    }

    /// Get number of characters
    pub fn len_chars(&self) -> usize {
        self.rope.len_chars()
    }

    /// Get number of bytes
    pub fn len_bytes(&self) -> usize {
        self.rope.len_bytes()
    }

    /// Check if buffer is empty
    pub fn is_empty(&self) -> bool {
        self.rope.len_chars() == 0
    }

    /// Get a specific line
    pub fn line(&self, line_idx: usize) -> Result<String> {
        if line_idx < self.len_lines() {
            Ok(self.rope.line(line_idx).to_string())
        } else {
            Err(EditorError::InvalidPosition {
                line: line_idx,
                column: 0,
            })
        }
    }

    /// Get line ending style
    pub fn line_ending(&self) -> LineEnding {
        self.line_ending
    }

    /// Set line ending style
    pub fn set_line_ending(&mut self, ending: LineEnding) {
        self.line_ending = ending;
        self.dirty = true;
    }

    /// Convert position to character index
    pub fn position_to_char_idx(&self, pos: Position) -> Result<usize> {
        if pos.line >= self.len_lines() {
            return Err(EditorError::InvalidPosition {
                line: pos.line,
                column: pos.column,
            });
        }

        let line_start = self.rope.line_to_char(pos.line);
        let line = self.rope.line(pos.line);

        // Count grapheme clusters to respect unicode properly
        let graphemes: Vec<&str> = line.as_str().unwrap_or("").graphemes(true).collect();

        if pos.column > graphemes.len() {
            return Err(EditorError::InvalidPosition {
                line: pos.line,
                column: pos.column,
            });
        }

        let column_offset = graphemes[..pos.column]
            .iter()
            .map(|g| g.chars().count())
            .sum::<usize>();

        Ok(line_start + column_offset)
    }

    /// Convert character index to position
    pub fn char_idx_to_position(&self, idx: usize) -> Result<Position> {
        if idx > self.len_chars() {
            return Err(EditorError::InvalidPosition {
                line: 0,
                column: idx,
            });
        }

        let line = self.rope.char_to_line(idx);
        let line_start = self.rope.line_to_char(line);
        let line_content = self.rope.line(line);

        let char_offset = idx - line_start;
        let graphemes: Vec<&str> = line_content.as_str().unwrap_or("").graphemes(true).collect();

        let mut chars_counted = 0;
        let mut column = 0;
        for grapheme in graphemes {
            if chars_counted >= char_offset {
                break;
            }
            chars_counted += grapheme.chars().count();
            column += 1;
        }

        Ok(Position::new(line, column))
    }

    /// Insert text at a position
    pub fn insert(&mut self, pos: Position, text: &str) -> Result<()> {
        if self.read_only {
            return Err(EditorError::BufferError("Buffer is read-only".to_string()));
        }

        let char_idx = self.position_to_char_idx(pos)?;
        self.rope.insert(char_idx, text);
        self.version = self.version.next();
        self.dirty = true;
        Ok(())
    }

    /// Delete a range of text
    pub fn delete(&mut self, start: Position, end: Position) -> Result<String> {
        if self.read_only {
            return Err(EditorError::BufferError("Buffer is read-only".to_string()));
        }

        let start_idx = self.position_to_char_idx(start)?;
        let end_idx = self.position_to_char_idx(end)?;

        if start_idx > end_idx {
            return Err(
                EditorError::InvalidRange(
                    format!("Start position {:?} is after end position {:?}", start, end)
                )
            );
        }

        let deleted_text = self.rope.slice(start_idx..end_idx).to_string();
        self.rope.remove(start_idx..end_idx);
        self.version = self.version.next();
        self.dirty = true;
        Ok(deleted_text)
    }

    /// Replace a range of text
    pub fn replace(&mut self, start: Position, end: Position, text: &str) -> Result<String> {
        let deleted = self.delete(start, end)?;
        self.insert(start, text)?;
        Ok(deleted)
    }

    /// Save buffer to file
    pub fn save(&mut self) -> std::io::Result<()> {
        if let Some(path) = &self.file_path {
            let content = self.text();
            std::fs::write(path, content)?;
            self.dirty = false;
            Ok(())
        } else {
            //TODO: should implement default fallback save path logic here using `self.save_as()`
            Err(std::io::Error::new(std::io::ErrorKind::NotFound, "No file path set for buffer"))
        }
    }

    /// Save buffer to a specific file
    pub fn save_as(&mut self, path: PathBuf) -> std::io::Result<()> {
        let content = self.text();
        std::fs::write(&path, content)?;
        self.file_path = Some(path);
        self.dirty = false;
        Ok(())
    }

    /// Create an immutable snapshot
    pub fn snapshot(&self) -> BufferSnapshot {
        BufferSnapshot {
            rope: self.rope.clone(),
            version: self.version,
        }
    }

    /// Get buffer metrics
    pub fn metrics(&self) -> BufferMetrics {
        let longest_line = (0..self.len_lines())
            .map(|i| self.rope.line(i).len_chars())
            .max()
            .unwrap_or(0);

        BufferMetrics {
            total_lines: self.len_lines(),
            total_chars: self.len_chars(),
            total_bytes: self.len_bytes(),
            longest_line_length: longest_line,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buffer_creation() {
        let buffer = Buffer::new(BufferId::new(1));
        assert!(buffer.is_empty());
        assert_eq!(buffer.len_lines(), 1); // Rope always has at least 1 line
        assert!(!buffer.is_dirty());
    }

    #[test]
    fn test_buffer_from_text() {
        let text = "Hello\nWorld\n";
        let buffer = Buffer::from_text(BufferId::new(1), text);
        assert_eq!(buffer.len_lines(), 3); // "Hello\n", "World\n", ""
        assert_eq!(buffer.text(), text);
        assert_eq!(buffer.line_ending(), LineEnding::Lf);
    }

    #[test]
    fn test_insert_and_delete() {
        let mut buffer = Buffer::from_text(BufferId::new(1), "Hello World");

        // Insert at position
        buffer.insert(Position::new(0, 5), " Beautiful").unwrap();
        assert_eq!(buffer.text(), "Hello Beautiful World");
        assert!(buffer.is_dirty());

        // Delete range
        let deleted = buffer.delete(Position::new(0, 5), Position::new(0, 15)).unwrap();
        assert_eq!(deleted, " Beautiful");
        assert_eq!(buffer.text(), "Hello World");
    }

    #[test]
    fn test_position_conversion() {
        let buffer = Buffer::from_text(BufferId::new(1), "Hello\nWorld\n");

        let pos = Position::new(1, 2);
        let idx = buffer.position_to_char_idx(pos).unwrap();
        let converted_pos = buffer.char_idx_to_position(idx).unwrap();
        assert_eq!(pos, converted_pos);
    }

    #[test]
    fn test_line_ending_detection() {
        assert_eq!(LineEnding::detect("Hello\nWorld"), LineEnding::Lf);
        assert_eq!(LineEnding::detect("Hello\r\nWorld"), LineEnding::Crlf);
        assert_eq!(LineEnding::detect("Hello\rWorld"), LineEnding::Cr);
    }
}
