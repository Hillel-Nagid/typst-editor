//! Cursor movement logic for bidirectional text

use crate::algorithm::BidiParagraph;
use crate::{ BidiError, Result };

/// Direction of cursor movement
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MovementDirection {
    Left,
    Right,
    Up,
    Down,
    Home,
    End,
}

/// Cursor movement in bidirectional text
pub struct CursorMovement;

impl CursorMovement {
    /// Move cursor in visual direction
    pub fn move_visual(
        paragraph: &BidiParagraph,
        logical_pos: usize,
        direction: MovementDirection
    ) -> Result<usize> {
        let text = paragraph.text();
        if text.is_empty() {
            return Ok(0);
        }

        match direction {
            MovementDirection::Left => {
                if logical_pos == 0 {
                    return Ok(0);
                }

                // Convert to visual, move left, convert back
                let visual_pos = paragraph.logical_to_visual(logical_pos);
                if visual_pos == 0 {
                    Ok(logical_pos) // Already at visual start
                } else {
                    let new_visual = visual_pos.saturating_sub(1);
                    Ok(paragraph.visual_to_logical(new_visual))
                }
            }

            MovementDirection::Right => {
                if logical_pos >= text.len() {
                    return Ok(text.len());
                }

                let visual_pos = paragraph.logical_to_visual(logical_pos);
                let text_len = text.len();
                if visual_pos >= text_len {
                    Ok(logical_pos) // Already at visual end
                } else {
                    let new_visual = (visual_pos + 1).min(text_len);
                    Ok(paragraph.visual_to_logical(new_visual))
                }
            }

            MovementDirection::Home => {
                // Move to visual start
                Ok(paragraph.visual_to_logical(0))
            }

            MovementDirection::End => {
                // Move to visual end
                Ok(paragraph.visual_to_logical(text.len()))
            }

            _ =>
                Err(
                    BidiError::ProcessingError(
                        "Vertical movement not implemented for single paragraph".to_string()
                    )
                ),
        }
    }

    /// Move cursor in logical direction (for navigation like Ctrl+Left/Right)
    pub fn move_logical(text: &str, logical_pos: usize, forward: bool) -> usize {
        if forward { (logical_pos + 1).min(text.len()) } else { logical_pos.saturating_sub(1) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ltr_movement() {
        let para = BidiParagraph::new("Hello".to_string(), None);

        // Moving right in LTR
        let new_pos = CursorMovement::move_visual(&para, 0, MovementDirection::Right).unwrap();
        assert_eq!(new_pos, 1);

        // Moving left in LTR
        let new_pos = CursorMovement::move_visual(&para, 1, MovementDirection::Left).unwrap();
        assert_eq!(new_pos, 0);
    }

    #[test]
    fn test_home_end() {
        let para = BidiParagraph::new("Hello".to_string(), None);

        let pos = CursorMovement::move_visual(&para, 3, MovementDirection::Home).unwrap();
        assert_eq!(pos, 0);

        let pos = CursorMovement::move_visual(&para, 3, MovementDirection::End).unwrap();
        assert_eq!(pos, para.text().len());
    }
}
