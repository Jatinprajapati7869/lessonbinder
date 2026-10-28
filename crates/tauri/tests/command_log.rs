use std::fs;

use lessonbinder_tauri::command_log::{
    command_log_path, log_command_error, redact_json_value, COMMAND_LOG_FILE,
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
