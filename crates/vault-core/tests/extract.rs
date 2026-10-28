use std::path::PathBuf;

use vault_core::extract::index_lesson_metadata;
use vault_core::extract::{
    extract_first_heading, extract_pdf_page_count, extract_subtitle_cues, ExtractorEnv,
    ExtractorOutcome,
};
use vault_core::import::import_folder;
use vault_core::VaultDb;

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(name)
}

#[test]
fn srt_cue_count_and_bounds() {
    let meta = extract_subtitle_cues(&fixture("sample.srt")).unwrap();
    assert_eq!(meta.cue_count, ExtractorOutcome::known("srt", 2));
    assert_eq!(
        meta.first_timestamp_ms,
        ExtractorOutcome::known("srt", 1_000)
    );
    assert_eq!(
        meta.last_timestamp_ms,
        ExtractorOutcome::known("srt", 12_250)
    );
}

#[test]
fn markdown_first_heading_or_unknown() {
    let heading = extract_first_heading(&fixture("readme.md")).unwrap();
    assert_eq!(
        heading,
        ExtractorOutcome::known("markdown_heading", "Course intro".to_string())
    );

    let missing = extract_first_heading(&fixture("no-such-file.md"));
    assert!(missing.is_err());
}

#[test]
fn pdf_pages_unknown_when_tool_missing() {
    let pages =
        extract_pdf_page_count(&fixture("dummy.pdf"), &ExtractorEnv::without_tools()).unwrap();
    assert_eq!(
        pages,
        ExtractorOutcome::unknown("pdfinfo", "pdfinfo not available")
    );
}

#[test]
fn video_duration_unknown_without_ffprobe() {
    let db = VaultDb::open_in_memory().unwrap();
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("clip.mp4"), b"not-a-real-video").unwrap();
    let report = import_folder(&db, dir.path()).unwrap();
    let lesson_id = &report.lesson_ids[0];
    let source = dir.path().join("clip.mp4");

    index_lesson_metadata(
        &db,
        lesson_id,
        &source,
        "mp4",
        None,
        &ExtractorEnv::without_tools(),
    )
    .unwrap();

    let view = db.lesson_metadata(lesson_id).unwrap();
    assert_eq!(
        view.duration_ms,
        ExtractorOutcome::unknown("ffprobe", "ffprobe not available")
    );
}
