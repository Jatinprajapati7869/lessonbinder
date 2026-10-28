//! Resolve `mpv` on PATH and build argv for embedded playback (no separate mpv window).

use std::ffi::OsStr;
use std::path::{Path, PathBuf};

pub const MPV_BINARY: &str = "mpv";

/// User-visible sentence when `mpv` is not on PATH (prompt 355).
pub const MPV_NOT_ON_PATH_MESSAGE: &str =
    "Install mpv and add it to PATH to play files the player cannot decode.";

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum MpvHandoffError {
    #[error("{MPV_NOT_ON_PATH_MESSAGE}")]
    MpvNotFound,
    #[error("media path does not exist")]
    MediaMissing,
}

pub fn resolve_mpv_binary() -> Result<PathBuf, MpvHandoffError> {
    resolve_mpv_binary_in(std::env::var_os("PATH").as_deref())
}

pub fn resolve_mpv_binary_in(path: Option<&OsStr>) -> Result<PathBuf, MpvHandoffError> {
    which::which_in(MPV_BINARY, path, ".").map_err(|_| MpvHandoffError::MpvNotFound)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MpvEmbedRect {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

impl MpvEmbedRect {
    pub fn geometry_arg(self) -> String {
        format!(
            "{}x{}+{}+{}",
            self.width.max(1),
            self.height.max(1),
            self.x.max(0),
            self.y.max(0)
        )
    }
}

/// mpv argv for embedded idle instance (loadfile happens over JSON IPC).
pub fn mpv_embed_argv(
    mpv_binary: impl AsRef<Path>,
    ipc_socket: impl AsRef<Path>,
    wid: u64,
    rect: MpvEmbedRect,
) -> Vec<String> {
    let mpv_binary = mpv_binary.as_ref().to_string_lossy().into_owned();
    let ipc_socket = ipc_socket.as_ref().to_string_lossy().into_owned();
    vec![
        mpv_binary,
        "--no-terminal".into(),
        "--idle=yes".into(),
        "--force-window=embed".into(),
        format!("--wid={wid:#x}"),
        format!("--geometry={}", rect.geometry_arg()),
        "--osc=no".into(),
        "--input-default-bindings=no".into(),
        "--input-builtin-bindings=no".into(),
        "--keep-open=yes".into(),
        format!("--input-ipc-server={ipc_socket}"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn missing_mpv_error() {
        let err = resolve_mpv_binary_in(Some(OsStr::new("/nonexistent-empty-path"))).unwrap_err();
        assert_eq!(err, MpvHandoffError::MpvNotFound);
    }

    #[test]
    fn embed_argv_uses_embed_and_ipc_not_osc() {
        let argv = mpv_embed_argv(
            "/usr/bin/mpv",
            "/tmp/lessonbinder-mpv.sock",
            0xabc,
            MpvEmbedRect {
                x: 10,
                y: 20,
                width: 640,
                height: 360,
            },
        );
        assert!(argv.iter().any(|a| a == "--force-window=embed"));
        assert!(!argv
            .iter()
            .any(|a| a.starts_with("--force-window") && a != "--force-window=embed"));
        assert!(argv.iter().any(|a| a.starts_with("--input-ipc-server=")));
        assert!(argv.iter().any(|a| a == "--osc=no"));
        assert!(!argv.iter().any(|a| a == "--osc=yes"));
        assert!(argv.iter().any(|a| a == "--wid=0xabc"));
        assert!(argv.iter().any(|a| a == "--geometry=640x360+10+20"));
    }
}
