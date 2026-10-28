use std::fs;
use std::path::Path;

use tempfile::tempdir;
use vault_core::anki_link::{anki_card_link, insert_anki_card_link};
use vault_core::import::import_folder;
use vault_core::lesson_note::{create_lesson_note, list_lesson_notes, LessonNote};
use vault_core::lesson_note_export::{
    course_notes_markdown, format_lesson_notes_markdown, lesson_notes_markdown,
};
use vault_core::order::reorder_lessons;
use vault_core::VaultDb;

fn sample_note(text: &str, timestamp_ms: Option<i64>) -> LessonNote {
    LessonNote {
        id: "note-id".into(),
        lesson_id: "lesson-id".into(),
        text: text.into(),
        timestamp_ms,
        created_at: "2020-01-01T00:00:00Z".into(),
        updated_at: "2020-01-01T00:00:00Z".into(),
    }
}

#[test]
fn formats_notes_with_mm_ss_when_timestamped() {
    let markdown = format_lesson_notes_markdown(&[
        sample_note("Standalone reminder", None),
        sample_note("At one minute five", Some(65_000)),
    ]);

    assert_eq!(
        markdown,
        "Standalone reminder\n\nAt one minute five (1:05)\n"
    );
}

#[test]
fn lesson_notes_markdown_uses_vault_list_order() {
    let dir = tempdir().unwrap();
    let root = dir.path();
    fs::write(root.join("lecture.mp4"), b"").unwrap();

    let db = VaultDb::open_in_memory().unwrap();
    let report = import_folder(&db, root).unwrap();
    let lesson_id = report.lesson_ids[0].clone();

    create_lesson_note(&db, &lesson_id, "First untimed", None).unwrap();
    create_lesson_note(&db, &lesson_id, "At forty-two seconds", Some(42_000)).unwrap();

    let markdown = lesson_notes_markdown(&db, &lesson_id).unwrap();
    assert_eq!(markdown, "At forty-two seconds (0:42)\n\nFirst untimed\n");
}

#[test]
fn course_notes_markdown_follows_learning_order_with_two_lessons() {
    let dir = tempdir().unwrap();
    let root = dir.path();
    fs::write(root.join("alpha.mp4"), b"").unwrap();
    fs::write(root.join("beta.mp4"), b"").unwrap();

    let db = VaultDb::open_in_memory().unwrap();
    let report = import_folder(&db, root).unwrap();
    assert_eq!(report.lesson_ids.len(), 2);

    let alpha_id = report
        .lesson_ids
        .iter()
        .find(|id| {
            db.lesson_display_title(id)
                .map(|t| t == "alpha")
                .unwrap_or(false)
        })
        .cloned()
        .expect("alpha lesson");
    let beta_id = report
        .lesson_ids
        .iter()
        .find(|id| {
            db.lesson_display_title(id)
                .map(|t| t == "beta")
                .unwrap_or(false)
        })
        .cloned()
        .expect("beta lesson");

    create_lesson_note(&db, &alpha_id, "Alpha-only note", None).unwrap();
    create_lesson_note(&db, &beta_id, "Beta at one minute", Some(60_000)).unwrap();

    reorder_lessons(&db, &report.course_id, &[beta_id.clone(), alpha_id.clone()]).unwrap();

    let markdown = course_notes_markdown(&db, &report.course_id).unwrap();
    assert_eq!(
        markdown,
        "## beta\n\nBeta at one minute (1:00)\n\n## alpha\n\nAlpha-only note\n"
    );
}

/// Markdown export reads vault `lesson_note` rows only; it must not open the Anki
/// collection or delete cards (prompt 181).
#[test]
fn markdown_export_reads_vault_notes_only_and_does_not_delete_cards() {
    let export_source = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/lesson_note_export.rs");
    let source = fs::read_to_string(&export_source).expect("read lesson_note_export.rs");
    let code_only: String = source
        .lines()
        .filter(|line| {
            let t = line.trim_start();
            !t.starts_with("//!") && !t.starts_with("//")
        })
        .collect::<Vec<_>>()
        .join("\n");
    let lower = code_only.to_ascii_lowercase();
    for forbidden in [
        "anki_bridge",
        "open_profile",
        "delete_card",
        "unlink_card",
        "collection.anki",
    ] {
        assert!(
            !lower.contains(forbidden),
            "lesson_note_export must stay out of Anki ({forbidden}); export is vault notes only"
        );
    }

    let dir = tempdir().unwrap();
    fs::write(dir.path().join("lecture.mp4"), b"").unwrap();
    let db = VaultDb::open_in_memory().unwrap();
    let report = import_folder(&db, dir.path()).unwrap();
    let lesson_id = report.lesson_ids[0].clone();

    create_lesson_note(&db, &lesson_id, "Vault-only export text", None).unwrap();
    let link_id =
        insert_anki_card_link(&db, &lesson_id, 12_000, "export-hash", "", 55_001).unwrap();

    let markdown = lesson_notes_markdown(&db, &lesson_id).unwrap();
    assert_eq!(markdown, "Vault-only export text\n");

    assert_eq!(list_lesson_notes(&db, &lesson_id).unwrap().len(), 1);
    assert_eq!(anki_card_link(&db, &link_id).unwrap().anki_note_id, 55_001);
}
