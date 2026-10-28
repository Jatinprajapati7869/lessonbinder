use std::ffi::OsStr;
use std::path::Path;

use vault_core::whisper_cpp::{
    resolve_whisper_cpp_binary_in, whisper_cli_argv, WHISPER_CPP_BINARY,
};

#[test]
fn whisper_cli_argv_includes_model_media_and_json_output() {
    let argv = whisper_cli_argv(
        Path::new("/usr/bin/whisper-cli"),
        Path::new("/models/ggml.bin"),
        Path::new("/course/lesson.mp4"),
        Path::new("/tmp/out/prefix"),
    );
    assert_eq!(argv[0], "/usr/bin/whisper-cli");
    assert!(argv.windows(2).any(|w| w == ["-m", "/models/ggml.bin"]));
    assert!(argv.windows(2).any(|w| w == ["-f", "/course/lesson.mp4"]));
    assert!(argv.contains(&"-oj".to_string()));
    assert!(argv.windows(2).any(|w| w == ["-of", "/tmp/out/prefix"]));
}

#[test]
fn resolve_whisper_binary_uses_whisper_cli_name() {
    assert_eq!(WHISPER_CPP_BINARY, "whisper-cli");
    assert!(resolve_whisper_cpp_binary_in(Some(OsStr::new("/nonexistent-empty-path"))).is_err());
}
