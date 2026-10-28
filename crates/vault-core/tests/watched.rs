use vault_core::extract::{ExtractorOutcome, SubtitleExtract};
use vault_core::watched::{
    union_watched_ranges, watched_ms, watched_percent, PlaybackRangeError, WatchedRange,
};
use vault_core::LessonStatus;
use vault_core::{record_playback, VaultDb};

fn empty_subtitle() -> SubtitleExtract {
    SubtitleExtract {
        cue_count: ExtractorOutcome::unknown("test", "none"),
        first_timestamp_ms: ExtractorOutcome::unknown("test", "none"),
        last_timestamp_ms: ExtractorOutcome::unknown("test", "none"),
    }
}

fn set_duration(db: &VaultDb, lesson_id: &str, ms: i64) {
    db.save_lesson_metadata(
        lesson_id,
        &ExtractorOutcome::known("test", ms),
        &ExtractorOutcome::unknown("test", "n/a"),
        &ExtractorOutcome::unknown("test", "n/a"),
        &empty_subtitle(),
    )
    .unwrap();
}

#[test]
fn union_merges_overlapping_ranges() {
    let first = WatchedRange {
        start_ms: 0,
        end_ms: 5_000,
    };
    let second = WatchedRange {
        start_ms: 3_000,
        end_ms: 8_000,
    };
    let merged = union_watched_ranges(&[first], second).unwrap();
    assert_eq!(
        merged,
        vec![WatchedRange {
            start_ms: 0,
            end_ms: 8_000,
        }]
    );
    assert_eq!(watched_ms(&merged), 8_000);
}

#[test]
fn union_merges_adjacent_ranges() {
    let first = WatchedRange {
        start_ms: 0,
        end_ms: 5_000,
    };
    let second = WatchedRange {
        start_ms: 5_000,
        end_ms: 10_000,
    };
    let merged = union_watched_ranges(&[first], second).unwrap();
    assert_eq!(
        merged,
        vec![WatchedRange {
            start_ms: 0,
            end_ms: 10_000,
        }]
    );
}

#[test]
fn union_keeps_gap_as_two_ranges() {
    let first = WatchedRange {
        start_ms: 0,
        end_ms: 1_000,
    };
    let second = WatchedRange {
        start_ms: 2_000,
        end_ms: 3_000,
    };
    let merged = union_watched_ranges(&[first], second).unwrap();
    assert_eq!(
        merged,
        vec![
            WatchedRange {
                start_ms: 0,
                end_ms: 1_000,
            },
            WatchedRange {
                start_ms: 2_000,
                end_ms: 3_000,
            },
        ]
    );
    assert_eq!(watched_ms(&merged), 2_000);
}

#[test]
fn watched_percent_from_union_length() {
    assert_eq!(watched_percent(30_000, Some(60_000)), Some(50));
    assert_eq!(watched_percent(10_000, None), None);
    assert_eq!(watched_percent(10_000, Some(0)), None);
}

fn lesson(db: &VaultDb) -> String {
    let course_id = db.insert_course("Course", "/root").unwrap();
    db.insert_local_file_lesson(&course_id, "a.mp4", "Lesson", 0)
        .unwrap()
}

#[test]
fn record_playback_persists_merged_ranges() {
    let db = VaultDb::open_in_memory().unwrap();
    let lesson_id = lesson(&db);

    record_playback(&db, &lesson_id, 0, 5_000).unwrap();
    record_playback(&db, &lesson_id, 4_000, 9_000).unwrap();

    let ranges = db.watched_ranges_for_lesson(&lesson_id).unwrap();
    assert_eq!(
        ranges,
        vec![WatchedRange {
            start_ms: 0,
            end_ms: 9_000,
        }]
    );
    assert_eq!(db.watched_ms_for_lesson(&lesson_id).unwrap(), 9_000);
}

#[test]
fn union_full_watch_does_not_auto_done() {
    let db = VaultDb::open_in_memory().unwrap();
    let lesson_id = lesson(&db);
    set_duration(&db, &lesson_id, 60_000);

    assert_eq!(
        db.progress_for_lesson(&lesson_id).unwrap().status,
        LessonStatus::NotStarted
    );

    record_playback(&db, &lesson_id, 0, 60_000).unwrap();

    assert_eq!(
        db.progress_for_lesson(&lesson_id).unwrap().status,
        LessonStatus::NotStarted
    );
    assert_eq!(
        db.watched_percent_for_lesson(&lesson_id).unwrap(),
        Some(100)
    );
}

#[test]
fn record_playback_does_not_mark_done() {
    let db = VaultDb::open_in_memory().unwrap();
    let lesson_id = lesson(&db);
    db.mark_complete(&lesson_id).unwrap();

    record_playback(&db, &lesson_id, 0, 60_000).unwrap();

    assert_eq!(
        db.progress_for_lesson(&lesson_id).unwrap().status,
        LessonStatus::Done
    );
}

#[test]
fn rejects_invalid_playback_range() {
    let db = VaultDb::open_in_memory().unwrap();
    let lesson_id = lesson(&db);
    let err = record_playback(&db, &lesson_id, 5_000, 5_000).unwrap_err();
    assert!(matches!(
        err,
        vault_core::ProgressPersistenceError::InvalidPlaybackRange
    ));
    assert_eq!(db.watched_ranges_for_lesson(&lesson_id).unwrap(), vec![]);
}

#[test]
fn union_rejects_invalid_range() {
    let err = union_watched_ranges(
        &[],
        WatchedRange {
            start_ms: 10,
            end_ms: 5,
        },
    )
    .unwrap_err();
    assert_eq!(err, PlaybackRangeError::InvalidRange);
}
