use vault_core::playlist::{
    import_playlist_from_pasted_lines, parse_playlist_lines, PlaylistError,
};
use vault_core::VaultDb;

#[test]
fn parse_playlist_lines_accepts_two_tab_separated_lines_and_skips_blanks() {
    let text = "Alpha\thttps://example.com/a\n\nBeta\thttps://example.com/b\n";
    let lines = parse_playlist_lines(text).unwrap();
    assert_eq!(lines.len(), 2);
    assert_eq!(lines[0].title, "Alpha");
    assert_eq!(lines[0].url, "https://example.com/a");
    assert_eq!(lines[1].title, "Beta");
    assert_eq!(lines[1].url, "https://example.com/b");
}

#[test]
fn parse_playlist_lines_rejects_line_without_tab_with_line_number() {
    let text = "One\thttps://one.example\nTwo\thttps://two.example\nnot a valid line\n";
    let err = parse_playlist_lines(text).unwrap_err();
    assert!(
        matches!(err, PlaylistError::InvalidLine { line: 3, .. }),
        "unexpected error: {err}"
    );
}

#[test]
fn import_playlist_from_pasted_lines_creates_playlist_course_without_http() {
    let db = VaultDb::open_in_memory().unwrap();
    let definition_path = "auravault://pasted/test-course";
    let text = "First lesson\thttps://lesson.one\nSecond lesson\thttps://lesson.two\n";
    let report =
        import_playlist_from_pasted_lines(&db, definition_path, "My pasted course", text).unwrap();

    assert_eq!(
        db.course_source_kind(&report.course_id).unwrap(),
        "playlist"
    );
    assert_eq!(db.lesson_count_for_course(&report.course_id).unwrap(), 2);
    assert_eq!(
        db.course_title(&report.course_id).unwrap(),
        "My pasted course"
    );
}
