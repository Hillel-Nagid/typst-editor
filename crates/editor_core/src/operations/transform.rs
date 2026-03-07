use crate::editor_session::EditorSession;

pub fn uppercase(session: &mut EditorSession) {
    session.transform_uppercase();
}

pub fn lowercase(session: &mut EditorSession) {
    session.transform_lowercase();
}
