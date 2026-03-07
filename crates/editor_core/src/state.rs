use crate::config::Config;
use crate::bidi_text::{Direction, LineDirection, detect_line_directions, layout_line};
use crate::bidi_text::layout::hit_test::visual_to_logical as hit_test_visual_to_logical;
use crate::bidi_text::layout::selection_ranges::selection_ranges as compute_selection_ranges;
use crate::document::{Document, DocumentId};
use crate::editor_session::EditorSession;
use crate::editor_session::MoveDirection;
use crate::selection::MultiCursor;
use parking_lot::RwLock;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;

pub type WindowId = usize;
pub type WorkspaceId = usize;

#[derive(Clone)]
pub struct ApplicationState {
    pub windows: Vec<WindowId>,
    pub active_window: Option<WindowId>,
    pub workspaces: HashMap<WindowId, Arc<RwLock<WorkspaceState>>>,
    pub config: Arc<RwLock<Config>>,
    pub recent_files: Vec<PathBuf>,
}

impl ApplicationState {
    pub fn new(config: Config) -> Self {
        Self {
            windows: Vec::new(),
            active_window: None,
            workspaces: HashMap::new(),
            config: Arc::new(RwLock::new(config)),
            recent_files: Vec::new(),
        }
    }

    pub fn add_window(&mut self, window_id: WindowId, workspace: WorkspaceState) {
        self.windows.push(window_id);
        self.workspaces
            .insert(window_id, Arc::new(RwLock::new(workspace)));
        if self.active_window.is_none() {
            self.active_window = Some(window_id);
        }
    }

    pub fn get_active_workspace(&self) -> Option<Arc<RwLock<WorkspaceState>>> {
        self.active_window
            .and_then(|id| self.workspaces.get(&id))
            .cloned()
    }
}

#[derive(Clone)]
pub struct WorkspaceState {
    pub workspace_id: WorkspaceId,
    pub root: Option<PathBuf>,
    pub open_documents: HashMap<DocumentId, Arc<RwLock<EditorState>>>,
    pub active_document: Option<DocumentId>,
    pub sidebar_visible: bool,
    pub preview_visible: bool,
    pub console_visible: bool,
}

impl WorkspaceState {
    pub fn new(workspace_id: WorkspaceId) -> Self {
        Self {
            workspace_id,
            root: None,
            open_documents: HashMap::new(),
            active_document: None,
            sidebar_visible: true,
            preview_visible: true,
            console_visible: false,
        }
    }

    pub fn open_document(&mut self, document: Document) -> DocumentId {
        let id = document.id;
        let editor_state = EditorState::new(document);
        self.open_documents
            .insert(id, Arc::new(RwLock::new(editor_state)));
        self.active_document = Some(id);
        id
    }

    pub fn get_active_editor(&self) -> Option<Arc<RwLock<EditorState>>> {
        self.active_document
            .and_then(|id| self.open_documents.get(&id))
            .cloned()
    }

    pub fn close_document(&mut self, id: DocumentId) {
        self.open_documents.remove(&id);
        if self.active_document == Some(id) {
            self.active_document = self.open_documents.keys().next().copied();
        }
    }
}

#[derive(Clone)]
pub struct EditorState {
    pub document: Document,
    session: EditorSession,
    pub scroll_offset: f32,
}

impl EditorState {
    pub fn new(document: Document) -> Self {
        let line_ending = document.line_ending;
        Self {
            document,
            session: EditorSession::new("", line_ending),
            scroll_offset: 0.0,
        }
    }

    pub fn set_content(&mut self, content: String) {
        self.session.set_text(content);
        self.document.mark_dirty();
        self.document.increment_version();
    }

    pub fn content(&self) -> String {
        self.session.text()
    }

    pub fn cursors(&self) -> &MultiCursor {
        &self.session.cursors
    }

    pub fn insert_text(&mut self, text: &str) {
        self.session.insert_text(text);
        self.document.mark_dirty();
        self.document.increment_version();
    }

    pub fn insert_char(&mut self, ch: char) {
        self.session.insert_char_with_pairing(ch);
        self.document.mark_dirty();
        self.document.increment_version();
    }

    pub fn backspace(&mut self) {
        self.session.delete_backward();
        self.document.mark_dirty();
        self.document.increment_version();
    }

    pub fn delete_forward(&mut self) {
        self.session.delete_forward();
        self.document.mark_dirty();
        self.document.increment_version();
    }

    pub fn insert_newline(&mut self) {
        self.session.insert_newline();
        self.document.mark_dirty();
        self.document.increment_version();
    }

