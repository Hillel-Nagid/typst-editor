use crate::editor_session::EditorSession;

pub fn duplicate_current_line_down(session: &mut EditorSession) {
    session.duplicate_current_line_down();
}

pub fn join_line_with_next(session: &mut EditorSession) {
    session.join_line_with_next();
}
