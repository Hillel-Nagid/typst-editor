//! Application state management

use editor_core::BufferId;
use std::path::PathBuf;
use serde::{ Deserialize, Serialize };

/// Global application state
pub struct ApplicationState {
    /// Window management
    pub windows: Vec<WindowState>,
    /// Global settings
    pub settings: Settings,
    /// Recent files
    pub recent_files: Vec<PathBuf>,
    /// Active theme
    pub theme: String,
}

impl ApplicationState {
    pub fn new() -> Self {
        Self {
            windows: Vec::new(),
            settings: Settings::default(),
            recent_files: Vec::new(),
            theme: "dark".to_string(),
        }
    }

    pub fn add_window(&mut self, window: WindowState) {
        self.windows.push(window);
    }

    pub fn add_recent_file(&mut self, path: PathBuf) {
        // Remove if already exists
        self.recent_files.retain(|p| p != &path);
        // Add to front
        self.recent_files.insert(0, path);
        // Keep only last 20
        self.recent_files.truncate(20);
    }
}

impl Default for ApplicationState {
    fn default() -> Self {
        Self::new()
    }
}

/// Window state
pub struct WindowState {
    /// Workspace for this window
    pub workspace: WorkspaceState,
    /// Window ID
    pub id: usize,
}

impl WindowState {
    pub fn new(id: usize) -> Self {
        Self {
            workspace: WorkspaceState::new(),
            id,
        }
    }
}

/// Workspace state
pub struct WorkspaceState {
    /// Project root directory
    pub root: Option<PathBuf>,
    /// Open editors
    pub editors: Vec<EditorState>,
    /// Active editor index
    pub active_editor: Option<usize>,
    /// Sidebar visibility
    pub sidebar_visible: bool,
    /// Panel visibility
    pub panel_visible: bool,
}

impl WorkspaceState {
    pub fn new() -> Self {
        Self {
            root: None,
            editors: Vec::new(),
            active_editor: None,
            sidebar_visible: true,
            panel_visible: false,
        }
    }

    pub fn add_editor(&mut self, editor: EditorState) -> usize {
        self.editors.push(editor);
        let index = self.editors.len() - 1;
        self.active_editor = Some(index);
        index
    }

    pub fn active_editor_mut(&mut self) -> Option<&mut EditorState> {
        self.active_editor.and_then(|idx| self.editors.get_mut(idx))
    }
}

impl Default for WorkspaceState {
    fn default() -> Self {
        Self::new()
    }
}

/// Editor state for a single buffer
pub struct EditorState {
    /// Buffer ID
    pub buffer_id: BufferId,
    /// Scroll position
    pub scroll_x: f32,
    pub scroll_y: f32,
    /// Cursor positions (multi-cursor support)
    pub cursors: Vec<editor_core::selection::Cursor>,
    /// Selections
    pub selections: editor_core::selection::SelectionSet,
}

impl EditorState {
    pub fn new(buffer_id: BufferId) -> Self {
        Self {
            buffer_id,
            scroll_x: 0.0,
            scroll_y: 0.0,
            cursors: vec![
                editor_core::selection::Cursor::new(editor_core::selection::Position::zero())
            ],
            selections: editor_core::selection::SelectionSet::default(),
        }
    }
}

/// Preview state
pub struct PreviewState {
    /// Rendered document path
    pub document: Option<PathBuf>,
    /// Zoom level
    pub zoom: f32,
    /// Scroll position
    pub scroll_x: f32,
    pub scroll_y: f32,
    /// Whether compilation is in progress
    pub compiling: bool,
    /// Last compilation error
    pub last_error: Option<String>,
}

impl PreviewState {
    pub fn new() -> Self {
        Self {
            document: None,
            zoom: 1.0,
            scroll_x: 0.0,
            scroll_y: 0.0,
            compiling: false,
            last_error: None,
        }
    }
}

impl Default for PreviewState {
    fn default() -> Self {
        Self::new()
    }
}

/// Global settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    /// Editor settings
    pub editor: EditorSettings,
    /// Preview settings
    pub preview: PreviewSettings,
    /// LSP settings
    pub lsp: LspSettings,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            editor: EditorSettings::default(),
            preview: PreviewSettings::default(),
            lsp: LspSettings::default(),
        }
    }
}

/// Editor settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorSettings {
    pub font_family: String,
    pub font_size: f32,
    pub line_height: f32,
    pub tab_size: usize,
    pub insert_spaces: bool,
    pub word_wrap: bool,
    pub show_line_numbers: bool,
}

impl Default for EditorSettings {
    fn default() -> Self {
        Self {
            font_family: "monospace".to_string(),
            font_size: 14.0,
            line_height: 1.5,
            tab_size: 2,
            insert_spaces: true,
            word_wrap: false,
            show_line_numbers: true,
        }
    }
}

/// Preview settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreviewSettings {
    pub default_zoom: String,
    pub sync_scroll: bool,
    pub render_quality: String,
}

impl Default for PreviewSettings {
    fn default() -> Self {
        Self {
            default_zoom: "fit_width".to_string(),
            sync_scroll: true,
            render_quality: "normal".to_string(),
        }
    }
}

/// LSP settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LspSettings {
    pub enable: bool,
    pub diagnostic_delay: u64,
    pub completion_trigger_delay: u64,
}

impl Default for LspSettings {
    fn default() -> Self {
        Self {
            enable: true,
            diagnostic_delay: 500,
            completion_trigger_delay: 100,
        }
    }
}
