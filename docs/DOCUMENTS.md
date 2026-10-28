# Document lessons (PDF, Markdown, text)

Lessonbinder treats **`.pdf`**, **`.md`**, and **`.txt`** imports as document lessons (not the video player). Progress uses the same rules as video resume:

- **`not_started` → `in_progress`** when a saved position is greater than zero.
- **`done`** is still **manual only** (Mark complete on the lesson screen).

## Stored position (`progress.document_position`)

| Media               | Meaning of `document_position`                                                                                                                                                                                                                                                                                     |
| ------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| **PDF**             | **Page number** (1-based). v1.1 does not embed a PDF renderer; the UI shows the file path and an iframe/object pointed at the local file URL with a `#page=N` fragment where the viewer supports it. The page number is always persisted in `vault.db` even when the webview cannot render in-document navigation. |
| **Markdown / text** | **Character offset** into the loaded source file (scroll restore maps offset → scroll position by source length).                                                                                                                                                                                                  |

Markdown lessons render in the **player main area** (no `<video>`). The UI uses a small markdown subset (headings, paragraphs, image syntax). **Remote images are not fetched**—`![](https://…)` appears as a placeholder line instead of an `<img>` request.

Video lessons continue to use `resume_position_ms`; document lessons do not write playback ms for reading position.

## API seam

- Rust: `vault_core::set_document_position(db, lesson_id, position)`
- Tauri: `save_document_position { lessonId, position }`
- Course detail / lesson rows expose `document_position` for restore on open.

## Tests

- `crates/vault-core/tests/document_position.rs` — round-trip, in-progress transition, survives mark incomplete, course detail restore.
