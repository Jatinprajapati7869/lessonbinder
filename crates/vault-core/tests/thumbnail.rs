use std::fs;
use std::path::Path;

use tempfile::tempdir;
use vault_core::import::import_folder;
use vault_core::playlist::import_playlist_definition;
use vault_core::thumbnail::{capture_thumbnail, ThumbnailCapturer, ThumbnailError};
use vault_core::VaultDb;

const FAKE_JPEG: &[u8] = &[0xff, 0xd8, 0xff, 0xdb, 0x00, 0x43, 0x00];

struct FakeCapturer {
    bytes: Vec<u8>,
}

impl ThumbnailCapturer for FakeCapturer {
    fn capture_jpeg(
        &self,
        _db: &VaultDb,
        _lesson_id: &str,
        _position_ms: i64,
    ) -> Result<Vec<u8>, ThumbnailError> {
        Ok(self.bytes.clone())
    }
}

#[test]
fn capture_thumbnail_writes_jpeg_under_app_data_and_stores_path() {
    let dir = tempdir().unwrap();
    let course_dir = dir.path().join("course");
    fs::create_dir(&course_dir).unwrap();
    fs::write(course_dir.join("lecture.mp4"), b"").unwrap();

    let db = VaultDb::open_in_memory().unwrap();
    let report = import_folder(&db, &course_dir).unwrap();
    let lesson_id = report.lesson_ids[0].clone();

    let thumbnails_dir = dir.path().join("thumbnails");
    let stored = capture_thumbnail(
        &db,
        &thumbnails_dir,
        &lesson_id,
        12_500,
        &FakeCapturer {
            bytes: FAKE_JPEG.to_vec(),
        },
    )
    .unwrap();

    let expected_file = thumbnails_dir.join(format!("{lesson_id}.jpg"));
    assert_eq!(stored, expected_file);
    assert!(expected_file.is_file());
    assert_eq!(fs::read(&expected_file).unwrap(), FAKE_JPEG);

    assert_eq!(
        db.lesson_thumbnail_path(&lesson_id).unwrap(),
        Some(expected_file.to_string_lossy().into_owned()),
    );
    assert_eq!(db.anki_card_link_count_for_lesson(&lesson_id).unwrap(), 0);
}

#[test]
fn capture_thumbnail_refuses_playlist_lesson_without_local_file() {
    let db = VaultDb::open_in_memory().unwrap();
    let path =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/playlists/three_items.json");
    let report = import_playlist_definition(&db, &path).unwrap();
    let lesson_id = db
        .lesson_id_for_stable_key(&report.course_id, "item-alpha")
        .unwrap();

    let dir = tempdir().unwrap();
    let err = capture_thumbnail(
        &db,
        dir.path(),
        &lesson_id,
        1_000,
        &FakeCapturer {
            bytes: FAKE_JPEG.to_vec(),
        },
    )
    .unwrap_err();

    assert!(matches!(err, ThumbnailError::RequiresLocalFile));
}
