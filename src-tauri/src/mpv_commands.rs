use std::path::PathBuf;
use std::process::{Child, Command};
use std::sync::Mutex;
use std::time::{Duration, Instant};

use lessonbinder_tauri::command_error::CommandError;
use lessonbinder_tauri::mpv::{
    mpv_embed_argv, resolve_mpv_binary, MpvEmbedRect, MpvHandoffError, MPV_NOT_ON_PATH_MESSAGE,
};
use mpv_ipc::{LoadFileMode, MpvIpcClient};
use raw_window_handle::{HasWindowHandle, RawWindowHandle};
use serde::Deserialize;
use tauri::Window;

use crate::app_state::AppState;

pub struct MpvLessonSession {
    child: Child,
    ipc_socket: PathBuf,
}

pub struct MpvPlaybackState {
    session: Mutex<Option<MpvLessonSession>>,
}

impl MpvPlaybackState {
    pub fn new() -> Self {
        Self {
            session: Mutex::new(None),
        }
    }

    fn stop_locked(session: &mut Option<MpvLessonSession>) {
        if let Some(mut active) = session.take() {
            if let Ok(mut client) = MpvIpcClient::connect(&active.ipc_socket) {
                let _ = client.quit();
            }
            let _ = active.child.kill();
            let _ = active.child.wait();
            let _ = std::fs::remove_file(&active.ipc_socket);
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct MpvEmbedRectDto {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

impl From<MpvEmbedRectDto> for MpvEmbedRect {
    fn from(value: MpvEmbedRectDto) -> Self {
        Self {
            x: value.x,
            y: value.y,
            width: value.width,
            height: value.height,
        }
    }
}

fn mpv_not_found_error() -> String {
    CommandError {
        code: "mpv_not_found".into(),
        message: MPV_NOT_ON_PATH_MESSAGE.into(),
    }
    .to_invoke_string()
}

fn handoff_error(err: MpvHandoffError) -> String {
    match err {
        MpvHandoffError::MpvNotFound => mpv_not_found_error(),
        MpvHandoffError::MediaMissing => CommandError {
            code: "mpv_media_missing".into(),
            message: err.to_string(),
        }
        .to_invoke_string(),
    }
}

fn embed_wid_from_window(window: &Window) -> Result<u64, String> {
    let handle = window
        .window_handle()
        .map_err(|e| format!("window handle: {e}"))?;
    match handle.as_raw() {
        RawWindowHandle::Xlib(x) => Ok(x.window),
        RawWindowHandle::Wayland(w) => Ok(w.surface.as_ptr() as u64),
        RawWindowHandle::Win32(w) => Ok(w.hwnd.get() as u64),
        RawWindowHandle::AppKit(a) => Ok(a.ns_view.as_ptr() as u64),
        other => Err(format!(
            "unsupported window handle for mpv embed: {other:?}"
        )),
    }
}

fn wait_for_ipc_socket(path: &std::path::Path) -> Result<(), String> {
    let deadline = Instant::now() + Duration::from_secs(3);
    while Instant::now() < deadline {
        if path.exists() {
            return Ok(());
        }
        std::thread::sleep(Duration::from_millis(25));
    }
    Err("mpv IPC socket did not appear".into())
}

fn spawn_embedded_mpv(
    media_path: &std::path::Path,
    wid: u64,
    rect: MpvEmbedRect,
    resume_seconds: f64,
    app_data_dir: &std::path::Path,
) -> Result<MpvLessonSession, String> {
    if !media_path.is_file() {
        return Err(handoff_error(MpvHandoffError::MediaMissing));
    }

    let mpv_program = resolve_mpv_binary().map_err(handoff_error)?;
    let ipc_socket = app_data_dir.join(format!("mpv-ipc-{}.sock", uuid::Uuid::new_v4()));

    let argv = mpv_embed_argv(&mpv_program, &ipc_socket, wid, rect);
    let mut child = Command::new(&argv[0])
        .args(&argv[1..])
        .spawn()
        .map_err(|e| format!("failed to start mpv: {e}"))?;

    if let Err(error) = wait_for_ipc_socket(&ipc_socket) {
        let _ = child.kill();
        let _ = std::fs::remove_file(&ipc_socket);
        return Err(error);
    }

    let mut client = MpvIpcClient::connect(&ipc_socket).map_err(|e| e.to_string())?;
    client
        .loadfile(media_path, LoadFileMode::Replace)
        .map_err(|e| e.to_string())?;
    if resume_seconds > 0.0 {
        client
            .seek_absolute_seconds(resume_seconds)
            .map_err(|e| e.to_string())?;
    }
    client.set_pause(false).map_err(|e| e.to_string())?;

    Ok(MpvLessonSession { child, ipc_socket })
}

#[tauri::command]
pub fn start_mpv_lesson_handoff(
    window: Window,
    state: tauri::State<'_, AppState>,
    media_path: String,
    rect: MpvEmbedRectDto,
    resume_seconds: Option<f64>,
) -> Result<(), String> {
    let path = std::path::PathBuf::from(&media_path);
    let wid = embed_wid_from_window(&window)?;
    let rect = MpvEmbedRect::from(rect);
    let resume_seconds = resume_seconds.unwrap_or(0.0).max(0.0);

    let mut guard = state
        .mpv
        .session
        .lock()
        .map_err(|_| "mpv session lock poisoned".to_string())?;
    MpvPlaybackState::stop_locked(&mut guard);

    let session = spawn_embedded_mpv(&path, wid, rect, resume_seconds, &state.app_data_dir)?;
    *guard = Some(session);
    Ok(())
}

#[tauri::command]
pub fn update_mpv_lesson_embed_rect(
    state: tauri::State<'_, AppState>,
    rect: MpvEmbedRectDto,
) -> Result<(), String> {
    let rect = MpvEmbedRect::from(rect);
    let guard = state
        .mpv
        .session
        .lock()
        .map_err(|_| "mpv session lock poisoned".to_string())?;
    let Some(session) = guard.as_ref() else {
        return Ok(());
    };

    let mut client = MpvIpcClient::connect(&session.ipc_socket).map_err(|e| e.to_string())?;
    client
        .set_geometry(&rect.geometry_arg())
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn stop_mpv_lesson_handoff(state: tauri::State<'_, AppState>) -> Result<(), String> {
    let mut guard = state
        .mpv
        .session
        .lock()
        .map_err(|_| "mpv session lock poisoned".to_string())?;
    MpvPlaybackState::stop_locked(&mut guard);
    Ok(())
}
