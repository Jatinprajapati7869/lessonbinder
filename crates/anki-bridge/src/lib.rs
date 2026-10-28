//! AuraVault ↔ Anki (`rslib`) seam.

mod clip;
mod commit;
mod media;
mod note_display;
mod profile;
mod study;

pub use clip::{
    add_clip_note, deck_name_for_course, ensure_auravault_clip_notetype, get_clip_note, get_note,
    validate_auravault_deck_name, ClipDraft, ClipNote, AURAVAULT_CLIP_FIELDS,
    AURAVAULT_CLIP_NOTETYPE, AURAVAULT_DECK_PREFIX,
};
pub use commit::{
    audio_filename_for_bytes, commit_card_draft, screenshot_filename_for_bytes, CommitCardError,
};
pub use media::{read_media, ReadMediaError};
pub use note_display::{note_study_back_text, note_study_text, NoteStudyText};
pub use profile::open_profile;
pub use profile::read_stored_schema_version;
pub use profile::AuraVaultProfilePaths;
pub use profile::GuardedProfile;
pub use profile::COLLECTION_SCHEMA_VERSION;
pub use study::{
    answer_card, answer_card_by_id, due_summary, next_review_card, next_review_card_for_course,
    next_review_card_for_course_note_ids, start_review_session, AnswerCardError, DueSummary,
    ReviewAnswerButtons, ReviewCard, ReviewEase, ReviewSession, ReviewSessionAnswer,
};
