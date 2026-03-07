use crate::editor_session::EditorSession;

pub fn backspace(session: &mut EditorSession) {
    session.delete_backward();
}

pub fn delete_forward(session: &mut EditorSession) {
    session.delete_forward();
}
