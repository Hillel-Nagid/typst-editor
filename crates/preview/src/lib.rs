//! Preview rendering for compiled Typst documents

pub mod renderer;
pub mod sync;
pub mod viewport;

pub use renderer::{ PreviewRenderer, RenderFormat };
pub use sync::{ SourceMapping, SyncManager };
pub use viewport::{ Viewport, ZoomLevel };

/// Preview errors
#[derive(Debug, thiserror::Error)]
pub enum PreviewError {
    #[error("Rendering failed: {0}")] RenderingFailed(String),

    #[error("Document not loaded")]
    DocumentNotLoaded,

    #[error("Invalid page: {0}")] InvalidPage(usize),
}

pub type Result<T> = std::result::Result<T, PreviewError>;
