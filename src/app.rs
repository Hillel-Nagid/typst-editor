//! Main application structure

use crate::state::{ ApplicationState, WindowState };
use editor_core::{ Buffer, BufferId };
use std::path::PathBuf;

/// The main Typst Editor application
pub struct TypstEditor {
    /// Application state
    pub state: ApplicationState,
    /// Buffer registry
    buffers: std::collections::HashMap<BufferId, Buffer>,
    /// Next buffer ID
    next_buffer_id: u64,
}

impl TypstEditor {
    pub fn new() -> Self {
        Self {
            state: ApplicationState::new(),
            buffers: std::collections::HashMap::new(),
            next_buffer_id: 1,
        }
    }

    /// Create a new buffer
    pub fn create_buffer(&mut self, text: &str) -> BufferId {
        let id = BufferId::new(self.next_buffer_id);
        self.next_buffer_id += 1;

        let buffer = Buffer::from_text(id, text);
        self.buffers.insert(id, buffer);

        id
    }

    /// Get a buffer by ID
    pub fn get_buffer(&self, id: BufferId) -> Option<&Buffer> {
        self.buffers.get(&id)
    }

    /// Get a mutable buffer by ID
    pub fn get_buffer_mut(&mut self, id: BufferId) -> Option<&mut Buffer> {
        self.buffers.get_mut(&id)
    }

    /// Open a file
    pub fn open_file(&mut self, path: PathBuf) -> Result<BufferId, std::io::Error> {
        let id = BufferId::new(self.next_buffer_id);
        self.next_buffer_id += 1;

        let buffer = Buffer::from_file(id, path.clone())?;
        self.buffers.insert(id, buffer);

        self.state.add_recent_file(path);

        Ok(id)
    }

    /// Create a new window
    pub fn new_window(&mut self) -> usize {
        let window_id = self.state.windows.len();
        let window = WindowState::new(window_id);
        self.state.add_window(window);
        window_id
    }

    /// Get application state
    pub fn state(&self) -> &ApplicationState {
        &self.state
    }

    /// Get mutable application state
    pub fn state_mut(&mut self) -> &mut ApplicationState {
        &mut self.state
    }
}

impl Default for TypstEditor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_creation() {
        let app = TypstEditor::new();
        assert_eq!(app.buffers.len(), 0);
    }

    #[test]
    fn test_create_buffer() {
        let mut app = TypstEditor::new();
        let id = app.create_buffer("Hello World");

        let buffer = app.get_buffer(id).unwrap();
        assert_eq!(buffer.text(), "Hello World");
    }

    #[test]
    fn test_create_window() {
        let mut app = TypstEditor::new();
        let window_id = app.new_window();
        assert_eq!(window_id, 0);
        assert_eq!(app.state.windows.len(), 1);
    }
}
