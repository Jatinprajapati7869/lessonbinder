use std::fs;

use lessonbinder_tauri::command_log::{
    command_log_path, log_command_error, read_command_log_lines, redact_json_value,
    COMMAND_LOG_FILE, COMMAND_LOG_MAX_LINES,
};
use serde_json::json;
use tempfile::tempdir;

#[test]
fn redact_json_value_redacts_front_field() {
    let value = json!({
        "course_id": "course-1",
        "front": "should not appear in logs"
    });
    let redacted = redact_json_value(value);
    assert_eq!(redacted["course_id"], "course-1");
    assert_eq!(redacted["front"], "[redacted]");
    let serialized = redacted.to_string();
    assert!(!serialized.contains("should not appear"));
}

#[test]
fn log_command_error_writes_one_line_with_code() {
    let dir = tempdir().unwrap();
    log_command_error(
        dir.path(),
        "rescan_course",
        "rescan_course_not_found",
        Some(json!({ "course_id": "x", "front": "secret" })),
    )
    .unwrap();

    let path = command_log_path(dir.path());
    assert_eq!(path.file_name().unwrap(), COMMAND_LOG_FILE);
    let contents = fs::read_to_string(path).unwrap();
    assert_eq!(contents.lines().count(), 1);
    let line = contents.lines().next().unwrap();
    assert!(line.contains("command=rescan_course"));
    assert!(line.contains("code=rescan_course_not_found"));
    assert!(line.contains("course_id"));
    assert!(!line.contains("secret"));
    assert!(line.contains("[redacted]"));
}

#[test]
fn log_command_error_keeps_last_twenty_newest_lines() {
    let dir = tempdir().unwrap();
    for i in 0..25 {
        log_command_error(
            dir.path(),
            "import_course_folder",
            &format!("import_fail_{i:02}"),
            None,
        )
        .unwrap();
    }

    let path = command_log_path(dir.path());
    let file_text = fs::read_to_string(&path).unwrap();
    let file_lines: Vec<_> = file_text.lines().collect();
    assert_eq!(file_lines.len(), COMMAND_LOG_MAX_LINES);

    let read_back = read_command_log_lines(dir.path()).unwrap();
    assert_eq!(read_back.len(), COMMAND_LOG_MAX_LINES);
    assert!(read_back
        .iter()
        .zip(file_lines.iter())
        .all(|(a, b)| a == *b));

    assert!(read_back[0].contains("import_fail_05"));
    assert!(read_back[COMMAND_LOG_MAX_LINES - 1].contains("import_fail_24"));
    assert!(!read_back.iter().any(|line| line.contains("import_fail_04")));
}
