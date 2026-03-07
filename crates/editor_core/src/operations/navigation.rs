use crate::editor_session::{EditorSession, MoveDirection};
use crate::buffer::TextBuffer;

pub fn move_left(session: &mut EditorSession, extend_selection: bool) {
    session.move_cursor(MoveDirection::Left, extend_selection);
}

pub fn move_right(session: &mut EditorSession, extend_selection: bool) {
    session.move_cursor(MoveDirection::Right, extend_selection);
}

pub fn move_up(session: &mut EditorSession, extend_selection: bool) {
    session.move_cursor(MoveDirection::Up, extend_selection);
}

pub fn move_down(session: &mut EditorSession, extend_selection: bool) {
    session.move_cursor(MoveDirection::Down, extend_selection);
}

pub fn move_word_left(session: &mut EditorSession, extend_selection: bool) {
    session.move_cursor(MoveDirection::WordLeft, extend_selection);
}

pub fn move_word_right(session: &mut EditorSession, extend_selection: bool) {
    session.move_cursor(MoveDirection::WordRight, extend_selection);
}

pub fn move_line_start(session: &mut EditorSession, extend_selection: bool) {
    session.move_cursor(MoveDirection::LineStart, extend_selection);
}

pub fn move_line_end(session: &mut EditorSession, extend_selection: bool) {
    session.move_cursor(MoveDirection::LineEnd, extend_selection);
}

pub fn move_document_start(session: &mut EditorSession, extend_selection: bool) {
    session.move_cursor(MoveDirection::DocumentStart, extend_selection);
}

pub fn move_document_end(session: &mut EditorSession, extend_selection: bool) {
    session.move_cursor(MoveDirection::DocumentEnd, extend_selection);
}

pub fn go_to_line(session: &mut EditorSession, line: usize, col: usize) {
    let offset = session.buffer.line_col_to_offset(line, col);
    session.cursors.set_single(offset);
}
