//! Append-only command error log under app data (no note text or image payloads).

use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use serde_json::Value;

pub const COMMAND_LOG_FILE: &str = "command-errors.log";
pub const UNEXPECTED_COMMAND_ERROR_CODE: &str = "unexpected_command_error";

const REDACTED: &str = "[redacted]";

const REDACT_FIELD_NAMES: &[&str] = &[
    "front",
    "back",
    "text",
    "note_text",
    "image",
    "image_bytes",
    "frame_data",
    "frame_data_url",
    "data_url",
    "png",
    "body",
];

pub fn command_log_path(app_data_dir: &Path) -> std::path::PathBuf {
    app_data_dir.join(COMMAND_LOG_FILE)
}

/// Redact sensitive object fields if they are passed in `context` by mistake.
pub fn redact_json_value(value: Value) -> Value {
    match value {
        Value::Object(map) => {
            let mut out = serde_json::Map::new();
            for (key, val) in map {
                if REDACT_FIELD_NAMES.iter().any(|name| name == &key) {
                    out.insert(key, Value::String(REDACTED.to_string()));
                } else {
                    out.insert(key, redact_json_value(val));
                }
            }
            Value::Object(out)
        }
        Value::Array(items) => Value::Array(items.into_iter().map(redact_json_value).collect()),
        other => other,
    }
}

pub fn format_log_line(command: &str, error_code: &str, context: Option<&Value>) -> String {
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    match context {
        None => format!("ts={ts} command={command} code={error_code}"),
        Some(ctx) => format!(
            "ts={ts} command={command} code={error_code} context={}",
            ctx
        ),
    }
}

/// Log a command failure without note text or image bytes in `context` (fields are redacted).
pub fn log_unexpected_command_error(
    app_data_dir: &Path,
    command: &str,
    context: Option<Value>,
) -> std::io::Result<()> {
    log_command_error(
        app_data_dir,
        command,
        UNEXPECTED_COMMAND_ERROR_CODE,
        context,
    )
}

pub fn log_command_error(
    app_data_dir: &Path,
    command: &str,
    error_code: &str,
    context: Option<Value>,
) -> std::io::Result<()> {
    std::fs::create_dir_all(app_data_dir)?;
    let redacted = context.map(redact_json_value);
    let line = format_log_line(command, error_code, redacted.as_ref());
    let path = command_log_path(app_data_dir);
    let mut file = OpenOptions::new().create(true).append(true).open(path)?;
    writeln!(file, "{line}")?;
    Ok(())
}
