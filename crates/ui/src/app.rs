use crate::theme::Theme;
use crate::workspace::main_window_view;
use editor_core::{ApplicationState, Config, Document, WorkspaceState};
use iced::keyboard::{self, Key, Modifiers, key};
use iced::{Element, Subscription, Task, application};
use parking_lot::RwLock;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub enum Message {
    ToggleSidebar,
    TogglePreview,
    ToggleConsole,
    EditorKey(EditorKey),
    UndoEdit,
    RedoEdit,
    ToggleLineComment,
    KeyboardShortcut(Shortcut),
}

#[derive(Debug, Clone, Copy)]
pub enum Shortcut {
    Undo,
    Redo,
    ToggleComment,
}

#[derive(Debug, Clone)]
pub enum EditorKey {
    InsertText(String),
    Enter,
    Backspace,
    Delete,
    Tab { shift: bool },
    Left { shift: bool, by_word: bool },
    Right { shift: bool, by_word: bool },
    Up { shift: bool },
    Down { shift: bool },
    Home { shift: bool },
    End { shift: bool },
    DocStart { shift: bool },
    DocEnd { shift: bool },
}

pub struct TypstEditorApp {
    pub state: Arc<RwLock<ApplicationState>>,
    pub theme: Arc<RwLock<Theme>>,
}

impl TypstEditorApp {
    fn new() -> Self {
        let config = Config::load();
        let theme = if config.appearance.theme == "light" {
            Theme::light()
        } else {
            Theme::dark()
        };

        let mut app_state = ApplicationState::new(config);
        app_state.add_window(0, WorkspaceState::new(0));
        if let Some(workspace) = app_state.get_active_workspace() {
            workspace.write().open_document(Document::new(None));
        }

        Self {
            state: Arc::new(RwLock::new(app_state)),
            theme: Arc::new(RwLock::new(theme)),
        }
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::KeyboardShortcut(Shortcut::Undo) => return self.update(Message::UndoEdit),
            Message::KeyboardShortcut(Shortcut::Redo) => return self.update(Message::RedoEdit),
            Message::KeyboardShortcut(Shortcut::ToggleComment) => {
                return self.update(Message::ToggleLineComment);
            }
            _ => {}
        }

        if let Some(workspace) = self.state.read().get_active_workspace() {
            let mut workspace = workspace.write();
            match message {
                Message::ToggleSidebar => workspace.sidebar_visible = !workspace.sidebar_visible,
                Message::TogglePreview => workspace.preview_visible = !workspace.preview_visible,
                Message::ToggleConsole => workspace.console_visible = !workspace.console_visible,
                Message::EditorKey(key) => {
                    if let Some(editor) = workspace.get_active_editor() {
                        let mut editor = editor.write();
                        match key {
                            EditorKey::InsertText(text) => editor.insert_text(&text),
                            EditorKey::Enter => editor.insert_newline(),
                            EditorKey::Backspace => editor.backspace(),
                            EditorKey::Delete => editor.delete_forward(),
                            EditorKey::Tab { shift } => {
                                let cfg = workspace_state_config_tab_settings(&workspace);
                                if shift {
                                    editor.outdent_selection(cfg.0);
                                } else {
                                    editor.insert_tab(cfg.0, cfg.1);
                                }
                            }
                            EditorKey::Left { shift, by_word } => {
                                if by_word {
                                    editor.move_word_left(shift);
                                } else {
                                    editor.move_left(shift);
                                }
                            }
                            EditorKey::Right { shift, by_word } => {
                                if by_word {
                                    editor.move_word_right(shift);
                                } else {
                                    editor.move_right(shift);
                                }
                            }
                            EditorKey::Up { shift } => editor.move_up(shift),
                            EditorKey::Down { shift } => editor.move_down(shift),
                            EditorKey::Home { shift } => editor.move_line_start(shift),
                            EditorKey::End { shift } => editor.move_line_end(shift),
                            EditorKey::DocStart { shift } => editor.move_document_start(shift),
                            EditorKey::DocEnd { shift } => editor.move_document_end(shift),
                        }
                    }
                }
                Message::UndoEdit => {
                    if let Some(editor) = workspace.get_active_editor() {
                        editor.write().undo();
                    }
                }
                Message::RedoEdit => {
                    if let Some(editor) = workspace.get_active_editor() {
                        editor.write().redo();
                    }
                }
                Message::ToggleLineComment => {
                    if let Some(editor) = workspace.get_active_editor() {
                        editor.write().toggle_line_comment();
                    }
                }
                Message::KeyboardShortcut(_) => {}
            }
        }

        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        main_window_view(&self.state, &self.theme)
    }

    fn subscription(&self) -> Subscription<Message> {
        keyboard::on_key_press(|key: Key, modifiers: Modifiers| {
            if !modifiers.command() {
                return match key.as_ref() {
                    Key::Named(key::Named::Enter) => Some(Message::EditorKey(EditorKey::Enter)),
                    Key::Named(key::Named::Backspace) => {
                        Some(Message::EditorKey(EditorKey::Backspace))
                    }
                    Key::Named(key::Named::Delete) => Some(Message::EditorKey(EditorKey::Delete)),
                    Key::Named(key::Named::Tab) => Some(Message::EditorKey(EditorKey::Tab {
                        shift: modifiers.shift(),
                    })),
                    Key::Named(key::Named::ArrowLeft) => Some(Message::EditorKey(EditorKey::Left {
                        shift: modifiers.shift(),
                        by_word: modifiers.alt(),
                    })),
                    Key::Named(key::Named::ArrowRight) => {
                        Some(Message::EditorKey(EditorKey::Right {
                            shift: modifiers.shift(),
                            by_word: modifiers.alt(),
                        }))
                    }
                    Key::Named(key::Named::ArrowUp) => {
                        Some(Message::EditorKey(EditorKey::Up { shift: modifiers.shift() }))
                    }
                    Key::Named(key::Named::ArrowDown) => {
                        Some(Message::EditorKey(EditorKey::Down { shift: modifiers.shift() }))
                    }
                    Key::Named(key::Named::Home) => {
                        Some(Message::EditorKey(EditorKey::Home { shift: modifiers.shift() }))
                    }
                    Key::Named(key::Named::End) => {
                        Some(Message::EditorKey(EditorKey::End { shift: modifiers.shift() }))
                    }
                    Key::Character(text) => {
                        if modifiers.control() {
                            None
                        } else {
                            Some(Message::EditorKey(EditorKey::InsertText(normalize_insert_text(
                                text,
                                modifiers.shift(),
                            ))))
                        }
                    }
                    _ => None,
                };
            }

            match key.as_ref() {
                Key::Character(text) if text.eq_ignore_ascii_case("z") && modifiers.shift() => {
                    Some(Message::KeyboardShortcut(Shortcut::Redo))
                }
                Key::Character(text) if text.eq_ignore_ascii_case("z") => {
                    Some(Message::KeyboardShortcut(Shortcut::Undo))
                }
                Key::Character("/") => Some(Message::KeyboardShortcut(Shortcut::ToggleComment)),
                Key::Named(key::Named::ArrowLeft) => Some(Message::EditorKey(EditorKey::Left {
                    shift: modifiers.shift(),
                    by_word: true,
                })),
                Key::Named(key::Named::ArrowRight) => {
                    Some(Message::EditorKey(EditorKey::Right {
                        shift: modifiers.shift(),
                        by_word: true,
                    }))
                }
                Key::Named(key::Named::Home) => Some(Message::EditorKey(EditorKey::DocStart {
                    shift: modifiers.shift(),
                })),
                Key::Named(key::Named::End) => Some(Message::EditorKey(EditorKey::DocEnd {
                    shift: modifiers.shift(),
                })),
                Key::Named(key::Named::F5) => None,
                _ => None,
            }
        })
    }
}

