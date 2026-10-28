use std::io;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum MpvIpcError {
    #[error("mpv IPC I/O: {0}")]
    Io(#[from] io::Error),
    #[error("mpv IPC JSON: {0}")]
    Json(#[from] serde_json::Error),
    #[error("mpv IPC protocol: {0}")]
    Protocol(String),
    #[error("mpv IPC command failed: {0}")]
    CommandFailed(String),
}
