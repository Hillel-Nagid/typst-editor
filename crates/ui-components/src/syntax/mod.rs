//! Syntax highlighting subsystem
//!
//! Phase 3.3: Syntax Highlighting

pub mod highlighting;
pub mod theme;

pub use highlighting::{ SyntaxHighlighter, HighlightResult, TokenType };
pub use theme::{ Theme, ThemeManager, ThemeVariant, ColorScheme };
