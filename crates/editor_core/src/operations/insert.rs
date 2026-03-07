use crate::editor_session::EditorSession;

pub fn insert_text(session: &mut EditorSession, text: &str) {
    session.insert_text(text);
}

pub fn insert_char(session: &mut EditorSession, ch: char) {
    session.insert_char_with_pairing(ch);
}
