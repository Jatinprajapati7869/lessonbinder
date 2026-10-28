use vault_core::progress::{
    apply_mark_complete, apply_mark_incomplete, apply_playback_starts, apply_set_resume,
    apply_source_missing, apply_source_restored, LessonProgressState, ProgressError,
};
use vault_core::LessonStatus;

fn fresh() -> LessonProgressState {
    LessonProgressState::default()
}

#[test]
fn legal_transitions() {
    let mut p = fresh();
    apply_playback_starts(&mut p).unwrap();
    assert_eq!(p.status, LessonStatus::InProgress);

    let mut p = fresh();
    apply_set_resume(&mut p, 1_000).unwrap();
    assert_eq!(p.status, LessonStatus::InProgress);
    assert_eq!(p.resume_position_ms, 1_000);

    let mut p = LessonProgressState {
        status: LessonStatus::InProgress,
        resume_position_ms: 500,
        max_position_reached_ms: 500,
        document_position: 0,
    };
    apply_mark_complete(&mut p).unwrap();
    assert_eq!(p.status, LessonStatus::Done);

    let mut p = LessonProgressState {
        status: LessonStatus::NotStarted,
        resume_position_ms: 0,
        max_position_reached_ms: 0,
        document_position: 0,
    };
    apply_mark_complete(&mut p).unwrap();
    assert_eq!(p.status, LessonStatus::Done);

    let mut p = LessonProgressState {
        status: LessonStatus::Done,
        resume_position_ms: 2_000,
        max_position_reached_ms: 2_000,
        document_position: 0,
    };
    apply_mark_incomplete(&mut p).unwrap();
    assert_eq!(p.status, LessonStatus::InProgress);

    let mut p = LessonProgressState {
        status: LessonStatus::Done,
        resume_position_ms: 0,
        max_position_reached_ms: 0,
        document_position: 0,
    };
    apply_mark_incomplete(&mut p).unwrap();
    assert_eq!(p.status, LessonStatus::NotStarted);

    let mut p = LessonProgressState {
        status: LessonStatus::InProgress,
        resume_position_ms: 100,
        max_position_reached_ms: 100,
        document_position: 0,
    };
    apply_source_missing(&mut p).unwrap();
    assert_eq!(p.status, LessonStatus::Broken);

    let mut p = LessonProgressState {
        status: LessonStatus::Broken,
        resume_position_ms: 0,
        max_position_reached_ms: 0,
        document_position: 0,
    };
    apply_source_restored(&mut p).unwrap();
    assert_eq!(p.status, LessonStatus::NotStarted);
}

#[test]
fn illegal_transitions_rejected() {
    let mut p = fresh();
    assert!(matches!(
        apply_mark_incomplete(&mut p),
        Err(ProgressError::IllegalTransition { .. })
    ));

    let mut p = LessonProgressState {
        status: LessonStatus::Broken,
        resume_position_ms: 0,
        max_position_reached_ms: 0,
        document_position: 0,
    };
    assert!(matches!(
        apply_mark_complete(&mut p),
        Err(ProgressError::IllegalTransition { .. })
    ));
    assert!(matches!(
        apply_playback_starts(&mut p),
        Err(ProgressError::IllegalTransition { .. })
    ));
    assert!(matches!(
        apply_mark_incomplete(&mut p),
        Err(ProgressError::IllegalTransition { .. })
    ));

    let mut p = LessonProgressState {
        status: LessonStatus::Done,
        resume_position_ms: 0,
        max_position_reached_ms: 0,
        document_position: 0,
    };
    assert!(matches!(
        apply_source_restored(&mut p),
        Err(ProgressError::IllegalTransition { .. })
    ));

    let mut p = LessonProgressState {
        status: LessonStatus::InProgress,
        resume_position_ms: 50,
        max_position_reached_ms: 50,
        document_position: 0,
    };
    assert!(matches!(
        apply_mark_incomplete(&mut p),
        Err(ProgressError::IllegalTransition { .. })
    ));
}
