use crate::selection::{Cursor, MultiCursor};

pub fn select_all(cursors: &mut MultiCursor, len: usize) {
    cursors.set_cursors(vec![Cursor::with_selection(0, len)]);
}

pub fn collapse_to_primary(cursors: &mut MultiCursor) {
    cursors.clear_secondary();
}
