//! Preview pane component

/// Preview pane component
pub struct PreviewPane {
    /// Whether preview is visible
    visible: bool,
}

impl PreviewPane {
    pub fn new() -> Self {
        Self { visible: true }
    }

    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    pub fn is_visible(&self) -> bool {
        self.visible
    }
}

impl Default for PreviewPane {
    fn default() -> Self {
        Self::new()
    }
}
