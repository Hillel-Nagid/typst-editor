use crate::buffer::command::BufferSnapshot;
use crate::buffer::command_types::SnapshotCommand;
use crate::buffer::{History, RopeBuffer, TextBuffer};
use crate::document::LineEnding;
use crate::operations::auto_pair::matching_pair;
use crate::operations::word_boundary::{find_next_word_start, find_prev_word_start};
use crate::selection::{Cursor, MultiCursor};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MoveDirection {
    Left,
    Right,
    Up,
    Down,
    WordLeft,
    WordRight,
    LineStart,
    LineEnd,
    DocumentStart,
    DocumentEnd,
}

#[derive(Clone)]
pub struct EditorSession {
    pub buffer: RopeBuffer,
    pub cursors: MultiCursor,
    pub history: History,
}

impl EditorSession {
    pub fn new(text: &str, line_ending: LineEnding) -> Self {
        let mut buffer = RopeBuffer::new(text);
        buffer.set_line_ending(line_ending);
        Self {
            buffer,
            cursors: MultiCursor::default(),
            history: History::default(),
        }
    }

    pub fn text(&self) -> String {
        self.buffer.text()
    }

    pub fn set_text(&mut self, text: String) {
        self.with_edit("set_text", None, |session| {
            let len = session.buffer.len();
            if len > 0 {
                session.buffer.delete(0..len);
            }
            session.buffer.insert(0, &text);
            session.cursors.set_single(session.buffer.len());
        });
    }

    pub fn is_dirty(&self) -> bool {
        self.buffer.is_dirty()
    }

    pub fn mark_saved(&mut self) {
        self.buffer.set_dirty(false);
        self.history.mark_saved();
    }

    pub fn undo(&mut self) -> bool {
        if let Some(command) = self.history.undo() {
            self.apply_snapshot(command.before.clone());
            return true;
        }
        false
    }

    pub fn redo(&mut self) -> bool {
        if let Some(command) = self.history.redo() {
            self.apply_snapshot(command.after.clone());
            return true;
        }
        false
    }

    pub fn insert_text(&mut self, text: &str) {
        let inserted = text.to_string();
        let merge_key = if inserted.chars().count() == 1 {
            Some("typing".to_string())
        } else {
            None
        };
        self.with_edit("insert_text", merge_key, |session| {
            let mut next = Vec::new();
            for cursor in session.cursors.ordered_desc() {
                let mut pos = cursor.position();
                if cursor.has_selection() {
                    let range = cursor.range();
                    session.buffer.delete(range.clone());
                    pos = range.start;
                }
                session.buffer.insert(pos, &inserted);
                let new_pos = pos + inserted.chars().count();
                next.push(Cursor::new(new_pos));
            }
            session.cursors.set_cursors(next);
        });
    }

    pub fn insert_char_with_pairing(&mut self, ch: char) {
        if let Some(pair) = matching_pair(ch) {
            let text = if ch == '"' || ch == '\'' || ch == '`' {
                format!("{ch}{pair}")
            } else {
                ch.to_string()
            };
            self.insert_text(&text);
            self.move_cursor(MoveDirection::Left, false);
        } else {
            self.insert_text(&ch.to_string());
        }
    }

    pub fn delete_backward(&mut self) {
        self.with_edit("backspace", None, |session| {
            let mut next = Vec::new();
            for cursor in session.cursors.ordered_desc() {
                if cursor.has_selection() {
                    let range = cursor.range();
                    session.buffer.delete(range.clone());
                    next.push(Cursor::new(range.start));
                } else {
                    let pos = cursor.position();
                    if pos > 0 {
                        let prev = session.buffer.prev_grapheme(pos);
                        session.buffer.delete(prev..pos);
                        next.push(Cursor::new(prev));
                    } else {
                        next.push(cursor);
                    }
                }
            }
            session.cursors.set_cursors(next);
        });
    }

    pub fn delete_forward(&mut self) {
        self.with_edit("delete", None, |session| {
            let mut next = Vec::new();
            for cursor in session.cursors.ordered_desc() {
                if cursor.has_selection() {
                    let range = cursor.range();
                    session.buffer.delete(range.clone());
                    next.push(Cursor::new(range.start));
                } else {
                    let pos = cursor.position();
                    let next_pos = session.buffer.next_grapheme(pos);
                    if next_pos > pos {
                        session.buffer.delete(pos..next_pos);
                    }
                    next.push(Cursor::new(pos));
                }
            }
            session.cursors.set_cursors(next);
        });
    }

