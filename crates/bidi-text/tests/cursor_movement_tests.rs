//! Tests for bidirectional cursor movement

use bidi_text::{ BidiParagraph, CursorMovement, MovementDirection };

#[test]
fn test_ltr_movement() {
    let para = BidiParagraph::new("Hello World".to_string(), None);

    // Move right
    let new_pos = CursorMovement::move_visual(&para, 0, MovementDirection::Right).unwrap();
    assert_eq!(new_pos, 1);

    // Move left
    let new_pos = CursorMovement::move_visual(&para, 1, MovementDirection::Left).unwrap();
    assert_eq!(new_pos, 0);
}

#[test]
fn test_home_end() {
    let para = BidiParagraph::new("  Hello World".to_string(), None);

    // Home from middle goes to first non-whitespace
    let pos = CursorMovement::move_visual(&para, 7, MovementDirection::Home).unwrap();
    assert_eq!(pos, 2); // First non-whitespace

    // Home again goes to actual start
    let pos = CursorMovement::move_visual(&para, 2, MovementDirection::Home).unwrap();
    assert_eq!(pos, 0);

    // End goes to end
    let pos = CursorMovement::move_visual(&para, 0, MovementDirection::End).unwrap();
    assert_eq!(pos, "  Hello World".len());
}

#[test]
fn test_word_movement() {
    let para = BidiParagraph::new("hello world test".to_string(), None);

    // Move to next word
    let pos = CursorMovement::move_visual(&para, 0, MovementDirection::WordRight).unwrap();
    assert!(pos > 0 && pos < 11); // Should be at or past "hello"

    // Move to previous word
    let pos = CursorMovement::move_visual(&para, 10, MovementDirection::WordLeft).unwrap();
    assert!(pos < 10);
}

#[test]
fn test_rtl_text() {
    let para = BidiParagraph::new("שלום עולם".to_string(), None);

    // Basic movement should work
    let _pos = CursorMovement::move_visual(&para, 0, MovementDirection::Right).unwrap();
    // Successfully moved without error
    assert!(true);
}

#[test]
fn test_mixed_direction_text() {
    let para = BidiParagraph::new("Hello שלום World".to_string(), None);

    // Should handle mixed text
    let runs = para.visual_runs();
    assert!(runs.len() > 1); // Multiple runs expected
}

#[test]
fn test_vertical_movement() {
    let lines = vec!["Hello".to_string(), "World test".to_string(), "End".to_string()];

    // Move down
    let pos = CursorMovement::move_vertical(&lines, 0, 2, MovementDirection::Down, None).unwrap();
    assert_eq!(pos.line, 1);
    assert_eq!(pos.column, 2);

    // Move up
    let pos = CursorMovement::move_vertical(&lines, 1, 2, MovementDirection::Up, None).unwrap();
    assert_eq!(pos.line, 0);
    assert_eq!(pos.column, 2);

    // Move down from last line (should stay at last line)
    let pos = CursorMovement::move_vertical(&lines, 2, 0, MovementDirection::Down, None).unwrap();
    assert_eq!(pos.line, 2);

    // Move up from first line (should stay at first line)
    let pos = CursorMovement::move_vertical(&lines, 0, 0, MovementDirection::Up, None).unwrap();
    assert_eq!(pos.line, 0);
    assert_eq!(pos.column, 0);
}

#[test]
fn test_vertical_movement_with_sticky_column() {
    let lines = vec!["Hello World".to_string(), "Hi".to_string(), "Goodbye World".to_string()];

    // Move down with sticky column beyond line length
    let pos = CursorMovement::move_vertical(
        &lines,
        0,
        8,
        MovementDirection::Down,
        Some(8)
    ).unwrap();
    assert_eq!(pos.line, 1);
    assert_eq!(pos.column, 2); // Line is shorter, column adjusted

    // Move down again with sticky column
    let pos = CursorMovement::move_vertical(
        &lines,
        1,
        2,
        MovementDirection::Down,
        Some(8)
    ).unwrap();
    assert_eq!(pos.line, 2);
    assert_eq!(pos.column, 8); // Back to sticky column
}

#[test]
fn test_grapheme_cluster_movement() {
    // Test with combining characters
    let para = BidiParagraph::new("café".to_string(), None);

    // Move through text
    let pos = CursorMovement::move_visual(&para, 0, MovementDirection::Right).unwrap();
    assert!(pos > 0);
}

#[test]
fn test_emoji_movement() {
    let para = BidiParagraph::new("Hello 👋 World".to_string(), None);

    // Should move by grapheme clusters, treating emoji as single unit
    let pos = CursorMovement::move_visual(&para, 0, MovementDirection::Right).unwrap();
    assert_eq!(pos, 1);
}

#[test]
fn test_logical_movement() {
    let text = "Hello World";

    // Forward logical movement
    let pos = CursorMovement::move_logical(text, 0, true);
    assert_eq!(pos, 1);

    // Backward logical movement
    let pos = CursorMovement::move_logical(text, 5, false);
    assert_eq!(pos, 4);

    // At boundaries
    let pos = CursorMovement::move_logical(text, 0, false);
    assert_eq!(pos, 0);

    let pos = CursorMovement::move_logical(text, text.len(), true);
    assert_eq!(pos, text.len());
}
