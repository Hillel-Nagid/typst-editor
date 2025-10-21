//! Tests for edit operations and undo/redo system

use editor_core::{ EditOperation, OperationType, UndoHistory, Position };

#[test]
fn test_insert_operation() {
    let op = EditOperation::insert(Position::new(0, 0), "Hello".to_string(), Position::new(0, 5));

    assert_eq!(op.op_type, OperationType::Insert);
    assert_eq!(op.start, Position::new(0, 0));
    assert_eq!(op.inserted_text, Some("Hello".to_string()));
}

#[test]
fn test_delete_operation() {
    let op = EditOperation::delete(
        Position::new(0, 0),
        Position::new(0, 5),
        "Hello".to_string(),
        Position::new(0, 0)
    );

    assert_eq!(op.op_type, OperationType::Delete);
    assert_eq!(op.deleted_text, Some("Hello".to_string()));
}

#[test]
fn test_replace_operation() {
    let op = EditOperation::replace(
        Position::new(0, 0),
        Position::new(0, 5),
        "Hello".to_string(),
        "Hi".to_string(),
        Position::new(0, 2)
    );

    assert_eq!(op.op_type, OperationType::Replace);
    assert_eq!(op.deleted_text, Some("Hello".to_string()));
    assert_eq!(op.inserted_text, Some("Hi".to_string()));
}

#[test]
fn test_operation_merging() {
    let op1 = EditOperation::insert(Position::new(0, 0), "H".to_string(), Position::new(0, 1));

    let op2 = EditOperation::insert(Position::new(0, 1), "e".to_string(), Position::new(0, 2));

    assert!(op1.can_merge_with(&op2));
}

#[test]
fn test_operation_no_merge_different_types() {
    let op1 = EditOperation::insert(Position::new(0, 0), "H".to_string(), Position::new(0, 1));

    let op2 = EditOperation::delete(
        Position::new(0, 0),
        Position::new(0, 1),
        "H".to_string(),
        Position::new(0, 0)
    );

    assert!(!op1.can_merge_with(&op2));
}

#[test]
fn test_undo_history_basic() {
    let mut history = UndoHistory::new();

    let op = EditOperation::insert(Position::new(0, 0), "Hello".to_string(), Position::new(0, 5));

    history.record_operation(op);
    assert!(history.can_undo());
    assert!(!history.can_redo());
}

#[test]
fn test_undo_history_undo_redo() {
    let mut history = UndoHistory::new();

    let op = EditOperation::insert(Position::new(0, 0), "Hello".to_string(), Position::new(0, 5));

    history.record_operation(op);

    // Undo
    let group = history.undo();
    assert!(group.is_some());
    assert!(!history.can_undo());
    assert!(history.can_redo());

    // Redo
    let group = history.redo();
    assert!(group.is_some());
    assert!(history.can_undo());
    assert!(!history.can_redo());
}

#[test]
fn test_undo_history_boundary() {
    let mut history = UndoHistory::new();

    let op1 = EditOperation::insert(Position::new(0, 0), "a".to_string(), Position::new(0, 1));
    history.record_operation(op1);

    let op2 = EditOperation::insert(Position::new(0, 1), "b".to_string(), Position::new(0, 2));
    history.record_operation(op2);

    // Create boundary
    history.create_boundary();

    let op3 = EditOperation::insert(Position::new(0, 2), "c".to_string(), Position::new(0, 3));
    history.record_operation(op3);

    // First undo should only undo op3
    let group = history.undo().unwrap();
    assert_eq!(group.operations.len(), 1);

    // Second undo should undo op1 and op2
    let group = history.undo().unwrap();
    assert!(group.operations.len() > 0);
}

#[test]
fn test_undo_history_clear() {
    let mut history = UndoHistory::new();

    let op = EditOperation::insert(Position::new(0, 0), "Hello".to_string(), Position::new(0, 5));
    history.record_operation(op);

    history.clear();
    assert!(!history.can_undo());
    assert!(!history.can_redo());
}

#[test]
fn test_undo_history_limits() {
    let mut history = UndoHistory::with_limits(3, 1000);

    // Add more operations than the limit
    for i in 0..5 {
        let op = EditOperation::insert(
            Position::new(0, i),
            "x".to_string(),
            Position::new(0, i + 1)
        );
        history.record_operation(op);
        history.create_boundary(); // Force separate groups
    }

    // Should still be able to undo (some operations dropped)
    assert!(history.can_undo());
}

#[test]
fn test_undo_clears_redo() {
    let mut history = UndoHistory::new();

    let op1 = EditOperation::insert(Position::new(0, 0), "Hello".to_string(), Position::new(0, 5));
    history.record_operation(op1);

    // Undo
    history.undo();
    assert!(history.can_redo());

    // Record new operation - should clear redo
    let op2 = EditOperation::insert(Position::new(0, 0), "Hi".to_string(), Position::new(0, 2));
    history.record_operation(op2);

    assert!(!history.can_redo());
}
