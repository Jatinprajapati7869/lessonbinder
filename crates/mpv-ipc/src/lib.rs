//! Client for [mpv JSON IPC](https://mpv.io/manual/master/#json-ipc) over a byte stream (typically a Unix socket).
//!
//! Does not spawn or link against the mpv binary.

mod client;
mod error;

pub use client::{LoadFileMode, MpvIpcClient};
pub use error::MpvIpcError;
