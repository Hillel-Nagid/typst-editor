//! Main text content rendering component
//!
//! Phase 3.1: Editor View Component Hierarchy

use gpui::*;
use editor_core::{ Buffer, Position, SelectionSet };
use crate::rendering::Viewport;

/// Text content area - the main editor canvas
pub struct TextContent {
    /// Viewport for virtual scrolling
    pub viewport: Viewport,
    /// Line height in pixels
    pub line_height: f32,
    /// Character width (for monospace fonts)
    pub char_width: f32,
    /// Tab size in spaces
    pub tab_size: usize,
    /// Word wrap enabled
    pub word_wrap: bool,
}

impl TextContent {
    pub fn new() -> Self {
        Self {
            viewport: Viewport::new(),
            line_height: 20.0,
            char_width: 8.0,
            tab_size: 4,
            word_wrap: false,
        }
    }

    /// Render visible lines
    pub fn render_visible_lines(&self, buffer: &Buffer, _selections: &SelectionSet) {
        let (padded_first_line, padded_last_line) = self.visible_lines();

        for line in padded_first_line..=padded_last_line.min(buffer.len_lines().saturating_sub(1)) {
            if let Ok(_line_text) = buffer.line(line as usize) {
                // Rendering is driven by higher-level systems (LineRenderer, shaders).
                // This method intentionally leaves rendering details to those systems.
                // For now we simply iterate the visible lines to ensure callers can
                // request line text and perform any per-line processing if needed.
            }
        }
    }

    /// Calculate which lines are visible
    pub fn visible_lines(&self) -> (usize, usize) {
        let bounds = self.viewport.bounds;

        // Get scroll position and viewport height as f32
        let scroll_y: f32 = bounds.origin.y.into();
        let viewport_height: f32 = bounds.size.height.into();

        if self.line_height <= 0.0 {
            return (0, 0);
        }

        // Calculate visible line range based on scroll position
        let first_line_f = (scroll_y / self.line_height).floor();
        let first_line: usize = first_line_f as usize;

        let last_line_f = ((scroll_y + viewport_height) / self.line_height).ceil();
        let last_line: usize = last_line_f as usize;

        // Add padding for smooth scrolling
        let padded_first_line = first_line.saturating_sub(3);
        let padded_last_line = last_line + 3;

        (padded_first_line, padded_last_line)
    }

    /// Convert screen position to buffer position
    pub fn screen_to_buffer_position(&self, screen_pos: Point<Pixels>) -> Position {
        let bounds = self.viewport.bounds;

        // Convert screen coordinates to f32
        let screen_x: f32 = screen_pos.x.into();
        let screen_y: f32 = screen_pos.y.into();

        // Subtract viewport offset
        let adjusted_y: f32 = bounds.origin.y.into();
        let adjusted_x: f32 = bounds.origin.x.into();

        // Calculate line number
        let line = ((screen_y - adjusted_y) / self.line_height).floor() as usize;

        // Calculate column (for now, simple monospace calculation)
        // TODO: Handle word wrap and bidirectional text
        let column = ((screen_x - adjusted_x) / self.char_width).floor() as usize;

        Position::new(line, column)
    }

    /// Convert buffer position to screen position
    pub fn buffer_to_screen_position(&self, pos: &Position) -> Point<Pixels> {
        let bounds = self.viewport.bounds;

        // Calculate base Y coordinate: line * line_height
        let y_pos = (pos.line as f32) * self.line_height;

        // Calculate X coordinate: column * char_width
        // TODO: Handle word wrap, variable-width fonts, and bidirectional text
        let x_pos = (pos.column as f32) * self.char_width;

        // Add viewport offset
        let bounds_x: f32 = bounds.origin.x.into();
        let bounds_y: f32 = bounds.origin.y.into();

        point(px(x_pos + bounds_x), px(y_pos + bounds_y))
    }

    /// Set line height
    pub fn set_line_height(&mut self, height: f32) {
        self.line_height = height;
    }

    /// Set character width
    pub fn set_char_width(&mut self, width: f32) {
        self.char_width = width;
    }

    /// Toggle word wrap
    pub fn toggle_word_wrap(&mut self) {
        self.word_wrap = !self.word_wrap;
    }

    /// Scroll to make position visible
    pub fn scroll_to_position(&mut self, position: &Position) {
        let bounds = self.viewport.bounds;

        // Calculate target Y position
        let target_y = (position.line as f32) * self.line_height;

        // Add small padding to keep line visible
        let padding = self.line_height * 2.0;

        // Adjust scroll if position is outside visible area
        let current_y: f32 = bounds.origin.y.into();
        let viewport_height: f32 = bounds.size.height.into();

        if target_y < current_y {
            // Line is above viewport
            self.viewport.scroll_offset.y = px(target_y - padding);
        } else if target_y + self.line_height > current_y + viewport_height {
            // Line is below viewport
            self.viewport.scroll_offset.y = px(
                target_y - viewport_height + self.line_height + padding
            );
        }
    }

    /// Handle scroll event
    pub fn handle_scroll(&mut self, delta: Point<Pixels>) {
        // Add delta to current scroll offset
        let delta_y: f32 = delta.y.into();

        let current_y: f32 = self.viewport.scroll_offset.y.into();

        // Update scroll offset (horizontal scrolling will be implemented later)
        self.viewport.scroll_offset.y = px((current_y + delta_y).max(0.0));

        // TODO: Add clamping based on content bounds
        // TODO: Trigger re-render of affected areas
    }
}

impl Default for TextContent {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {}
