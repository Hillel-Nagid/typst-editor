//! Tests for selection and cursor management

use editor_core::{ Position, Selection, SelectionSet, Cursor, Affinity, Granularity };

#[test]
fn test_position_ordering() {
    let p1 = Position::new(1, 5);
    let p2 = Position::new(1, 10);
    let p3 = Position::new(2, 0);

    assert!(p1 < p2);
    assert!(p2 < p3);
    assert!(p1 < p3);
}

#[test]
fn test_collapsed_selection() {
    let sel = Selection::collapsed(Position::new(5, 10));
    assert!(sel.is_collapsed());

    let (start, end) = sel.range();
    assert_eq!(start, Position::new(5, 10));
    assert_eq!(end, Position::new(5, 10));
}

#[test]
fn test_forward_selection() {
    let sel = Selection::new(Position::new(1, 5), Position::new(1, 10));
    assert!(sel.is_forward());

    let (start, end) = sel.range();
    assert_eq!(start, Position::new(1, 5));
    assert_eq!(end, Position::new(1, 10));
}

#[test]
fn test_backward_selection() {
    let sel = Selection::new(Position::new(1, 10), Position::new(1, 5));
    assert!(!sel.is_forward());

    let (start, end) = sel.range();
    assert_eq!(start, Position::new(1, 5));
    assert_eq!(end, Position::new(1, 10));
}

#[test]
fn test_cursor_affinity() {
    let cursor = Cursor::with_affinity(Position::new(1, 5), Affinity::Upstream);
    assert_eq!(cursor.affinity, Affinity::Upstream);
    assert_eq!(cursor.position, Position::new(1, 5));
}

#[test]
fn test_selection_set_single() {
    let sel = Selection::collapsed(Position::new(1, 5));
    let set = SelectionSet::new(sel);

    assert_eq!(set.selections().len(), 1);
    assert_eq!(set.primary().cursor.position, Position::new(1, 5));
}

#[test]
fn test_selection_set_multiple() {
    let sel1 = Selection::collapsed(Position::new(1, 5));
    let mut set = SelectionSet::new(sel1);

    let sel2 = Selection::collapsed(Position::new(2, 10));
    set.add_selection(sel2);

    assert_eq!(set.selections().len(), 2);
}

#[test]
fn test_selection_set_clear_secondary() {
    let sel1 = Selection::collapsed(Position::new(1, 5));
    let mut set = SelectionSet::new(sel1);

    set.add_selection(Selection::collapsed(Position::new(2, 10)));
    set.add_selection(Selection::collapsed(Position::new(3, 15)));

    set.clear_secondary();
    assert_eq!(set.selections().len(), 1);
}

#[test]
fn test_selection_set_merge_overlapping() {
    let sel1 = Selection::new(Position::new(1, 0), Position::new(1, 10));
    let mut set = SelectionSet::new(sel1);

    // Add overlapping selection
    let sel2 = Selection::new(Position::new(1, 5), Position::new(1, 15));
    set.add_selection(sel2);

    set.merge_overlapping();

    // Should be merged into one selection
    assert_eq!(set.selections().len(), 1);
    let (start, end) = set.primary().range();
    assert_eq!(start, Position::new(1, 0));
    assert_eq!(end, Position::new(1, 15));
}

#[test]
fn test_selection_set_merge_non_overlapping() {
    let sel1 = Selection::new(Position::new(1, 0), Position::new(1, 5));
    let mut set = SelectionSet::new(sel1);

    // Add non-overlapping selection
    let sel2 = Selection::new(Position::new(2, 0), Position::new(2, 5));
    set.add_selection(sel2);

    set.merge_overlapping();

    // Should remain as two selections
    assert_eq!(set.selections().len(), 2);
}

#[test]
fn test_selection_granularity() {
    let mut sel = Selection::collapsed(Position::new(1, 5));
    assert_eq!(sel.granularity, Granularity::Character);

    sel.granularity = Granularity::Word;
    assert_eq!(sel.granularity, Granularity::Word);
}

#[test]
fn test_cursor_sticky_column() {
    let mut cursor = Cursor::new(Position::new(1, 5));
    assert_eq!(cursor.sticky_column, None);

    cursor.sticky_column = Some(10);
    assert_eq!(cursor.sticky_column, Some(10));
}

#[test]
fn test_multi_line_selection() {
    let sel = Selection::new(Position::new(1, 5), Position::new(3, 10));

    let (start, end) = sel.range();
    assert_eq!(start.line, 1);
    assert_eq!(end.line, 3);
}

#[test]
fn test_selection_set_default() {
    let set = SelectionSet::default();
    assert_eq!(set.selections().len(), 1);
    assert_eq!(set.primary().cursor.position, Position::zero());
}
