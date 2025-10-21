//! Comprehensive tests for the text buffer implementation

use editor_core::{ Buffer, BufferId, Position, LineEnding };

#[test]
fn test_buffer_creation_and_basic_operations() {
    let mut buffer = Buffer::new(BufferId::new(1));
    assert!(buffer.is_empty());
    assert!(!buffer.is_dirty());

    // Test insertion
    buffer.insert(Position::new(0, 0), "Hello").unwrap();
    assert_eq!(buffer.text(), "Hello");
    assert!(buffer.is_dirty());

    // Test multiple lines
    buffer.insert(Position::new(0, 5), "\nWorld").unwrap();
    assert_eq!(buffer.len_lines(), 2);
}

#[test]
fn test_undo_redo() {
    let mut buffer = Buffer::from_text(BufferId::new(1), "Hello");

    // Insert text
    buffer.insert(Position::new(0, 5), " World").unwrap();
    assert_eq!(buffer.text(), "Hello World");
    assert!(buffer.can_undo());

    // Undo
    let pos = buffer.undo().unwrap();
    assert_eq!(buffer.text(), "Hello");
    assert_eq!(pos, Position::new(0, 5));
    assert!(buffer.can_redo());

    // Redo
    buffer.redo().unwrap();
    assert_eq!(buffer.text(), "Hello World");
}

#[test]
fn test_undo_delete_operation() {
    let mut buffer = Buffer::from_text(BufferId::new(1), "Hello World");

    // Delete text
    buffer.delete(Position::new(0, 5), Position::new(0, 11)).unwrap();
    assert_eq!(buffer.text(), "Hello");

    // Undo delete
    buffer.undo().unwrap();
    assert_eq!(buffer.text(), "Hello World");
}

#[test]
fn test_undo_replace_operation() {
    let mut buffer = Buffer::from_text(BufferId::new(1), "Hello World");

    // Replace text
    buffer.replace(Position::new(0, 6), Position::new(0, 11), "Rust").unwrap();
    assert_eq!(buffer.text(), "Hello Rust");

    // Undo replace
    buffer.undo().unwrap();
    assert_eq!(buffer.text(), "Hello World");

    // Redo replace
    buffer.redo().unwrap();
    assert_eq!(buffer.text(), "Hello Rust");
}

#[test]
fn test_backspace() {
    let mut buffer = Buffer::from_text(BufferId::new(1), "Hello");

    // Backspace in middle of text
    let pos = buffer.backspace(Position::new(0, 3)).unwrap();
    assert_eq!(buffer.text(), "Helo");
    assert_eq!(pos, Position::new(0, 2));

    // Backspace at start (should not change)
    let pos = buffer.backspace(Position::new(0, 0)).unwrap();
    assert_eq!(pos, Position::new(0, 0));
    assert_eq!(buffer.text(), "Helo");
}

#[test]
fn test_backspace_at_line_start() {
    let mut buffer = Buffer::from_text(BufferId::new(1), "Hello\nWorld");

    // Backspace at start of second line should join lines
    let pos = buffer.backspace(Position::new(1, 0)).unwrap();
    assert_eq!(buffer.text(), "HelloWorld");
    assert_eq!(pos.line, 0);
}

#[test]
fn test_delete_forward() {
    let mut buffer = Buffer::from_text(BufferId::new(1), "Hello");

    // Delete forward
    let pos = buffer.delete_forward(Position::new(0, 2)).unwrap();
    assert_eq!(buffer.text(), "Helo");
    assert_eq!(pos, Position::new(0, 2));
}

#[test]
fn test_delete_forward_at_line_end() {
    let mut buffer = Buffer::from_text(BufferId::new(1), "Hello\nWorld");

    // Delete forward at end of first line should join lines
    let pos = buffer.delete_forward(Position::new(0, 5)).unwrap();
    assert_eq!(buffer.text(), "HelloWorld");
    assert_eq!(pos, Position::new(0, 5));
}

#[test]
fn test_word_boundaries() {
    let buffer = Buffer::from_text(BufferId::new(1), "hello world test");

    // Test next word boundary
    let next = buffer.next_word_boundary(Position::new(0, 0)).unwrap();
    assert!(next.column > 0);

    let next = buffer.next_word_boundary(Position::new(0, 5)).unwrap();
    assert!(next.column > 5);

    // Test previous word boundary
    let prev = buffer.prev_word_boundary(Position::new(0, 10)).unwrap();
    assert!(prev.column < 10);
}

