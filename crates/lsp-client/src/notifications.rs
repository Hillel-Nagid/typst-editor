//! LSP notification handling

use lsp_types::*;
use serde_json::Value;

/// Notification handler trait
pub trait NotificationHandler: Send + Sync {
    fn handle_notification(&mut self, method: &str, params: Value);
}

/// Default notification handler
pub struct DefaultNotificationHandler;

impl NotificationHandler for DefaultNotificationHandler {
    fn handle_notification(&mut self, method: &str, _params: Value) {
        tracing::debug!("Received notification: {}", method);
    }
}

/// Specific notification types
#[derive(Debug, Clone)]
pub enum Notification {
    PublishDiagnostics(PublishDiagnosticsParams),
    ShowMessage(ShowMessageParams),
    LogMessage(LogMessageParams),
    Other {
        method: String,
        params: Value,
    },
}

impl Notification {
    pub fn from_method_and_params(method: &str, params: Value) -> Self {
        match method {
            "textDocument/publishDiagnostics" => {
                if let Ok(params) = serde_json::from_value(params) {
                    Notification::PublishDiagnostics(params)
                } else {
                    Notification::Other {
                        method: method.to_string(),
                        params: Value::Null,
                    }
                }
            }
            "window/showMessage" => {
                if let Ok(params) = serde_json::from_value(params) {
                    Notification::ShowMessage(params)
                } else {
                    Notification::Other {
                        method: method.to_string(),
                        params: Value::Null,
                    }
                }
            }
            "window/logMessage" => {
                if let Ok(params) = serde_json::from_value(params) {
                    Notification::LogMessage(params)
                } else {
                    Notification::Other {
                        method: method.to_string(),
                        params: Value::Null,
                    }
                }
            }
            _ =>
                Notification::Other {
                    method: method.to_string(),
                    params,
                },
        }
    }
}
