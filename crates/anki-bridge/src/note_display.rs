//! Plain-text front/back and local media references for in-app review.

use std::collections::BTreeSet;
use std::sync::LazyLock;

use anki::prelude::*;
use anki::services::NotesService;
use anki::text::strip_html_preserving_media_filenames;
use anki_proto::notes::NoteId as ProtoNoteId;
use regex::Regex;
use regex::RegexBuilder;

use crate::clip::LESSONBINDER_CLIP_NOTETYPE;

static SCRIPT_TAG: LazyLock<Regex> = LazyLock::new(|| {
    RegexBuilder::new(r"(?is)<script\b[^>]*>.*?</script>")
        .build()
        .expect("script regex")
});

static EVENT_HANDLER_ATTR: LazyLock<Regex> = LazyLock::new(|| {
    RegexBuilder::new(r#"(?i)\s+on[a-z0-9_-]+\s*=\s*("[^"]*"|'[^']*'|[^\s>]+)"#)
        .build()
        .expect("event handler regex")
});

static SOUND_TAG: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?i)\[sound:([^\]]+)\]").expect("sound tag regex"));

static IMG_SRC: LazyLock<Regex> = LazyLock::new(|| {
    RegexBuilder::new(r#"(?i)<img\b[^>]*\bsrc\s*=\s*("([^"]+)"|'([^']+)'|([^\s>]+))"#)
        .build()
        .expect("img src regex")
});

/// Sanitized plain-text sides of a note plus referenced local media filenames.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NoteStudyText {
    pub front: String,
    pub back: String,
    pub media_filenames: Vec<String>,
}

/// Build review-safe plain text for `note_id`. Does not fetch remote URLs.
/// `note_study_text` with a primitive id for IPC boundaries.
pub fn note_study_back_text(col: &mut Collection, note_id: i64) -> Result<String> {
    note_study_text(col, NoteId(note_id)).map(|t| t.back)
}

pub fn note_study_text(col: &mut Collection, note_id: NoteId) -> Result<NoteStudyText> {
    let note: Note = NotesService::get_note(col, ProtoNoteId { nid: note_id.0 })?.into();
    let nt = col
        .get_notetype(note.notetype_id)?
        .or_not_found(note.notetype_id)?;
    let (front_raw, back_raw) = raw_front_back(&note, nt.as_ref());
    let mut media = BTreeSet::new();
    let front = plain_text_field(&front_raw, &mut media);
    let back = plain_text_field(&back_raw, &mut media);
    Ok(NoteStudyText {
        front,
        back,
        media_filenames: media.into_iter().collect(),
    })
}

fn raw_front_back(note: &Note, nt: &anki::notetype::Notetype) -> (String, String) {
    let fields = note.fields();
    if nt.name == LESSONBINDER_CLIP_NOTETYPE {
        return (fields[2].clone(), fields[3].clone());
    }
    if fields.len() >= 2 {
        (fields[0].clone(), fields[1].clone())
    } else if fields.len() == 1 {
        (fields[0].clone(), String::new())
    } else {
        (String::new(), String::new())
    }
}

fn plain_text_field(html: &str, media: &mut BTreeSet<String>) -> String {
    collect_media_references(html, media);
    let stripped = strip_scripts_and_handlers(html);
    strip_html_preserving_media_filenames(&stripped)
        .to_string()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .trim()
        .to_string()
}

fn strip_scripts_and_handlers(html: &str) -> String {
    let without_scripts = SCRIPT_TAG.replace_all(html, "");
    EVENT_HANDLER_ATTR
        .replace_all(&without_scripts, "")
        .to_string()
}

fn collect_media_references(html: &str, out: &mut BTreeSet<String>) {
    for cap in SOUND_TAG.captures_iter(html) {
        if let Some(name) = cap.get(1) {
            push_local_media(out, name.as_str());
        }
    }
    for cap in IMG_SRC.captures_iter(html) {
        let name = cap
            .get(2)
            .or_else(|| cap.get(3))
            .or_else(|| cap.get(4))
            .map(|m| m.as_str());
        if let Some(name) = name {
            push_local_media(out, name);
        }
    }
}

fn push_local_media(out: &mut BTreeSet<String>, name: &str) {
    let trimmed = name.trim();
    if trimmed.is_empty() {
        return;
    }
    if trimmed.starts_with("http://")
        || trimmed.starts_with("https://")
        || trimmed.starts_with("//")
    {
        return;
    }
    out.insert(trimmed.to_string());
}