    pub fn insert_newline(&mut self) {
        let line_ending = self.buffer.line_ending().as_str().to_string();
        self.insert_text(&line_ending);
    }

    pub fn insert_tab(&mut self, tab_size: usize, insert_spaces: bool) {
        if insert_spaces {
            self.insert_text(&" ".repeat(tab_size.max(1)));
        } else {
            self.insert_text("\t");
        }
    }

    pub fn indent_selected_lines(&mut self, tab_size: usize, insert_spaces: bool) {
        let indent = if insert_spaces {
            " ".repeat(tab_size.max(1))
        } else {
            "\t".to_string()
        };
        self.with_edit("indent", None, |session| {
            let mut lines = Vec::new();
            for cursor in session.cursors.cursors() {
                let range = cursor.range();
                let (start_line, _) = session.buffer.offset_to_line_col(range.start);
                let (end_line, _) = session.buffer.offset_to_line_col(range.end);
                for line in start_line..=end_line {
                    lines.push(line);
                }
            }
            lines.sort_unstable();
            lines.dedup();

            for line in lines.into_iter().rev() {
                if let Some(line_range) = session.buffer.line_range(line) {
                    session.buffer.insert(line_range.start, &indent);
                }
            }
        });
    }

    pub fn outdent_selected_lines(&mut self, tab_size: usize) {
        self.with_edit("outdent", None, |session| {
            let mut lines = Vec::new();
            for cursor in session.cursors.cursors() {
                let range = cursor.range();
                let (start_line, _) = session.buffer.offset_to_line_col(range.start);
                let (end_line, _) = session.buffer.offset_to_line_col(range.end);
                for line in start_line..=end_line {
                    lines.push(line);
                }
            }
            lines.sort_unstable();
            lines.dedup();

            for line in lines.into_iter().rev() {
                if let Some(range) = session.buffer.line_range(line) {
                    let text = session.buffer.text_range(range.clone());
                    if text.starts_with('\t') {
                        session.buffer.delete(range.start..range.start + 1);
                    } else {
                        let mut spaces = 0;
                        for ch in text.chars().take(tab_size.max(1)) {
                            if ch == ' ' {
                                spaces += 1;
                            } else {
                                break;
                            }
                        }
                        if spaces > 0 {
                            session.buffer.delete(range.start..range.start + spaces);
                        }
                    }
                }
            }
        });
    }

    pub fn toggle_line_comment(&mut self) {
        self.with_edit("toggle_line_comment", None, |session| {
            let mut edits = Vec::new();
            for cursor in session.cursors.cursors() {
                let (line, _) = session.buffer.offset_to_line_col(cursor.position());
                edits.push(line);
            }
            edits.sort_unstable();
            edits.dedup();

            for line in edits.into_iter().rev() {
                if let Some(range) = session.buffer.line_range(line) {
                    let line_text = session.buffer.text_range(range.clone());
                    let trimmed = line_text.trim_start();
                    let indent_len = line_text.len() - trimmed.len();
                    let indent_chars = line_text[..indent_len].chars().count();
                    let comment_pos = range.start + indent_chars;
                    if trimmed.starts_with("//") {
                        session.buffer.delete(comment_pos..comment_pos + 2);
                    } else {
                        session.buffer.insert(comment_pos, "//");
                    }
                }
            }
        });
    }

    pub fn duplicate_current_line_down(&mut self) {
        self.with_edit("duplicate_line_down", None, |session| {
            let pos = session.cursors.primary_cursor().position();
            let (line, _) = session.buffer.offset_to_line_col(pos);
            if let Some(range) = session.buffer.line_range(line) {
                let line_text = session.buffer.text_range(range.clone());
                session.buffer.insert(range.end, &line_text);
            }
        });
    }

    pub fn join_line_with_next(&mut self) {
        self.with_edit("join_lines", None, |session| {
            let pos = session.cursors.primary_cursor().position();
            let (line, _) = session.buffer.offset_to_line_col(pos);
            if line + 1 >= session.buffer.line_count() {
                return;
            }
            if let Some(current) = session.buffer.line_range(line) {
                let join_pos = current.end.saturating_sub(1);
                if join_pos < session.buffer.len() {
                    let next_text = session
                        .buffer
                        .line(line + 1)
                        .unwrap_or_default()
                        .trim_start()
                        .to_string();
                    let next_range = session.buffer.line_range(line + 1).unwrap_or(current.clone());
                    session.buffer.delete(join_pos..next_range.end.min(session.buffer.len()));
                    session.buffer.insert(join_pos, &format!(" {next_text}"));
                }
            }
        });
    }

