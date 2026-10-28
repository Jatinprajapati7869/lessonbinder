use anki::prelude::NoteId;
use anki_bridge::{
    audio_filename_for_bytes, commit_card_draft, get_note, open_profile, LessonbinderProfilePaths,
};
use tempfile::tempdir;
use vault_core::audio_clip::encode_lesson_audio_clip;
use vault_core::card::card_draft_from_audio_clip;
use vault_core::clip_encoder::FakeClipEncoder;
use vault_core::VaultDb;

fn fixture_lesson(db: &VaultDb, dir: &tempfile::TempDir) -> String {
    let course_id = db
        .insert_course("Linear Algebra", dir.path().to_str().unwrap())
        .unwrap();
    std::fs::write(dir.path().join("lecture.mp4"), b"video").unwrap();
    db.insert_local_file_lesson(&course_id, "lecture.mp4", "Intro clip", 0)
        .unwrap()
}

#[test]
fn commit_audio_clip_stores_mp3_reference_in_screenshot_field() {
    let profile = tempdir().unwrap();
    let mut col = open_profile(profile.path()).unwrap();
    let paths = LessonbinderProfilePaths::in_dir(profile.path());

    let db = VaultDb::open_in_memory().unwrap();
    let lesson_id = fixture_lesson(&db, &profile);
    let draft = card_draft_from_audio_clip(&db, &lesson_id, 12_000, Some("Spoken line")).unwrap();
    let mp3 = b"FAKEMP3BYTES";
    let encoder = FakeClipEncoder::new(mp3.to_vec());
    let encoded = encode_lesson_audio_clip(&db, &encoder, &lesson_id, 10_000, 12_000).unwrap();

    let note_id = commit_card_draft(
        col.collection_mut(),
        &paths.media_folder,
        &db,
        &lesson_id,
        12_000,
        &draft,
        &[],
        Some(&encoded),
    )
    .unwrap();

    let filename = audio_filename_for_bytes(&encoded);
    assert!(paths.media_folder.join(&filename).is_file());

    let stored = get_note(col.collection_mut(), NoteId(note_id)).unwrap();
    assert_eq!(stored.screenshot, format!("[sound:{filename}]"));
    assert_eq!(stored.front, "Spoken line");
}
