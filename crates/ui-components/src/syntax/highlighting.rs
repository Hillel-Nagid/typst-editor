//! Syntax highlighting using Typst's own parser
//!
//! Phase 3.3: Syntax Highlighting

use typst_syntax::{ parse, SyntaxNode, SyntaxKind };
use std::sync::Arc;

/// Syntax highlighter using Typst's parser
pub struct SyntaxHighlighter {
    // Typst parser is stateless, no need to store state
}

impl SyntaxHighlighter {
    pub fn new() -> Self {
        Self {}
    }

    /// Parse and highlight Typst text
    pub fn highlight(&self, text: &str) -> Arc<HighlightResult> {
        let root = parse(text);
        let tokens = Self::extract_tokens(&root);

        Arc::new(HighlightResult {
            root,
            tokens,
        })
    }

    /// Extract tokens from the syntax tree for highlighting
    /// Uses iterative approach to avoid stack overflow on deep trees
    fn extract_tokens(node: &SyntaxNode) -> Vec<HighlightToken> {
        let mut tokens = Vec::new();
        let mut stack = vec![node];

        while let Some(current) = stack.pop() {
            let token_type = Self::syntax_kind_to_token_type(current.kind());

            if let Some(token_type) = token_type {
                tokens.push(HighlightToken {
                    start: 0, // TODO: Calculate actual byte offset in Phase 3 implementation
                    end: 0, // TODO: Calculate actual byte offset in Phase 3 implementation
                    token_type,
                });
            }

            // Push children in reverse order to process in correct order
            for child in current.children().rev() {
                stack.push(child);
            }
        }

        tokens
    }

    /// Map Typst SyntaxKind to our TokenType
    fn syntax_kind_to_token_type(kind: SyntaxKind) -> Option<TokenType> {
        match kind {
            // Keywords
            | SyntaxKind::Let
            | SyntaxKind::Set
            | SyntaxKind::Show
            | SyntaxKind::If
            | SyntaxKind::Else
            | SyntaxKind::For
            | SyntaxKind::While
            | SyntaxKind::Break
            | SyntaxKind::Continue
            | SyntaxKind::Return
            | SyntaxKind::Import
            | SyntaxKind::Include
            | SyntaxKind::As
            | SyntaxKind::In
            | SyntaxKind::Not
            | SyntaxKind::And
            | SyntaxKind::Or => Some(TokenType::Keyword),

            // Functions and identifiers
            SyntaxKind::FuncCall => Some(TokenType::Function),
            SyntaxKind::Ident => Some(TokenType::Variable),

            // Literals
            SyntaxKind::Str | SyntaxKind::RawLang | SyntaxKind::RawTrimmed =>
                Some(TokenType::String),
            SyntaxKind::Int | SyntaxKind::Float | SyntaxKind::Bool => Some(TokenType::Constant),

            // Comments
            SyntaxKind::LineComment | SyntaxKind::BlockComment => Some(TokenType::Comment),

            // Operators
            | SyntaxKind::Plus
            | SyntaxKind::Minus
            | SyntaxKind::Star
            | SyntaxKind::Slash
            | SyntaxKind::Eq
            | SyntaxKind::EqEq
            | SyntaxKind::ExclEq
            | SyntaxKind::Lt
            | SyntaxKind::LtEq
            | SyntaxKind::Gt
            | SyntaxKind::GtEq => Some(TokenType::Operator),

            // Math mode
            SyntaxKind::Math | SyntaxKind::MathAlignPoint | SyntaxKind::MathIdent =>
                Some(TokenType::Math),

            // Markup
            SyntaxKind::Markup | SyntaxKind::Strong | SyntaxKind::Emph | SyntaxKind::Heading =>
                Some(TokenType::Markup),

            // Labels and references
            SyntaxKind::Label => Some(TokenType::Label),
            SyntaxKind::Ref => Some(TokenType::Reference),

            _ => None,
        }
    }
}

impl Default for SyntaxHighlighter {
    fn default() -> Self {
        Self::new()
    }
}

/// Highlight result with token information
pub struct HighlightResult {
    pub root: SyntaxNode,
    pub tokens: Vec<HighlightToken>,
}

/// A highlighted token
pub struct HighlightToken {
    pub start: usize,
    pub end: usize,
    pub token_type: TokenType,
}

/// Token types for syntax highlighting
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenType {
    Keyword,
    Function,
    Variable,
    Constant,
    String,
    Comment,
    Type,
    Operator,
    Markup,
    Math,
    Label,
    Reference,
}
