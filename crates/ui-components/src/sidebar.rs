//! Sidebar component

/// Sidebar component
pub struct Sidebar {
    /// Whether sidebar is visible
    visible: bool,
    /// Sidebar width
    width: f32,
}

impl Sidebar {
    pub fn new() -> Self {
        Self {
            visible: true,
            width: 200.0,
        }
    }

    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    pub fn is_visible(&self) -> bool {
        self.visible
    }

    pub fn set_width(&mut self, width: f32) {
        self.width = width.max(100.0).min(500.0);
    }

    pub fn width(&self) -> f32 {
        self.width
    }
}

impl Default for Sidebar {
    fn default() -> Self {
        Self::new()
    }
}
