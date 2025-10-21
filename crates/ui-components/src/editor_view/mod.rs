//! Editor view components
//!
//! Phase 3.1: Editor View Component Hierarchy

use editor_core::BufferId;

pub mod gutter;
pub mod text_content;
pub mod line_renderer;
pub mod cursor_renderer;
pub mod scrollbar;
pub mod overlays;
pub mod status_bar;

pub use gutter::Gutter;
pub use text_content::TextContent;
pub use line_renderer::LineRenderer;
pub use cursor_renderer::{
    CursorRenderer,
    CursorShape,
    CursorStyle,
    PrimaryCursor,
    SecondaryCursors,
};
pub use scrollbar::ScrollBar;
pub use overlays::Overlays;
pub use status_bar::StatusBar;

/// Editor view component - the main editor interface
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
