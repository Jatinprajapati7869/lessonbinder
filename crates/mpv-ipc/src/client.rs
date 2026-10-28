use std::io::{BufRead, BufReader, Read, Write};
use std::path::Path;

#[cfg(unix)]
use std::os::unix::net::UnixStream;

use serde::Serialize;
use serde_json::{json, Value};

use crate::MpvIpcError;

/// How `loadfile` replaces the current playlist entry (mpv `loadfile` mode argument).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoadFileMode {
    Replace,
    Append,
}

impl LoadFileMode {
    fn as_str(self) -> &'static str {
        match self {
            LoadFileMode::Replace => "replace",
            LoadFileMode::Append => "append",
        }
    }
}

/// JSON IPC client speaking mpv's newline-delimited protocol.
pub struct MpvIpcClient<S> {
    stream: S,
    next_request_id: u64,
}

#[cfg(unix)]
impl MpvIpcClient<UnixStream> {
    /// Connect to an mpv instance listening on a Unix socket (`--input-ipc-server=PATH`).
    pub fn connect(socket_path: &Path) -> Result<Self, MpvIpcError> {
        let stream = UnixStream::connect(socket_path)?;
        Ok(Self::new(stream))
    }
}

impl<S: Read + Write> MpvIpcClient<S> {
    pub fn new(stream: S) -> Self {
        Self {
            stream,
            next_request_id: 0,
        }
    }

    pub fn loadfile(&mut self, path: &Path, mode: LoadFileMode) -> Result<(), MpvIpcError> {
        self.run_command(&[
            Value::from("loadfile"),
            Value::from(path.to_string_lossy().into_owned()),
            Value::from(mode.as_str()),
        ])?;
        Ok(())
    }

    /// Seek to an absolute time in seconds (`seek` … `absolute`).
    pub fn seek_absolute_seconds(&mut self, seconds: f64) -> Result<(), MpvIpcError> {
        self.run_command(&[Value::from("seek"), json!(seconds), Value::from("absolute")])?;
        Ok(())
    }

    pub fn set_pause(&mut self, pause: bool) -> Result<(), MpvIpcError> {
        self.set_property("pause", json!(pause))
    }

    /// Volume as mpv `volume` property (0–100 scale in typical configs).
    pub fn set_volume(&mut self, volume: f64) -> Result<(), MpvIpcError> {
        self.set_property("volume", json!(volume))
    }

    pub fn set_speed(&mut self, speed: f64) -> Result<(), MpvIpcError> {
        self.set_property("speed", json!(speed))
    }

    /// Window geometry string (`WxH+X+Y`) when embedding with `--wid`.
    pub fn set_geometry(&mut self, geometry: &str) -> Result<(), MpvIpcError> {
        self.set_property("geometry", Value::from(geometry))
    }

    /// Current playback time in seconds (`time-pos`); `None` when mpv returns null data.
    pub fn get_time_pos(&mut self) -> Result<Option<f64>, MpvIpcError> {
        let response = self.run_command(&[Value::from("get_property"), Value::from("time-pos")])?;
        Ok(response.data.and_then(|v| v.as_f64()))
    }

    pub fn quit(&mut self) -> Result<(), MpvIpcError> {
        self.run_command(&[Value::from("quit")])?;
        Ok(())
    }

    fn set_property(&mut self, name: &str, value: Value) -> Result<(), MpvIpcError> {
        self.run_command(&[Value::from("set_property"), Value::from(name), value])?;
        Ok(())
    }

    fn run_command(&mut self, command: &[Value]) -> Result<IpcResponse, MpvIpcError> {
        let request_id = self.next_request_id;
        self.next_request_id += 1;

        let payload = json!({
            "command": Value::Array(command.to_vec()),
            "request_id": request_id,
        });
        self.write_line(&payload)?;

        self.read_command_response(request_id)
    }

    fn write_line(&mut self, value: &impl Serialize) -> Result<(), MpvIpcError> {
        let mut line = serde_json::to_string(value)?;
        line.push('\n');
        self.stream.write_all(line.as_bytes())?;
        self.stream.flush()?;
        Ok(())
    }

    fn read_command_response(&mut self, request_id: u64) -> Result<IpcResponse, MpvIpcError> {
        let mut reader = BufReader::new(&mut self.stream);
        loop {
            let mut line = String::new();
            let bytes = reader.read_line(&mut line)?;
            if bytes == 0 {
                return Err(MpvIpcError::Protocol("unexpected end of IPC stream".into()));
            }
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }
            let value: Value = serde_json::from_str(trimmed)?;
            if value.get("event").is_some() {
                continue;
            }
            let id_matches = match value.get("request_id").and_then(Value::as_u64) {
                None => true,
                Some(id) => id == request_id,
            };
            if !id_matches {
                continue;
            }
            return IpcResponse::from_value(value);
        }
    }
}

struct IpcResponse {
    data: Option<Value>,
}

impl IpcResponse {
    fn from_value(value: Value) -> Result<Self, MpvIpcError> {
        let error = value
            .get("error")
            .and_then(Value::as_str)
            .ok_or_else(|| MpvIpcError::Protocol("response missing error field".into()))?;
        if error != "success" {
            return Err(MpvIpcError::CommandFailed(error.to_string()));
        }
        Ok(Self {
            data: value.get("data").cloned(),
        })
    }
}
