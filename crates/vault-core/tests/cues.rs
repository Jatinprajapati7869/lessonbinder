use std::fs;
use std::path::PathBuf;

use tempfile::tempdir;
use vault_core::card::lesson_exists;
use vault_core::cues::{list_cues, list_player_cues, CueError, SubtitleCue};
use vault_core::import::import_folder;
use vault_core::transcription::run_lesson_transcription;
use vault_core::transcription::FakeTranscriptionEngine;
use vault_core::VaultDb;

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(name)
}

#[test]
fn list_cues_returns_three_ordered_cues_from_fixture_srt() {
    let dir = tempdir().unwrap();
    let root = dir.path();
    fs::write(root.join("lesson.mp4"), b"").unwrap();
    fs::copy(fixture("three_cues.srt"), root.join("lesson.srt")).unwrap();

    let db = VaultDb::open_in_memory().unwrap();
    let report = import_folder(&db, root).unwrap();
    let lesson_id = &report.lesson_ids[0];

    let cues = list_cues(&db, lesson_id).unwrap();
    assert_eq!(
        cues,
        vec![
            SubtitleCue {
                start_ms: 1_000,
                end_ms: 2_000,
                text: "First".into(),
            },
            SubtitleCue {
                start_ms: 3_000,
                end_ms: 4_500,
                text: "Second line".into(),
            },
            SubtitleCue {
                start_ms: 10_000,
                end_ms: 12_000,
                text: "Third".into(),
            },
        ]
    );
}

#[test]
fn list_cues_empty_when_lesson_has_no_subtitle_file() {
    let dir = tempdir().unwrap();
    let root = dir.path();
    fs::write(root.join("solo.mp4"), b"").unwrap();

    let db = VaultDb::open_in_memory().unwrap();
    let report = import_folder(&db, root).unwrap();

    let cues = list_cues(&db, &report.lesson_ids[0]).unwrap();
    assert!(cues.is_empty());
}

#[test]
fn list_cues_empty_when_subtitle_file_missing_on_disk() {
    let dir = tempdir().unwrap();
    let root = dir.path();
    fs::write(root.join("clip.mp4"), b"").unwrap();
    fs::write(
        root.join("clip.srt"),
        b"1\n00:00:01,000 --> 00:00:02,000\nHi\n",
    )
    .unwrap();

    let db = VaultDb::open_in_memory().unwrap();
    let report = import_folder(&db, root).unwrap();
    let lesson_id = &report.lesson_ids[0];
    fs::remove_file(root.join("clip.srt")).unwrap();

    let cues = list_cues(&db, lesson_id).unwrap();
    assert!(cues.is_empty());
}

#[test]
fn list_cues_malformed_subtitle_is_typed_error_and_lesson_remains() {
    let dir = tempdir().unwrap();
    let root = dir.path();
    fs::write(root.join("talk.mp4"), b"").unwrap();
    fs::write(root.join("talk.srt"), b"not a valid subtitle\n{{{\n").unwrap();

    let db = VaultDb::open_in_memory().unwrap();
    let report = import_folder(&db, root).unwrap();
    let lesson_id = &report.lesson_ids[0];
    let course_id = &report.course_id;

    let err = list_cues(&db, lesson_id).unwrap_err();
    assert!(matches!(err, CueError::Parse(_)));

    assert_eq!(db.lesson_count_for_course(course_id).unwrap(), 1);
    assert!(lesson_exists(&db, lesson_id).unwrap());
}

#[test]
fn list_player_cues_keeps_sidecar_and_adds_transcription() {
    let dir = tempdir().unwrap();
    let root = dir.path();
    fs::write(root.join("lesson.mp4"), b"").unwrap();
    fs::copy(fixture("three_cues.srt"), root.join("lesson.srt")).unwrap();

    let db = VaultDb::open_in_memory().unwrap();
    let report = import_folder(&db, root).unwrap();
    let lesson_id = &report.lesson_ids[0];

    let sidecar = list_cues(&db, lesson_id).unwrap();
    assert_eq!(sidecar.len(), 3);

    let generated = vec![SubtitleCue {
        start_ms: 500,
        end_ms: 900,
        text: "Generated".into(),
    }];
    run_lesson_transcription(
        &db,
        lesson_id,
        &FakeTranscriptionEngine::new(generated.clone()),
    )
    .unwrap();

    let merged = list_player_cues(&db, lesson_id).unwrap();
    assert_eq!(merged.len(), 4);
    assert_eq!(list_cues(&db, lesson_id).unwrap(), sidecar);
    assert_eq!(merged[0].text, "Generated");
    assert_eq!(merged[1].text, "First");
}
