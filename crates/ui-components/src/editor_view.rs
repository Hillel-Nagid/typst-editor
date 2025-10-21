//! Editor view component

use editor_core::BufferId;

/// Editor view component
pub struct EditorView {
    /// Current buffer
    buffer_id: Option<BufferId>,
    /// View state
    scroll_offset: f32,
}

impl EditorView {
    pub fn new() -> Self {
        Self {
            buffer_id: None,
            scroll_offset: 0.0,
        }
    }

    pub fn set_buffer(&mut self, buffer_id: BufferId) {
        self.buffer_id = Some(buffer_id);
    }

    pub fn buffer_id(&self) -> Option<BufferId> {
        self.buffer_id
    }
}

impl Default for EditorView {
    fn default() -> Self {
        Self::new()
    }
}
