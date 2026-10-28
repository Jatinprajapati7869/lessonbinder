use std::path::Path;

use tempfile::tempdir;
use vault_core::clip_encoder::{
    ClipEncoder, ClipEncoderError, FakeClipEncoder, FfmpegClipEncoder, MAX_CLIP_DURATION_MS,
};

#[test]
fn fake_encoder_returns_configured_bytes() {
    let dir = tempdir().unwrap();
    let media = dir.path().join("lesson.mp4");
    std::fs::write(&media, b"not-real-video").unwrap();

    let encoder = FakeClipEncoder::new(b"FAKE-AUDIO".to_vec());
    let bytes = encoder.encode_clip(&media, 1_000, 4_000).unwrap();
    assert_eq!(bytes, b"FAKE-AUDIO");
}

#[test]
fn refuses_end_before_or_equal_to_start() {
    let encoder = FakeClipEncoder::new(vec![1]);
    let path = Path::new("/unused");

    let err = encoder.encode_clip(path, 5_000, 5_000).unwrap_err();
    assert_eq!(err, ClipEncoderError::InvalidRange);

    let err = encoder.encode_clip(path, 5_000, 4_999).unwrap_err();
    assert_eq!(err, ClipEncoderError::InvalidRange);
}

#[test]
fn refuses_clip_longer_than_sixty_seconds() {
    let encoder = FakeClipEncoder::new(vec![1]);
    let path = Path::new("/unused");

    let err = encoder
        .encode_clip(path, 0, MAX_CLIP_DURATION_MS + 1)
        .unwrap_err();
    assert_eq!(err, ClipEncoderError::DurationTooLong);

    assert!(encoder.encode_clip(path, 0, MAX_CLIP_DURATION_MS).is_ok());
}

#[test]
fn ffmpeg_encoder_reports_unavailable_when_not_configured() {
    let encoder = FfmpegClipEncoder::without_ffmpeg();
    let dir = tempdir().unwrap();
    let media = dir.path().join("lesson.mp4");
    std::fs::write(&media, b"x").unwrap();

    let err = encoder.encode_clip(&media, 0, 1_000).unwrap_err();
    assert_eq!(err, ClipEncoderError::ClipEncoderUnavailable);
}
