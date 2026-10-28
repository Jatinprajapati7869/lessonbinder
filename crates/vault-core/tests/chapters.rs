use std::fs;
use std::path::PathBuf;

use vault_core::chapters::{
    parse_ffprobe_chapters_json, refresh_lesson_chapters, ChapterIndexOutcome, MediaChapter,
};
use vault_core::extract::{index_lesson_metadata, ExtractorEnv};
use vault_core::VaultDb;

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(name)
}

#[test]
fn parse_fixture_with_two_titled_chapters() {
    let json = fs::read_to_string(fixture("ffprobe_chapters_two.json")).unwrap();
    match parse_ffprobe_chapters_json(&json) {
        ChapterIndexOutcome::Chapters(chapters) => {
            assert_eq!(
                chapters,
                vec![
                    MediaChapter {
                        start_ms: 0,
                        title: "Opening".into(),
                    },
                    MediaChapter {
                        start_ms: 125_500,
                        title: "Main topic".into(),
                    },
                ]
            );
        }
        other => panic!("expected chapters, got {other:?}"),
    }
}

#[test]
fn parse_empty_chapter_list_records_reason() {
    let json = fs::read_to_string(fixture("ffprobe_chapters_empty.json")).unwrap();
    assert_eq!(
        parse_ffprobe_chapters_json(&json),
        ChapterIndexOutcome::Empty {
            reason: "no chapters in file".into(),
        }
    );
}

#[test]
fn list_chapters_after_index_without_ffprobe() {
    let db = VaultDb::open_in_memory().unwrap();
    let dir = tempfile::tempdir().unwrap();
    let course_id = db
        .insert_course("Course", dir.path().to_str().unwrap())
        .unwrap();
    std::fs::write(dir.path().join("lecture.mp4"), b"video").unwrap();
    let lesson_id = db
        .insert_local_file_lesson(&course_id, "lecture.mp4", "Lecture", 0)
        .unwrap();

    index_lesson_metadata(
        &db,
        &lesson_id,
        &dir.path().join("lecture.mp4"),
        "mp4",
        None,
        &ExtractorEnv::without_tools(),
    )
    .unwrap();

    let listed = vault_core::list_chapters(&db, &lesson_id).unwrap();
    assert!(listed.chapters.is_empty());
    assert_eq!(listed.reason.as_deref(), Some("ffprobe not available"));
}

#[test]
fn refresh_stores_parsed_chapters_from_fixture_json() {
    let db = VaultDb::open_in_memory().unwrap();
    let dir = tempfile::tempdir().unwrap();
    let course_id = db
        .insert_course("Course", dir.path().to_str().unwrap())
        .unwrap();
    std::fs::write(dir.path().join("lecture.mp4"), b"video").unwrap();
    let lesson_id = db
        .insert_local_file_lesson(&course_id, "lecture.mp4", "Lecture", 0)
        .unwrap();

    let json = fs::read_to_string(fixture("ffprobe_chapters_two.json")).unwrap();
    let outcome = parse_ffprobe_chapters_json(&json);
    db.save_lesson_chapters(&lesson_id, &outcome).unwrap();

    let listed = vault_core::list_chapters(&db, &lesson_id).unwrap();
    assert_eq!(listed.chapters.len(), 2);
    assert_eq!(listed.chapters[0].title, "Opening");
    assert!(listed.reason.is_none());
}

#[test]
fn refresh_without_local_file_is_unavailable_not_invented() {
    let db = VaultDb::open_in_memory().unwrap();
    let report = vault_core::playlist::import_playlist_definition(
        &db,
        &fixture("playlists/three_items.json"),
    )
    .unwrap();
    let lesson_id = db
        .lesson_id_for_stable_key(&report.course_id, "item-alpha")
        .unwrap();

    refresh_lesson_chapters(&db, &lesson_id, &ExtractorEnv::without_tools()).unwrap();

    let listed = vault_core::list_chapters(&db, &lesson_id).unwrap();
    assert!(listed.chapters.is_empty());
    assert_eq!(listed.reason.as_deref(), Some("no local media file"));
}
