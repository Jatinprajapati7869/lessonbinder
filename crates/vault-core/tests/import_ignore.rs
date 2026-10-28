use std::fs;

use tempfile::tempdir;
use vault_core::import::import_folder;
use vault_core::lessonbinderignore::parse_patterns;
use vault_core::VaultDb;

#[test]
fn parse_lessonbinderignore_skips_comments_and_blanks() {
    let patterns = parse_patterns(
        "\
# secrets stay local
secret/*.mp4

extras/
",
    );
    assert_eq!(
        patterns,
        vec!["secret/*.mp4".to_string(), "extras/".to_string()]
    );
}

#[test]
fn import_skips_secret_mp4_and_keeps_lecture() {
    let dir = tempdir().unwrap();
    let root = dir.path();
    fs::write(
        root.join(".lessonbinderignore"),
        "# do not import secrets\nsecret/*.mp4\n",
    )
    .unwrap();
    fs::write(root.join("lecture.mp4"), b"").unwrap();
    fs::create_dir(root.join("secret")).unwrap();
    fs::write(root.join("secret/leak.mp4"), b"").unwrap();

    let db = VaultDb::open_in_memory().unwrap();
    let report = import_folder(&db, root).unwrap();

    assert_eq!(db.lesson_count_for_course(&report.course_id).unwrap(), 1);
    assert_eq!(
        db.lesson_stable_keys_for_course(&report.course_id).unwrap(),
        vec!["lecture.mp4"]
    );
}

#[test]
fn rescan_honors_lessonbinderignore_for_new_files() {
    let dir = tempdir().unwrap();
    let root = dir.path();
    fs::write(root.join(".lessonbinderignore"), "secret/*.mp4\n").unwrap();
    fs::write(root.join("lecture.mp4"), b"").unwrap();

    let db = VaultDb::open_in_memory().unwrap();
    let first = import_folder(&db, root).unwrap();
    fs::create_dir(root.join("secret")).unwrap();
    fs::write(root.join("secret/new.mp4"), b"").unwrap();

    let second = import_folder(&db, root).unwrap();

    assert_eq!(first.course_id, second.course_id);
    assert_eq!(db.lesson_count_for_course(&first.course_id).unwrap(), 1);
    assert_eq!(
        db.lesson_stable_keys_for_course(&first.course_id).unwrap(),
        vec!["lecture.mp4"]
    );
}
