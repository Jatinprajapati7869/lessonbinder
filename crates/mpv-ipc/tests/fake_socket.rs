//! Fake mpv JSON IPC socket for tests (no mpv binary).

#[cfg(unix)]
mod unix {
    use std::io::{BufRead, BufReader, Write};
    use std::os::unix::net::UnixStream;
    use std::path::Path;
    use std::thread;

    use mpv_ipc::{LoadFileMode, MpvIpcClient};
    use serde_json::{json, Value};

    fn fake_mpv_server(mut server: UnixStream) {
        loop {
            let mut line = String::new();
            let read_ok = {
                let mut reader = BufReader::new(&mut server);
                reader.read_line(&mut line).ok().filter(|&n| n > 0)
            };
            if read_ok.is_none() {
                break;
            }
            let Ok(value) = serde_json::from_str::<Value>(line.trim()) else {
                continue;
            };
            let Some(command) = value.get("command").and_then(Value::as_array) else {
                continue;
            };
            let request_id = value.get("request_id").cloned().unwrap_or(Value::Null);
            let response = match command.first().and_then(Value::as_str) {
                Some("get_property")
                    if command.get(1).and_then(Value::as_str) == Some("time-pos") =>
                {
                    json!({ "data": 125.5, "error": "success", "request_id": request_id })
                }
                _ => json!({ "error": "success", "request_id": request_id }),
            };
            let mut out = serde_json::to_string(&response).expect("serialize");
            out.push('\n');
            if server.write_all(out.as_bytes()).is_err() {
                break;
            }
        }
    }

    #[test]
    fn client_speaks_json_ipc_over_fake_socket() {
        let (client_stream, server_stream) = UnixStream::pair().expect("socket pair");
        thread::spawn(move || fake_mpv_server(server_stream));

        let mut client = MpvIpcClient::new(client_stream);
        let path = Path::new("/tmp/lessonbinder-fake.mkv");

        client
            .loadfile(path, LoadFileMode::Replace)
            .expect("loadfile");
        client.set_pause(true).expect("pause");
        client.seek_absolute_seconds(30.0).expect("seek");
        client.set_volume(80.0).expect("volume");
        client.set_speed(1.25).expect("speed");
        let time_pos = client.get_time_pos().expect("time-pos");
        assert_eq!(time_pos, Some(125.5));
    }

    #[test]
    fn client_skips_event_lines_before_response() {
        let (client_stream, mut server_stream) = UnixStream::pair().expect("socket pair");
        thread::spawn(move || {
            let mut line = String::new();
            {
                let mut reader = BufReader::new(&mut server_stream);
                reader.read_line(&mut line).expect("read command");
            }
            let event = json!({"event": "file-loaded"}).to_string() + "\n";
            server_stream.write_all(event.as_bytes()).expect("event");
            let response = json!({"error": "success", "request_id": 0}).to_string() + "\n";
            server_stream
                .write_all(response.as_bytes())
                .expect("response");
        });

        let mut client = MpvIpcClient::new(client_stream);
        client
            .set_pause(false)
            .expect("set pause through event noise");
    }
}

#[cfg(not(unix))]
#[test]
fn client_speaks_json_ipc_over_fake_socket() {}
