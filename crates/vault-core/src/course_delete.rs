//! Course deletion side effects on vault-side Anki links (SPEC A-03).
//!
//! Deleting a course marks related [`AnkiCardLink`](crate::anki_link::AnkiCardLinkRecord) rows
//! as broken and retains `anki_note_id`; Anki collection notes are not touched here.
