use std::fs;
use std::path::PathBuf;

use tempfile::tempdir;
use vault_core::import::import_folder;
use vault_core::search::{search, SearchMatchKind};
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
