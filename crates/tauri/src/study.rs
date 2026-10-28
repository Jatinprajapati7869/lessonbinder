//! Launch Anki Desktop for review on the Lessonbinder profile.

use std::ffi::OsStr;
use std::path::{Path, PathBuf};

/// Executable name resolved from `PATH`.
pub const ANKI_DESKTOP_BINARY: &str = "anki";

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum LaunchAnkiError {
    #[error("anki binary not found on PATH")]
    AnkiNotFound,
    #[error("profile directory must have a parent path for Anki `-b` base folder")]
    ProfileDirHasNoParent,
}

/// Resolve the Anki Desktop binary using the process `PATH`.
pub fn resolve_anki_binary() -> Result<PathBuf, LaunchAnkiError> {
    resolve_anki_binary_in(std::env::var_os("PATH").as_deref())
}

/// Resolve `anki` in the given `PATH` list (for tests).
pub fn resolve_anki_binary_in(path: Option<&OsStr>) -> Result<PathBuf, LaunchAnkiError> {
    which::which_in(ANKI_DESKTOP_BINARY, path, ".").map_err(|_| LaunchAnkiError::AnkiNotFound)
}

/// Argument vector to open Anki Desktop on `profile_dir` without spawning.
///
/// `profile_dir` is Lessonbinder's `{app_data}/anki-profile/` folder (contains `collection.anki2`).
/// Anki expects `-b` on the parent folder and `-p` on the profile folder name.
pub fn anki_argv_for_profile(
    profile_dir: impl AsRef<Path>,
    anki_binary: impl AsRef<Path>,
) -> Result<Vec<String>, LaunchAnkiError> {
    let profile_dir = profile_dir.as_ref();
    let parent = profile_dir
        .parent()
        .ok_or(LaunchAnkiError::ProfileDirHasNoParent)?;
    let profile_name = profile_dir
        .file_name()
        .ok_or(LaunchAnkiError::ProfileDirHasNoParent)?;

    Ok(vec![
        anki_binary.as_ref().to_string_lossy().into_owned(),
        "-b".into(),
        parent.to_string_lossy().into_owned(),
        "-p".into(),
        profile_name.to_string_lossy().into_owned(),
    ])
}

/// Binary path plus argv for `std::process::Command` (still does not spawn).
pub fn anki_launch_command(
    profile_dir: impl AsRef<Path>,
) -> Result<(PathBuf, Vec<String>), LaunchAnkiError> {
    let program = resolve_anki_binary()?;
    let mut argv = anki_argv_for_profile(profile_dir, &program)?;
    let program = argv.remove(0);
    let program = PathBuf::from(program);
    Ok((program, argv))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::OsStr;

    #[test]
    fn anki_argv_for_profile() {
        let profile = Path::new("/data/Lessonbinder/anki-profile");
        let argv =
            super::anki_argv_for_profile(profile, "/usr/bin/anki").expect("argv should build");
        assert_eq!(
            argv,
            vec![
                "/usr/bin/anki",
                "-b",
                "/data/Lessonbinder",
                "-p",
                "anki-profile",
            ]
        );
    }

    #[test]
    fn missing_binary_error() {
        let err = resolve_anki_binary_in(Some(OsStr::new("/nonexistent-empty-path"))).unwrap_err();
        assert_eq!(err, LaunchAnkiError::AnkiNotFound);
    }
}
