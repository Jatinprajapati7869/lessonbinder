use anki_bridge::{add_clip_note, note_study_text, open_profile, ClipDraft};

#[test]
fn strips_script_handlers_and_lists_local_media() {
    let dir = tempfile::tempdir().unwrap();
    let mut col = open_profile(dir.path()).unwrap();
    let col = col.collection_mut();

    let note_id = add_clip_note(
        col,
        ClipDraft {
            course: "Course".into(),
            lesson: "Lesson".into(),
            front: r#"<script>alert(1)</script>Hi [sound:beep.mp3]<img src="shot.png" onclick="evil()">"#
                .into(),
            back: "Answer".into(),
            source: "file:///a.mp4".into(),
            timestamp: "00:00:01".into(),
            screenshot: "clip.png".into(),
        },
    )
    .unwrap();

    let content = note_study_text(col, note_id).unwrap();

    let front_lower = content.front.to_lowercase();
    assert!(!front_lower.contains("<script"));
    assert!(!front_lower.contains("onclick"));
    assert!(content.front.contains("Hi"));
    assert!(content.media_filenames.iter().any(|f| f == "beep.mp3"));
    assert!(content.media_filenames.iter().any(|f| f == "shot.png"));
    assert!(!content
        .media_filenames
        .iter()
        .any(|f| f.starts_with("http://") || f.starts_with("https://")));
    assert_eq!(content.back, "Answer");
}
