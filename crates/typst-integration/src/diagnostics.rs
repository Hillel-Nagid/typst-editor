//! Diagnostic information from Typst compilation

use serde::{ Deserialize, Serialize };
use std::path::PathBuf;

/// Severity level of a diagnostic
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Severity {
    Error,
    Warning,
    Info,
    Hint,
}

/// Source location for a diagnostic
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceLocation {
    pub file: PathBuf,
    pub line: usize,
    pub column: usize,
}

/// Represents a compilation diagnostic
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Diagnostic {
    /// Severity level
    pub severity: Severity,
    /// Diagnostic message
    pub message: String,
    /// Source location
    pub location: Option<SourceLocation>,
    /// Optional code for the diagnostic
    pub code: Option<String>,
    /// Related information (other locations)
    pub related: Vec<DiagnosticRelated>,
}

impl Diagnostic {
    pub fn error(message: String) -> Self {
        Self {
            severity: Severity::Error,
            message,
            location: None,
            code: None,
            related: Vec::new(),
        }
    }

    pub fn warning(message: String) -> Self {
        Self {
            severity: Severity::Warning,
            message,
            location: None,
            code: None,
            related: Vec::new(),
        }
    }

    pub fn with_location(mut self, location: SourceLocation) -> Self {
        self.location = Some(location);
        self
    }

    pub fn with_code(mut self, code: String) -> Self {
        self.code = Some(code);
        self
    }
}

/// Related diagnostic information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticRelated {
    pub location: SourceLocation,
    pub message: String,
}

/// Collection of diagnostics from a compilation
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DiagnosticList {
    pub diagnostics: Vec<Diagnostic>,
}

impl DiagnosticList {
    pub fn new() -> Self {
        Self {
            diagnostics: Vec::new(),
        }
    }

    pub fn add(&mut self, diagnostic: Diagnostic) {
        self.diagnostics.push(diagnostic);
    }

    pub fn has_errors(&self) -> bool {
        self.diagnostics.iter().any(|d| d.severity == Severity::Error)
    }

    pub fn errors(&self) -> impl Iterator<Item = &Diagnostic> {
        self.diagnostics.iter().filter(|d| d.severity == Severity::Error)
    }

    pub fn warnings(&self) -> impl Iterator<Item = &Diagnostic> {
        self.diagnostics.iter().filter(|d| d.severity == Severity::Warning)
    }

    pub fn clear(&mut self) {
        self.diagnostics.clear();
    }
}
