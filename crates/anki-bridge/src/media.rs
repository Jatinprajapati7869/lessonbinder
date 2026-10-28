//! Read files from the AuraVault profile media folder (local only).

use std::path::{Component, Path};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ReadMediaError {
    #[error("invalid media filename")]
    InvalidFilename,
    #[error("media file not found")]
    NotFound,
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}

/// Read bytes for `filename` when it is a single file name inside `media_folder`.
///
/// Rejects absolute paths, `..`, and path separators. Does not fetch remote URLs.
pub fn read_media(media_folder: &Path, filename: &str) -> Result<Vec<u8>, ReadMediaError> {
    validate_media_filename(filename)?;
    let path = media_folder.join(filename);
    if !path.is_file() {
        return Err(ReadMediaError::NotFound);
    }
    let media_root = media_folder.canonicalize()?;
    let resolved = path.canonicalize()?;
    if !resolved.starts_with(&media_root) {
        return Err(ReadMediaError::InvalidFilename);
    }
    std::fs::read(resolved).map_err(ReadMediaError::from)
}

fn validate_media_filename(filename: &str) -> Result<(), ReadMediaError> {
    if filename.is_empty() {
        return Err(ReadMediaError::InvalidFilename);
    }
    if filename.contains("..") {
        return Err(ReadMediaError::InvalidFilename);
    }
    let path = Path::new(filename);
    if path.is_absolute() {
        return Err(ReadMediaError::InvalidFilename);
    }
    for component in path.components() {
        match component {
            Component::Normal(_) => {}
            Component::CurDir => {}
            _ => return Err(ReadMediaError::InvalidFilename),
        }
    }
    Ok(())
}
