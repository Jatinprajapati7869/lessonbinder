//! whisper.cpp CLI argument builder and JSON parsing (no network; binary on PATH).

use std::ffi::OsStr;
use std::path::{Path, PathBuf};

use crate::cues::SubtitleCue;

/// Executable name resolved from `PATH` (whisper.cpp `whisper-cli` target).
pub const WHISPER_CPP_BINARY: &str = "whisper-cli";

#[derive(Debug, thiserror::Error)]
pub enum WhisperCppError {
    #[error("whisper-cli binary not found on PATH")]
    BinaryNotFound,
    #[error("whisper json parse error: {0}")]
    JsonParse(String),
    #[error("whisper run failed: {0}")]
    RunFailed(String),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

pub fn resolve_whisper_cpp_binary() -> Result<PathBuf, WhisperCppError> {
    resolve_whisper_cpp_binary_in(std::env::var_os("PATH").as_deref())
}

pub fn resolve_whisper_cpp_binary_in(path: Option<&OsStr>) -> Result<PathBuf, WhisperCppError> {
    which::which_in(WHISPER_CPP_BINARY, path, ".").map_err(|_| WhisperCppError::BinaryNotFound)
}

/// Argument vector for `whisper-cli` (program is `argv[0]`).
pub fn whisper_cli_argv(
    binary: &Path,
    model_path: &Path,
    media_path: &Path,
    output_prefix: &Path,
) -> Vec<String> {
    vec![
        binary.to_string_lossy().into_owned(),
        "-m".into(),
        model_path.to_string_lossy().into_owned(),
        "-f".into(),
        media_path.to_string_lossy().into_owned(),
        "-oj".into(),
        "-of".into(),
        output_prefix.to_string_lossy().into_owned(),
    ]
}

pub fn parse_whisper_json_output(text: &str) -> Result<Vec<SubtitleCue>, WhisperCppError> {
    let value: serde_json::Value =
        serde_json::from_str(text).map_err(|e| WhisperCppError::JsonParse(e.to_string()))?;
    let segments = value
        .get("transcription")
        .and_then(|v| v.as_array())
        .ok_or_else(|| WhisperCppError::JsonParse("missing transcription array".into()))?;

    let mut cues = Vec::with_capacity(segments.len());
    for segment in segments {
        let text = segment
            .get("text")
            .and_then(|v| v.as_str())
            .map(str::trim)
            .filter(|t| !t.is_empty())
            .ok_or_else(|| WhisperCppError::JsonParse("segment missing text".into()))?;
        let offsets = segment
            .get("offsets")
            .ok_or_else(|| WhisperCppError::JsonParse("segment missing offsets".into()))?;
        let start_ms = offsets
            .get("from")
            .and_then(|v| v.as_i64())
            .ok_or_else(|| WhisperCppError::JsonParse("segment missing start offset".into()))?;
        let end_ms = offsets
            .get("to")
            .and_then(|v| v.as_i64())
            .ok_or_else(|| WhisperCppError::JsonParse("segment missing end offset".into()))?;
        cues.push(SubtitleCue {
            start_ms,
            end_ms,
            text: text.to_string(),
        });
    }
    Ok(cues)
}