    pub fn insert_tab(&mut self, tab_size: usize, insert_spaces: bool) {
        self.session.insert_tab(tab_size, insert_spaces);
        self.document.mark_dirty();
        self.document.increment_version();
    }

    pub fn outdent_selection(&mut self, tab_size: usize) {
        self.session.outdent_selected_lines(tab_size);
        self.document.mark_dirty();
        self.document.increment_version();
    }

    pub fn toggle_line_comment(&mut self) {
        self.session.toggle_line_comment();
        self.document.mark_dirty();
        self.document.increment_version();
    }

    pub fn undo(&mut self) -> bool {
        let changed = self.session.undo();
        self.document.is_dirty = self.session.is_dirty();
        changed
    }

    pub fn redo(&mut self) -> bool {
        let changed = self.session.redo();
        self.document.is_dirty = self.session.is_dirty();
        changed
    }

    pub fn save_point(&mut self) {
        self.session.mark_saved();
        self.document.mark_clean();
    }

    pub fn line_count(&self) -> usize {
        self.content().lines().count().max(1)
    }

    pub fn lines(&self) -> Vec<String> {
        let text = self.content();
        if text.is_empty() {
            return vec![String::new()];
        }
        text.split('\n').map(|line| line.to_string()).collect()
    }

    pub fn visual_lines(&self) -> Vec<crate::bidi_text::VisualLine> {
        let lines = self.lines();
        let directions = detect_line_directions(&lines, Direction::Ltr);
        lines
            .iter()
            .zip(directions.iter().copied())
            .map(|(line, dir)| {
                let base_direction = match dir {
                    LineDirection::Rtl => Direction::Rtl,
                    LineDirection::Ltr | LineDirection::Math => Direction::Ltr,
                };
                layout_line(line, base_direction)
            })
            .collect()
    }

    pub fn line_directions(&self) -> Vec<LineDirection> {
        detect_line_directions(&self.lines(), Direction::Ltr)
    }

    pub fn hit_test_visual_x(&self, line_index: usize, visual_x: f32) -> Option<usize> {
        let lines = self.lines();
        let line = lines.get(line_index)?;
        let visual = layout_line(line, Direction::Ltr);
        let line_start = lines
            .iter()
            .take(line_index)
            .map(|l| l.chars().count() + 1)
            .sum::<usize>();
        Some(line_start + hit_test_visual_to_logical(&visual, visual_x))
    }

    pub fn visual_selection_ranges_for_line(
        &self,
        line_index: usize,
        selection_start: usize,
        selection_end: usize,
    ) -> Vec<(f32, f32)> {
        let lines = self.lines();
        let Some(line) = lines.get(line_index) else {
            return Vec::new();
        };
        let visual = layout_line(line, Direction::Ltr);
        let line_start = lines
            .iter()
            .take(line_index)
            .map(|l| l.chars().count() + 1)
            .sum::<usize>();
        let local_start = selection_start.saturating_sub(line_start).min(line.chars().count());
        let local_end = selection_end.saturating_sub(line_start).min(line.chars().count());
        compute_selection_ranges(&visual, local_start.min(local_end), local_start.max(local_end))
    }

    pub fn move_left(&mut self, extend_selection: bool) {
        self.session.move_cursor(MoveDirection::Left, extend_selection);
    }

    pub fn move_right(&mut self, extend_selection: bool) {
        self.session.move_cursor(MoveDirection::Right, extend_selection);
    }

    pub fn move_up(&mut self, extend_selection: bool) {
        self.session.move_cursor(MoveDirection::Up, extend_selection);
    }

    pub fn move_down(&mut self, extend_selection: bool) {
        self.session.move_cursor(MoveDirection::Down, extend_selection);
    }

    pub fn move_word_left(&mut self, extend_selection: bool) {
        self.session.move_cursor(MoveDirection::WordLeft, extend_selection);
    }

    pub fn move_word_right(&mut self, extend_selection: bool) {
        self.session.move_cursor(MoveDirection::WordRight, extend_selection);
    }

    pub fn move_line_start(&mut self, extend_selection: bool) {
        self.session.move_cursor(MoveDirection::LineStart, extend_selection);
    }

    pub fn move_line_end(&mut self, extend_selection: bool) {
        self.session.move_cursor(MoveDirection::LineEnd, extend_selection);
    }

    pub fn move_document_start(&mut self, extend_selection: bool) {
        self.session.move_cursor(MoveDirection::DocumentStart, extend_selection);
    }

    pub fn move_document_end(&mut self, extend_selection: bool) {
        self.session.move_cursor(MoveDirection::DocumentEnd, extend_selection);
    }
}

