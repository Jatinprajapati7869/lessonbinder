//! AuraVault clip notes (`AuraVaultClip` notetype) and course-scoped decks.

use std::sync::Arc;

use anki::invalid_input;
use anki::notetype::CardTemplate;
use anki::notetype::NoteField;
use anki::notetype::Notetype;
use anki::prelude::*;
use anki::require;
use anki::services::NotesService;
use anki_proto::notes::NoteId as ProtoNoteId;

pub const AURAVAULT_CLIP_NOTETYPE: &str = "AuraVaultClip";
pub const AURAVAULT_DECK_PREFIX: &str = "AuraVault::";

pub const AURAVAULT_CLIP_FIELDS: [&str; 7] = [
    "Course",
    "Lesson",
    "Front",
    "Back",
    "Source",
    "Timestamp",
    "Screenshot",
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClipDraft {
    pub course: String,
    pub lesson: String,
    pub front: String,
    pub back: String,
    pub source: String,
    pub timestamp: String,
    pub screenshot: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClipNote {
    pub course: String,
    pub lesson: String,
    pub front: String,
    pub back: String,
    pub source: String,
    pub timestamp: String,
    pub screenshot: String,
}

/// Deck names must be exactly `AuraVault::<course title>` (no nesting).
pub fn validate_auravault_deck_name(name: &str) -> Result<()> {
    if !name.starts_with(AURAVAULT_DECK_PREFIX) {
        invalid_input!("deck name must start with `{AURAVAULT_DECK_PREFIX}`");
    }
    let title = name.strip_prefix(AURAVAULT_DECK_PREFIX).unwrap_or_default();
    if title.is_empty() {
        invalid_input!("deck name must include a course title after `{AURAVAULT_DECK_PREFIX}`");
    }
    if title.contains("::") {
        invalid_input!("deck name must be `{AURAVAULT_DECK_PREFIX}<course title>` only");
    }
    Ok(())
}

pub fn deck_name_for_course(course: &str) -> String {
    format!("{AURAVAULT_DECK_PREFIX}{course}")
}

pub fn ensure_auravault_clip_notetype(col: &mut Collection) -> Result<Arc<Notetype>> {
    if let Some(nt) = col.get_notetype_by_name(AURAVAULT_CLIP_NOTETYPE)? {
        return Ok(nt);
    }
    let mut nt = auravault_clip_notetype();
    col.add_notetype(&mut nt, false)?;
    col.get_notetype_by_name(AURAVAULT_CLIP_NOTETYPE)?
        .or_invalid("AuraVaultClip notetype missing after add")
}

pub fn add_clip_note(col: &mut Collection, draft: ClipDraft) -> Result<NoteId> {
    let deck_name = deck_name_for_course(&draft.course);
    validate_auravault_deck_name(&deck_name)?;

    let nt = ensure_auravault_clip_notetype(col)?;
    let deck = col.get_or_create_normal_deck(&deck_name)?;

    let mut note = nt.new_note();
    set_clip_fields(&mut note, &draft)?;
    col.add_note(&mut note, deck.id)?;
    Ok(note.id)
}

pub fn get_clip_note(col: &mut Collection, note_id: i64) -> Result<ClipNote> {
    get_note(col, NoteId(note_id))
}

pub fn get_note(col: &mut Collection, id: NoteId) -> Result<ClipNote> {
    let note: Note = NotesService::get_note(col, ProtoNoteId { nid: id.0 })?.into();
    let nt = col
        .get_notetype(note.notetype_id)?
        .or_not_found(note.notetype_id)?;
    if nt.name != AURAVAULT_CLIP_NOTETYPE {
        invalid_input!("note {id} is not an {AURAVAULT_CLIP_NOTETYPE} note");
    }
    clip_note_from_storage(&note)
}

fn fieldref(name: &str) -> String {
    format!("{{{{{}}}}}", name)
}

fn auravault_clip_notetype() -> Notetype {
    let front = fieldref(AURAVAULT_CLIP_FIELDS[2]);
    let screenshot = fieldref(AURAVAULT_CLIP_FIELDS[6]);
    let back = fieldref(AURAVAULT_CLIP_FIELDS[3]);
    let qfmt = format!("{front}\n{screenshot}");
    let afmt = format!("{{{{FrontSide}}}}\n<hr id=answer>\n{back}");
    Notetype {
        name: AURAVAULT_CLIP_NOTETYPE.into(),
        fields: AURAVAULT_CLIP_FIELDS
            .iter()
            .map(|name| NoteField::new(*name))
            .collect(),
        templates: vec![CardTemplate::new("Card 1", qfmt, afmt)],
        ..Notetype::default()
    }
}

fn set_clip_fields(note: &mut Note, draft: &ClipDraft) -> Result<()> {
    let values = [
        &draft.course,
        &draft.lesson,
        &draft.front,
        &draft.back,
        &draft.source,
        &draft.timestamp,
        &draft.screenshot,
    ];
    for (idx, value) in values.iter().enumerate() {
        note.set_field(idx, value.as_str())?;
    }
    Ok(())
}

fn clip_note_from_storage(note: &Note) -> Result<ClipNote> {
    let fields = note.fields();
    require!(
        fields.len() == AURAVAULT_CLIP_FIELDS.len(),
        "expected {} fields",
        AURAVAULT_CLIP_FIELDS.len()
    );
    Ok(ClipNote {
        course: fields[0].clone(),
        lesson: fields[1].clone(),
        front: fields[2].clone(),
        back: fields[3].clone(),
        source: fields[4].clone(),
        timestamp: fields[5].clone(),
        screenshot: fields[6].clone(),
    })
}
