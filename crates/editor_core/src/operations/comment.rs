use crate::editor_session::EditorSession;

pub fn toggle_line_comment(session: &mut EditorSession) {
    session.toggle_line_comment();
}
