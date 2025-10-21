//! LSP protocol implementation

use crate::{ LspError, Result };
use lsp_types::*;
use serde_json::Value;
use tokio::sync::mpsc;

#[derive(Debug, Clone)]
pub struct ResponseError {
    pub code: i32,
    pub message: String,
}

/// LSP message types
#[derive(Debug, Clone)]
pub enum LspMessage {
    Request {
        id: i64,
        method: String,
        params: Value,
    },
    Response {
        id: i64,
        result: Option<Value>,
        error: Option<ResponseError>,
    },
    Notification {
        method: String,
        params: Value,
    },
}

/// LSP client state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LspState {
    NotStarted,
    Initializing,
    Running,
    ShuttingDown,
    Stopped,
}

/// LSP client
pub struct LspClient {
    /// Current state
    state: LspState,
    /// Message sender
    message_tx: mpsc::Sender<LspMessage>,
    /// Message receiver
    message_rx: mpsc::Receiver<LspMessage>,
    /// Next request ID
    next_id: i64,
    /// Server capabilities
    capabilities: Option<ServerCapabilities>,
}

impl LspClient {
    pub fn new() -> Self {
        let (message_tx, message_rx) = mpsc::channel(100);

        Self {
            state: LspState::NotStarted,
            message_tx,
            message_rx,
            next_id: 1,
            capabilities: None,
        }
    }

    /// Start the LSP server
    pub async fn start(&mut self) -> Result<()> {
        if self.state != LspState::NotStarted {
            return Err(LspError::ProtocolError("LSP already started".to_string()));
        }

        self.state = LspState::Initializing;

        // TODO: Actually spawn LSP process and connect
        // For now, just transition to running
        self.state = LspState::Running;

        Ok(())
    }

    /// Initialize the LSP
    pub async fn initialize(&mut self, root_uri: Url) -> Result<InitializeResult> {
        if self.state != LspState::Initializing && self.state != LspState::Running {
            return Err(LspError::NotInitialized);
        }

        let _params = InitializeParams {
            process_id: Some(std::process::id()),
            root_uri: Some(root_uri),
            capabilities: ClientCapabilities {
                text_document: Some(TextDocumentClientCapabilities {
                    synchronization: Some(TextDocumentSyncClientCapabilities {
                        dynamic_registration: Some(false),
                        will_save: Some(true),
                        will_save_wait_until: Some(false),
                        did_save: Some(true),
                    }),
                    completion: Some(CompletionClientCapabilities {
                        dynamic_registration: Some(false),
                        completion_item: Some(CompletionItemCapability {
                            snippet_support: Some(true),
                            ..Default::default()
                        }),
                        ..Default::default()
                    }),
                    hover: Some(HoverClientCapabilities {
                        dynamic_registration: Some(false),
                        content_format: Some(vec![MarkupKind::Markdown, MarkupKind::PlainText]),
                    }),
                    ..Default::default()
                }),
                ..Default::default()
            },
            ..Default::default()
        };

        // TODO: Actually send request and wait for response
        // For now, return mock result
        let result = InitializeResult {
            capabilities: ServerCapabilities::default(),
            server_info: None,
        };

        self.capabilities = Some(result.capabilities.clone());
        self.state = LspState::Running;

        Ok(result)
    }

    /// Send a request
    pub async fn send_request(&mut self, method: String, params: Value) -> Result<i64> {
        if self.state != LspState::Running {
            return Err(LspError::NotInitialized);
        }

        let id = self.next_id;
        self.next_id += 1;

        self.message_tx
            .send(LspMessage::Request { id, method, params }).await
            .map_err(|e| LspError::ConnectionError(e.to_string()))?;

        Ok(id)
    }

    /// Send a notification
    pub async fn send_notification(&self, method: String, params: Value) -> Result<()> {
        self.message_tx
            .send(LspMessage::Notification { method, params }).await
            .map_err(|e| LspError::ConnectionError(e.to_string()))
    }

    /// Receive a message
    pub async fn receive_message(&mut self) -> Option<LspMessage> {
        self.message_rx.recv().await
    }

    /// Get server capabilities
    pub fn capabilities(&self) -> Option<&ServerCapabilities> {
        self.capabilities.as_ref()
    }

    /// Get current state
    pub fn state(&self) -> LspState {
        self.state
    }

    /// Shutdown the LSP
    pub async fn shutdown(&mut self) -> Result<()> {
        if self.state != LspState::Running {
            return Ok(());
        }

        self.state = LspState::ShuttingDown;

        // TODO: Send shutdown request

        self.state = LspState::Stopped;
        Ok(())
    }
}

impl Default for LspClient {
    fn default() -> Self {
        Self::new()
    }
}
