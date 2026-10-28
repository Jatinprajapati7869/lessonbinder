use std::fs;
use std::path::PathBuf;

use tempfile::tempdir;
use vault_core::import::import_folder;
use vault_core::lesson_note::create_lesson_note;
use vault_core::search::{lesson_note_body, search, SearchMatchKind};
use vault_core::VaultDb;

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(name)
}

#[test]
fn empty_query_returns_no_rows() {
    let db = VaultDb::open_in_memory().unwrap();
    assert!(search(&db, "").unwrap().is_empty());
    assert!(search(&db, "   ").unwrap().is_empty());
}

#[test]
fn matches_lesson_title_case_insensitive() {
    let dir = tempdir().unwrap();
    let root = dir.path();
    fs::write(root.join("Galaxy_Nebula_Lecture.mp4"), b"").unwrap();

    let db = VaultDb::open_in_memory().unwrap();
    let report = import_folder(&db, root).unwrap();
    let lesson_id = report.lesson_ids[0].clone();

    let hits = search(&db, "nebula").unwrap();
    let title_hits = hits
        .iter()
        .filter(|hit| hit.match_kind == SearchMatchKind::LessonTitle)
        .collect::<Vec<_>>();
    assert_eq!(title_hits.len(), 1);
    assert_eq!(title_hits[0].lesson_id, lesson_id);
    assert_eq!(title_hits[0].course_id, report.course_id);
    assert!(title_hits[0].snippet.to_lowercase().contains("nebula"));
}

#[test]
fn matches_subtitle_cue_text() {
    let dir = tempdir().unwrap();
    let root = dir.path();
    fs::write(root.join("lesson.mp4"), b"").unwrap();
    let mut srt = fs::read_to_string(fixture("three_cues.srt")).unwrap();
    srt = srt.replace("Second line", "Quasar emission line");
    fs::write(root.join("lesson.srt"), srt).unwrap();

    let db = VaultDb::open_in_memory().unwrap();
    let report = import_folder(&db, root).unwrap();
    let lesson_id = report.lesson_ids[0].clone();

    let hits = search(&db, "QUASAR").unwrap();
    let cue_hits = hits
        .iter()
        .filter(|hit| hit.match_kind == SearchMatchKind::SubtitleCue)
        .collect::<Vec<_>>();
    assert_eq!(cue_hits.len(), 1);
    assert_eq!(cue_hits[0].lesson_id, lesson_id);
    assert_eq!(cue_hits[0].course_id, report.course_id);
    assert!(cue_hits[0].snippet.to_lowercase().contains("quasar"));
    assert_eq!(cue_hits[0].cue_start_ms, Some(3_000));
}

#[test]
fn matches_lesson_note_text() {
    let dir = tempdir().unwrap();
    let root = dir.path();
    fs::write(root.join("lecture.mp4"), b"").unwrap();

    let db = VaultDb::open_in_memory().unwrap();
    let report = import_folder(&db, root).unwrap();
    let lesson_id = report.lesson_ids[0].clone();

    create_lesson_note(
        &db,
        &lesson_id,
        "Orion belt mnemonic for exam prep",
        Some(9_000),
    )
    .unwrap();

    let hits = search(&db, "mnemonic").unwrap();
    let note_hits = hits
        .iter()
        .filter(|hit| hit.match_kind == SearchMatchKind::LessonNote)
        .collect::<Vec<_>>();
    assert_eq!(note_hits.len(), 1);
    assert_eq!(note_hits[0].lesson_id, lesson_id);
    assert_eq!(note_hits[0].course_id, report.course_id);
    assert!(note_hits[0].snippet.to_lowercase().contains("mnemonic"));
    assert_eq!(note_hits[0].cue_start_ms, Some(9_000));
}

#[test]
fn untimed_lesson_note_hit_has_no_seek_timestamp() {
    let dir = tempdir().unwrap();
    let root = dir.path();
    fs::write(root.join("lecture.mp4"), b"").unwrap();

    let db = VaultDb::open_in_memory().unwrap();
    let report = import_folder(&db, root).unwrap();
    let lesson_id = report.lesson_ids[0].clone();

    create_lesson_note(&db, &lesson_id, "Standalone reminder text", None).unwrap();

    let hits = search(&db, "reminder").unwrap();
    let note_hits = hits
        .iter()
        .filter(|hit| hit.match_kind == SearchMatchKind::LessonNote)
        .collect::<Vec<_>>();
    assert_eq!(note_hits.len(), 1);
    assert_eq!(note_hits[0].lesson_id, lesson_id);
    assert_eq!(note_hits[0].cue_start_ms, None);
}

/// Lesson-note search loads bodies from `vault.db` only (prompt 182), same seam as [`search`].
#[test]
fn search_stays_local_lesson_note_body_returned_and_missing_note_not() {
    let dir = tempdir().unwrap();
    let root = dir.path();
    fs::write(root.join("lecture.mp4"), b"").unwrap();

    let db = VaultDb::open_in_memory().unwrap();
    let report = import_folder(&db, root).unwrap();
    let lesson_id = report.lesson_ids[0].clone();

    let note = create_lesson_note(
        &db,
        &lesson_id,
        "Full vault note body for search",
        Some(1_000),
    )
    .unwrap();

    assert_eq!(
        lesson_note_body(&db, &note.id).unwrap().as_deref(),
        Some("Full vault note body for search")
    );
    assert_eq!(lesson_note_body(&db, "missing-note-id").unwrap(), None);
}
