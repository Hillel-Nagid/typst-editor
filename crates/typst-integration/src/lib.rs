//! Typst compiler integration

pub mod compiler;
pub mod diagnostics;
pub mod world;

pub use compiler::{ CompileRequest, CompileResult, Compiler };
pub use diagnostics::{ Diagnostic, Severity };
pub use world::SystemWorld;

/// Common error types
#[derive(Debug, thiserror::Error)]
pub enum TypstError {
    #[error("Compilation failed: {0}")] CompilationFailed(String),

    #[error("File not found: {0}")] FileNotFound(String),

    #[error("IO error: {0}")] IoError(#[from] std::io::Error),

    #[error("World error: {0}")] WorldError(String),
}

pub type Result<T> = std::result::Result<T, TypstError>;
