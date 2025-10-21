//! UI components for the Typst editor

// pub mod editor-view;  // Can't use hyphens in Rust module names
// pub mod preview-pane;
// pub mod sidebar;
// pub mod panels;

// For now, we'll create placeholder modules
pub mod editor_view;
pub mod preview_pane;
pub mod sidebar;
pub mod panels;

pub use editor_view::EditorView;
pub use preview_pane::PreviewPane;
pub use sidebar::Sidebar;
pub use panels::Panel;