pub fn run() -> iced::Result {
    application("Typst Studio", TypstEditorApp::update, TypstEditorApp::view)
        .subscription(TypstEditorApp::subscription)
        .run_with(|| (TypstEditorApp::new(), Task::none()))
}

fn workspace_state_config_tab_settings(workspace: &WorkspaceState) -> (usize, bool) {
    let _ = workspace;
    (4, true)
}

fn normalize_insert_text(text: &str, shift_pressed: bool) -> String {
    if !shift_pressed {
        return text.to_string();
    }

    if text.chars().count() != 1 {
        return text.to_string();
    }

    let ch = text.chars().next().unwrap_or_default();
    if ch.is_ascii_lowercase() {
        return ch.to_ascii_uppercase().to_string();
    }

    let shifted = match ch {
        '`' => '~',
        '1' => '!',
        '2' => '@',
        '3' => '#',
        '4' => '$',
        '5' => '%',
        '6' => '^',
        '7' => '&',
        '8' => '*',
        '9' => '(',
        '0' => ')',
        '-' => '_',
        '=' => '+',
        '[' => '{',
        ']' => '}',
        '\\' => '|',
        ';' => ':',
        '\'' => '"',
        ',' => '<',
        '.' => '>',
        '/' => '?',
        _ => ch,
    };
    shifted.to_string()
}

#[cfg(test)]
mod tests {
    use super::normalize_insert_text;

    #[test]
    fn normalize_insert_text_preserves_unshifted_input() {
        assert_eq!(normalize_insert_text("a", false), "a");
        assert_eq!(normalize_insert_text("1", false), "1");
        assert_eq!(normalize_insert_text("/", false), "/");
    }

    #[test]
    fn normalize_insert_text_applies_shift_to_letters_and_symbols() {
        assert_eq!(normalize_insert_text("a", true), "A");
        assert_eq!(normalize_insert_text("z", true), "Z");
        assert_eq!(normalize_insert_text("1", true), "!");
        assert_eq!(normalize_insert_text("2", true), "@");
        assert_eq!(normalize_insert_text("/", true), "?");
        assert_eq!(normalize_insert_text("=", true), "+");
        assert_eq!(normalize_insert_text("`", true), "~");
    }

    #[test]
    fn normalize_insert_text_leaves_multichar_text_unchanged() {
        assert_eq!(normalize_insert_text("ab", true), "ab");
        assert_eq!(normalize_insert_text("ß", true), "ß");
    }
}