    pub fn transform_uppercase(&mut self) {
        self.transform_selections(|text| text.to_uppercase(), "uppercase");
    }

    pub fn transform_lowercase(&mut self) {
        self.transform_selections(|text| text.to_lowercase(), "lowercase");
    }

    pub fn move_cursor(&mut self, direction: MoveDirection, extend_selection: bool) {
        let text = self.buffer.text();
        let mut next = Vec::new();
        let cursors = self.cursors.cursors().to_vec();
        for mut cursor in cursors {
            let pos = cursor.position();
            let target = match direction {
                MoveDirection::Left => self.buffer.prev_grapheme(pos),
                MoveDirection::Right => self.buffer.next_grapheme(pos),
                MoveDirection::WordLeft => find_prev_word_start(&text, pos),
                MoveDirection::WordRight => find_next_word_start(&text, pos),
                MoveDirection::LineStart => {
                    let (line, _) = self.buffer.offset_to_line_col(pos);
                    self.buffer.line_col_to_offset(line, 0)
                }
                MoveDirection::LineEnd => {
                    let (line, _) = self.buffer.offset_to_line_col(pos);
                    if let Some(range) = self.buffer.line_range(line) {
                        range.end
                    } else {
                        pos
                    }
                }
                MoveDirection::DocumentStart => 0,
                MoveDirection::DocumentEnd => self.buffer.len(),
                MoveDirection::Up => self.vertical_move(pos, -1),
                MoveDirection::Down => self.vertical_move(pos, 1),
            };

            if extend_selection {
                cursor.head = target;
            } else {
                cursor.collapse_to(target);
            }
            next.push(cursor);
        }
        self.cursors.set_cursors(next);
    }

    fn vertical_move(&self, offset: usize, delta: isize) -> usize {
        let (line, col) = self.buffer.offset_to_line_col(offset);
        let target_line = if delta < 0 {
            line.saturating_sub(delta.unsigned_abs())
        } else {
            (line + delta as usize).min(self.buffer.line_count().saturating_sub(1))
        };
        self.buffer.line_col_to_offset(target_line, col)
    }

    fn transform_selections<F>(&mut self, transform: F, key: &str)
    where
        F: Fn(String) -> String,
    {
        self.with_edit(key, None, |session| {
            let mut next = Vec::new();
            for cursor in session.cursors.ordered_desc() {
                if cursor.has_selection() {
                    let range = cursor.range();
                    let original = session.buffer.text_range(range.clone());
                    let replacement = transform(original);
                    session.buffer.replace(range.clone(), &replacement);
                    let end = range.start + replacement.chars().count();
                    next.push(Cursor::with_selection(range.start, end));
                } else {
                    next.push(cursor);
                }
            }
            session.cursors.set_cursors(next);
        });
    }

    fn with_edit<F>(&mut self, _label: &str, merge_key: Option<String>, f: F)
    where
        F: FnOnce(&mut Self),
    {
        let before = self.snapshot();
        f(self);
        let after = self.snapshot();

        if before.text != after.text || before.cursors != after.cursors || before.dirty != after.dirty {
            let command = SnapshotCommand::new(before, after, merge_key, History::now());
            self.history.record(command);
        }
    }

    fn snapshot(&self) -> BufferSnapshot {
        BufferSnapshot {
            text: self.buffer.text(),
            cursors: self.cursors.clone(),
            dirty: self.buffer.is_dirty(),
        }
    }

    fn apply_snapshot(&mut self, snapshot: BufferSnapshot) {
        self.buffer = RopeBuffer::new(&snapshot.text);
        self.buffer.set_dirty(snapshot.dirty);
        self.cursors = snapshot.cursors;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn undo_redo_restores_content() {
        let mut session = EditorSession::new("", LineEnding::Lf);
        session.insert_text("abc");
        assert_eq!(session.text(), "abc");
        assert!(session.undo());
        assert_eq!(session.text(), "");
        assert!(session.redo());
        assert_eq!(session.text(), "abc");
    }

    #[test]
    fn records_multi_cursor_insertion_in_descending_order() {
        let mut session = EditorSession::new("123456", LineEnding::Lf);
        session
            .cursors
            .set_cursors(vec![Cursor::new(1), Cursor::new(5), Cursor::new(3)]);
        session.insert_text("x");
        assert_eq!(session.text(), "1x23x45x6");
    }
}
