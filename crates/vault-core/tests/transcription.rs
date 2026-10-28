use std::fs;

use tempfile::tempdir;
use vault_core::cues::SubtitleCue;
use vault_core::import::import_folder;
use vault_core::order::{accept_proposal, learning_order, propose_order};
use vault_core::transcription::{
    list_transcription_cues, run_lesson_transcription, transcription_disabled,
    transcription_engine_from_settings, FakeTranscriptionEngine, TranscriptionEngine,
    TranscriptionError,
};
use vault_core::vault_settings::VaultSettings;
use vault_core::VaultDb;

#[test]
fn disabled_without_model_path() {
    let dir = tempdir().unwrap();
    fs::write(dir.path().join("a.mp4"), b"").unwrap();
    fs::write(dir.path().join("b.mp4"), b"").unwrap();

    let db = VaultDb::open_in_memory().unwrap();
    let report = import_folder(&db, dir.path()).unwrap();
    let course_id = &report.course_id;
    let lesson_ids = &report.lesson_ids;

    let proposed = vec![lesson_ids[1].clone(), lesson_ids[0].clone()];
    let proposal_id = propose_order(&db, course_id, &proposed).unwrap();
    accept_proposal(&db, &proposal_id).unwrap();
    db.set_resume_position(&lesson_ids[0], 42_000).unwrap();

    let order_before = learning_order(&db, course_id).unwrap();
    let progress_before = db.progress_for_lesson(&lesson_ids[0]).unwrap();

    let err = run_lesson_transcription(&db, &lesson_ids[0], &transcription_disabled()).unwrap_err();
    assert_eq!(err, TranscriptionError::TranscriptionDisabled);

    assert_eq!(learning_order(&db, course_id).unwrap(), order_before);
    assert_eq!(
        db.progress_for_lesson(&lesson_ids[0]).unwrap(),
        progress_before
    );
}

#[test]
fn empty_model_path_engine_is_disabled() {
    let db = VaultDb::open_in_memory().unwrap();
    let settings = VaultSettings {
        tesseract_language: "eng".into(),
        transcription_model_path: String::new(),
        anki_desktop_binary_path: "anki".into(),
        ui_theme: vault_core::vault_settings::UiTheme::Light,
        playback_speed: vault_core::vault_settings::PlaybackSpeed::default(),
        player_volume: vault_core::vault_settings::DEFAULT_PLAYER_VOLUME,
        player_muted: false,
    };
    let engine = transcription_engine_from_settings(&settings, None);
    let dir = tempdir().unwrap();
    fs::write(dir.path().join("clip.mp4"), b"").unwrap();
    let lesson_id = import_folder(&db, dir.path()).unwrap().lesson_ids[0].clone();

    let err = run_lesson_transcription(&db, &lesson_id, &engine).unwrap_err();
    assert_eq!(err, TranscriptionError::TranscriptionDisabled);
    assert!(list_transcription_cues(&db, &lesson_id).unwrap().is_empty());
}

#[test]
fn writes_segments_when_configured() {
    let dir = tempdir().unwrap();
    fs::write(dir.path().join("a.mp4"), b"").unwrap();
    fs::write(dir.path().join("b.mp4"), b"").unwrap();

    let db = VaultDb::open_in_memory().unwrap();
    let report = import_folder(&db, dir.path()).unwrap();
    let course_id = &report.course_id;
    let lesson_id = &report.lesson_ids[0];

    let proposed = vec![report.lesson_ids[1].clone(), lesson_id.clone()];
    let proposal_id = propose_order(&db, course_id, &proposed).unwrap();
    accept_proposal(&db, &proposal_id).unwrap();
    db.set_resume_position(lesson_id, 9_000).unwrap();

    let order_before = learning_order(&db, course_id).unwrap();
    let progress_before = db.progress_for_lesson(lesson_id).unwrap();

    let cues = vec![
        SubtitleCue {
            start_ms: 0,
            end_ms: 1_000,
            text: "One".into(),
        },
        SubtitleCue {
            start_ms: 1_500,
            end_ms: 2_500,
            text: "Two".into(),
        },
        SubtitleCue {
            start_ms: 3_000,
            end_ms: 4_000,
            text: "Three".into(),
        },
    ];
    let engine = FakeTranscriptionEngine::new(cues.clone());
    run_lesson_transcription(&db, lesson_id, &engine).unwrap();

    assert_eq!(list_transcription_cues(&db, lesson_id).unwrap(), cues);
    assert_eq!(learning_order(&db, course_id).unwrap(), order_before);
    assert_eq!(db.progress_for_lesson(lesson_id).unwrap(), progress_before);
}

#[test]
fn no_network_weight_download() {
    use std::path::Path;

    use vault_core::whisper_cpp::whisper_cli_argv;

    let argv = whisper_cli_argv(
        Path::new("/usr/bin/whisper-cli"),
        Path::new("/models/whisper.bin"),
        Path::new("/media/lesson.mp4"),
        Path::new("/tmp/out"),
    );
    for arg in argv {
        assert!(
            !arg.starts_with("http://") && !arg.starts_with("https://"),
            "whisper argv must not reference remote URLs: {arg}"
        );
    }
}

#[derive(Debug, Clone, Copy)]
struct FailingTranscriptionEngine;

impl TranscriptionEngine for FailingTranscriptionEngine {
    fn transcribe_lesson(&self, _db: &VaultDb, _lesson_id: &str) -> Result<(), TranscriptionError> {
        Err(TranscriptionError::Whisper("injected failure".into()))
    }
}

#[test]
fn failure_leaves_lesson_unchanged() {
    let dir = tempdir().unwrap();
    fs::write(dir.path().join("a.mp4"), b"").unwrap();

    let db = VaultDb::open_in_memory().unwrap();
    let report = import_folder(&db, dir.path()).unwrap();
    let course_id = &report.course_id;
    let lesson_id = &report.lesson_ids[0];

    let order_before = learning_order(&db, course_id).unwrap();
    let progress_before = db.progress_for_lesson(lesson_id).unwrap();
    assert!(list_transcription_cues(&db, lesson_id).unwrap().is_empty());

    let err = run_lesson_transcription(&db, lesson_id, &FailingTranscriptionEngine).unwrap_err();
    assert!(matches!(err, TranscriptionError::Whisper(_)));

    assert_eq!(learning_order(&db, course_id).unwrap(), order_before);
    assert_eq!(db.progress_for_lesson(lesson_id).unwrap(), progress_before);
    assert!(list_transcription_cues(&db, lesson_id).unwrap().is_empty());
}
