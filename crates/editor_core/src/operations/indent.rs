use crate::editor_session::EditorSession;

pub fn insert_tab(session: &mut EditorSession, tab_size: usize, insert_spaces: bool) {
    session.insert_tab(tab_size, insert_spaces);
}

pub fn indent_selection(session: &mut EditorSession, tab_size: usize, insert_spaces: bool) {
    session.indent_selected_lines(tab_size, insert_spaces);
}

pub fn outdent_selection(session: &mut EditorSession, tab_size: usize) {
    session.outdent_selected_lines(tab_size);
}
