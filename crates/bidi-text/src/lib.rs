//! Bidirectional text processing using Unicode Bidirectional Algorithm (UAX #9)

pub mod algorithm;
pub mod layout;
pub mod cursor;

pub use algorithm::{ BidiParagraph, Direction, BidiInfo };
pub use layout::{ VisualRun, VisualLine, BidiLayoutEngine };
pub use cursor::{ CursorMovement, MovementDirection, TextPosition };

/// Common error types for bidi text processing
#[derive(Debug, thiserror::Error)]
pub enum BidiError {
    #[error("Invalid paragraph index: {0}")] InvalidParagraph(usize),

    #[error("Invalid position: {0}")] InvalidPosition(usize),

    #[error("Processing error: {0}")] ProcessingError(String),
}

pub type Result<T> = std::result::Result<T, BidiError>;
