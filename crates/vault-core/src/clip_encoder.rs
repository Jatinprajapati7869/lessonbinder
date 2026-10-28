//! Local audio clip extraction from lesson media (fake for tests; ffmpeg when on PATH).

use std::path::{Path, PathBuf};
use std::process::Command;

/// Maximum clip length accepted by [`ClipEncoder`] implementations.
pub const MAX_CLIP_DURATION_MS: i64 = 60_000;

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ClipEncoderError {
    #[error("clip_encoder_unavailable")]
    ClipEncoderUnavailable,
    #[error("clip end must be after start")]
    InvalidRange,
    #[error("clip longer than 60 seconds")]
    DurationTooLong,
    #[error("media file not found")]
    MediaNotFound,
    #[error("io error: {0}")]
    Io(String),
    #[error("clip encode failed: {0}")]
    EncodeFailed(String),
}

pub trait ClipEncoder {
    fn encode_clip(
        &self,
        local_media_path: &Path,
        start_ms: i64,
        end_ms: i64,
    ) -> Result<Vec<u8>, ClipEncoderError>;
}

pub fn validate_clip_range(start_ms: i64, end_ms: i64) -> Result<(), ClipEncoderError> {
    if end_ms <= start_ms {
        return Err(ClipEncoderError::InvalidRange);
    }
    if end_ms - start_ms > MAX_CLIP_DURATION_MS {
        return Err(ClipEncoderError::DurationTooLong);
    }
    Ok(())
}

/// Deterministic encoder for unit tests (ignores media path and range aside from validation).
#[derive(Debug, Clone)]
pub struct FakeClipEncoder {
    bytes: Vec<u8>,
}

impl FakeClipEncoder {
    pub fn new(bytes: Vec<u8>) -> Self {
        Self { bytes }
    }
}

impl ClipEncoder for FakeClipEncoder {
    fn encode_clip(
        &self,
        _local_media_path: &Path,
        start_ms: i64,
        end_ms: i64,
    ) -> Result<Vec<u8>, ClipEncoderError> {
        validate_clip_range(start_ms, end_ms)?;
        Ok(self.bytes.clone())
    }
}

/// ffmpeg CLI adapter. Requires `ffmpeg` on PATH.
#[derive(Debug, Clone)]
pub struct FfmpegClipEncoder {
    ffmpeg: Option<PathBuf>,
}

impl FfmpegClipEncoder {
    pub fn discover() -> Self {
        Self {
            ffmpeg: which::which("ffmpeg").ok(),
        }
    }

    pub fn without_ffmpeg() -> Self {
        Self { ffmpeg: None }
    }

    pub fn with_ffmpeg_path(path: impl Into<PathBuf>) -> Self {
        Self {
            ffmpeg: Some(path.into()),
        }
    }

    pub fn is_available(&self) -> bool {
        self.ffmpeg.as_ref().is_some_and(|path| path.is_file())
    }
}

impl ClipEncoder for FfmpegClipEncoder {
    fn encode_clip(
        &self,
        local_media_path: &Path,
        start_ms: i64,
        end_ms: i64,
    ) -> Result<Vec<u8>, ClipEncoderError> {
        validate_clip_range(start_ms, end_ms)?;
        let ffmpeg = self
            .ffmpeg
            .as_ref()
            .ok_or(ClipEncoderError::ClipEncoderUnavailable)?;
        if !ffmpeg.is_file() {
            return Err(ClipEncoderError::ClipEncoderUnavailable);
        }
        if !local_media_path.is_file() {
            return Err(ClipEncoderError::MediaNotFound);
        }

        let start_secs = start_ms as f64 / 1_000.0;
        let duration_secs = (end_ms - start_ms) as f64 / 1_000.0;

        let output = Command::new(ffmpeg)
            .arg("-hide_banner")
            .arg("-loglevel")
            .arg("error")
            .arg("-ss")
            .arg(format!("{start_secs:.3}"))
            .arg("-i")
            .arg(local_media_path)
            .arg("-t")
            .arg(format!("{duration_secs:.3}"))
            .arg("-vn")
            .arg("-f")
            .arg("mp3")
            .arg("pipe:1")
            .output()
            .map_err(|e| {
                if e.kind() == std::io::ErrorKind::NotFound {
                    ClipEncoderError::ClipEncoderUnavailable
                } else {
                    ClipEncoderError::Io(e.to_string())
                }
            })?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(ClipEncoderError::EncodeFailed(stderr.trim().to_string()));
        }

        Ok(output.stdout)
    }
}