#[test]
fn test_unicode_grapheme_handling() {
    // Test with combining characters (e + combining acute accent = é)
    let mut buffer = Buffer::from_text(BufferId::new(1), "café");

    // The 'é' might be a single grapheme cluster
    let pos = buffer.backspace(Position::new(0, 4)).unwrap();
    assert_eq!(pos.column, 3);
}

#[test]
fn test_line_ending_detection() {
    assert_eq!(LineEnding::detect("Hello\nWorld"), LineEnding::Lf);
    assert_eq!(LineEnding::detect("Hello\r\nWorld"), LineEnding::Crlf);
    assert_eq!(LineEnding::detect("Hello\rWorld"), LineEnding::Cr);
}

#[test]
fn test_buffer_snapshot() {
    let mut buffer = Buffer::from_text(BufferId::new(1), "Hello");
    let snapshot = buffer.snapshot();

    // Modify buffer
    buffer.insert(Position::new(0, 5), " World").unwrap();

    // Snapshot should remain unchanged
    assert_eq!(snapshot.text(), "Hello");
    assert_eq!(buffer.text(), "Hello World");
}

#[test]
fn test_buffer_metrics() {
    let buffer = Buffer::from_text(BufferId::new(1), "Hello\nWorld\nTest");
    let metrics = buffer.metrics();

    assert_eq!(metrics.total_lines, 3);
    assert!(metrics.total_chars > 0);
    assert!(metrics.total_bytes > 0);
}

#[test]
fn test_position_conversion() {
    let buffer = Buffer::from_text(BufferId::new(1), "Hello\nWorld\n");

    // Test round-trip conversion
    let pos = Position::new(1, 2);
    let idx = buffer.position_to_char_idx(pos).unwrap();
    let converted_pos = buffer.char_idx_to_position(idx).unwrap();
    assert_eq!(pos, converted_pos);
}

#[test]
fn test_multiple_undo_redo() {
    let mut buffer = Buffer::from_text(BufferId::new(1), "");

    // Multiple operations with boundaries to prevent merging
    buffer.insert(Position::new(0, 0), "a").unwrap();
    buffer.create_undo_boundary();
    buffer.insert(Position::new(0, 1), "b").unwrap();
    buffer.create_undo_boundary();
    buffer.insert(Position::new(0, 2), "c").unwrap();

    // Undo all
    buffer.undo().unwrap();
    buffer.undo().unwrap();
    buffer.undo().unwrap();
    assert_eq!(buffer.text(), "");

    // Redo all
    buffer.redo().unwrap();
    buffer.redo().unwrap();
    buffer.redo().unwrap();
    assert_eq!(buffer.text(), "abc");
}

#[test]
fn test_undo_boundary() {
    let mut buffer = Buffer::from_text(BufferId::new(1), "");

    buffer.insert(Position::new(0, 0), "a").unwrap();
    buffer.insert(Position::new(0, 1), "b").unwrap();

    // Create boundary
    buffer.create_undo_boundary();

    buffer.insert(Position::new(0, 2), "c").unwrap();
    buffer.insert(Position::new(0, 3), "d").unwrap();

    // First undo should only undo "cd"
    buffer.undo().unwrap();
    assert_eq!(buffer.text(), "ab");

    // Second undo should undo "ab"
    buffer.undo().unwrap();
    assert_eq!(buffer.text(), "");
}

#[test]
fn test_read_only_buffer() {
    let mut buffer = Buffer::from_text(BufferId::new(1), "Hello");
    buffer.set_read_only(true);

    // Should fail to insert
    let result = buffer.insert(Position::new(0, 5), " World");
    assert!(result.is_err());

    // Should fail to delete
    let result = buffer.delete(Position::new(0, 0), Position::new(0, 5));
    assert!(result.is_err());
}

#[test]
fn test_large_buffer_operations() {
    let mut buffer = Buffer::new(BufferId::new(1));

    // Build a large text
    let mut text = String::new();
    for i in 0..100 {
        text.push_str(&format!("Line {}\n", i));
    }

    // Insert all at once
    buffer.insert(Position::new(0, 0), &text).unwrap();
    assert_eq!(buffer.len_lines(), 101); // 100 lines + 1 empty line at end

    // Test that we can read the buffer
    assert!(buffer.text().contains("Line 0"));
    assert!(buffer.text().contains("Line 99"));

    // Test metrics
    let metrics = buffer.metrics();
    assert!(metrics.total_lines >= 100);
}
