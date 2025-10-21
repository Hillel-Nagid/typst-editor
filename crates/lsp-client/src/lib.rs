//! LSP client for Typst language server

pub mod protocol;
pub mod requests;
pub mod notifications;

pub use protocol::{ LspClient, LspMessage };
pub use requests::RequestManager;
pub use notifications::NotificationHandler;

/// LSP errors
#[derive(Debug, thiserror::Error)]
pub enum LspError {
    #[error("Connection error: {0}")] ConnectionError(String),

    #[error("Protocol error: {0}")] ProtocolError(String),

    #[error("Request timeout")]
    Timeout,

    #[error("LSP not initialized")]
    NotInitialized,
}

pub type Result<T> = std::result::Result<T, LspError>;
