use std::path::PathBuf;

use vault_core::audio_clip::{
    audio_clip_range_ms, encode_lesson_audio_clip, ensure_local_clip_source,
    PLAYHEAD_FALLBACK_CLIP_MS,
};
use vault_core::clip_encoder::{
    validate_clip_range, ClipEncoderError, FakeClipEncoder, MAX_CLIP_DURATION_MS,
};
use vault_core::cues::SubtitleCue;
use vault_core::VaultDb;

#[test]
fn active_cue_range_when_playhead_inside_cue() {
    let cues = vec![SubtitleCue {
        start_ms: 10_000,
        end_ms: 25_000,
        text: "Hello".into(),
    }];
    assert_eq!(audio_clip_range_ms(&cues, 12_000), (10_000, 25_000));
}

#[test]
fn fallback_last_fifteen_seconds_without_active_cue() {
    let cues = vec![SubtitleCue {
        start_ms: 0,
        end_ms: 5_000,
        text: "Early".into(),
    }];
    assert_eq!(
        audio_clip_range_ms(&cues, 40_000),
        (40_000 - PLAYHEAD_FALLBACK_CLIP_MS, 40_000),
    );
}

#[test]
fn playlist_lesson_has_no_local_clip_source() {
    let db = VaultDb::open_in_memory().unwrap();
    let fixture =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/playlists/three_items.json");
    let report = vault_core::playlist::import_playlist_definition(&db, &fixture).unwrap();
    let lesson_id = db
        .lesson_id_for_stable_key(&report.course_id, "item-alpha")
        .unwrap();
    let err = ensure_local_clip_source(&db, &lesson_id).unwrap_err();
    assert_eq!(err, ClipEncoderError::ClipEncoderUnavailable);
}

#[test]
fn long_active_cue_fails_validation() {
    let cues = vec![SubtitleCue {
        start_ms: 0,
        end_ms: MAX_CLIP_DURATION_MS + 1,
        text: "Too long".into(),
    }];
    let (start_ms, end_ms) = audio_clip_range_ms(&cues, 1_000);
    assert_eq!(
        validate_clip_range(start_ms, end_ms).unwrap_err(),
        ClipEncoderError::DurationTooLong
    );
}

#[test]
fn encode_uses_fake_bytes_for_local_file() {
    let db = VaultDb::open_in_memory().unwrap();
    let dir = tempfile::tempdir().unwrap();
    let course_id = db
        .insert_course("Course", dir.path().to_str().unwrap())
        .unwrap();
    std::fs::write(dir.path().join("lecture.mp4"), b"video").unwrap();
    let lesson_id = db
        .insert_local_file_lesson(&course_id, "lecture.mp4", "Lecture", 0)
        .unwrap();
    let encoder = FakeClipEncoder::new(vec![0x49, 0x44, 0x33]);
    let bytes = encode_lesson_audio_clip(&db, &encoder, &lesson_id, 1_000, 2_000).unwrap();
    assert_eq!(bytes, vec![0x49, 0x44, 0x33]);
}
