# Cancel folder import — design

This note defines how **Cancel** during **Add folder** / drag-and-drop import should behave. It applies to the Tauri command **`import_course_folder`** (vault: folder scan + course rows in **`vault.db`**). Playlist paste and JSON import are out of scope here.

---

## Why Cancel must wait for yield

**`import_course_folder`** is a **`#[tauri::command(async)]`** handler: the folder walk and vault upserts run off the main thread, but the frontend still awaits the command until import finishes, errors, or cancels. A **Cancel** control sets a flag (`cancel_import_course_folder`); import only stops when the implementation **checks that flag between work units**—between directory entries while scanning and again before each lesson upsert—not in the middle of a single lesson write.

---

## Cancel granularity

| Rule                                 | Meaning                                                                                                                                                                             |
| ------------------------------------ | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Between files**                    | Cancel is honored only **after** the current file’s vault row is committed and **before** starting the next eligible path. Do **not** abort halfway through writing one lesson row. |
| **Lessons already written stay**     | Rows inserted before cancel remain in the course; nothing rolls back completed lesson records.                                                                                      |
| **Cancelled import is not “done”**   | Cancel does **not** call manual completion or mark any lesson **`done`**. Lesson progress stays **`not_started`** (or whatever it was before that upsert).                          |
| **Cancelled import is not “broken”** | Cancel does **not** mark lessons **`broken`** (that path is for missing media on rescan). Partial courses stay usable; the user can rescan or import again later.                   |

If cancel fires before any lesson is written, the course row may be empty or absent depending on implementation; the important part is **no half-written lesson** and **no false “done”**.

---

## What import does not touch

**Folder import does not write Anki notes.** It only updates **`vault.db`** (courses, lessons, paths, ignore rules, etc.). Cancelling import therefore never deletes or rolls back cards in the Lessonbinder **`anki-profile/`** collection.

---

## Escape is not Cancel

**Escape** exits **theater** and **zen** player modes (and related player chrome). It does **not** cancel folder import and must not be wired to **`cancel_import_course_folder`**.

Import cancel belongs on the library import / scanning UI (e.g. the in-progress folder scan affordance), not on player keyboard shortcuts.

---

## Related

- Implementation target: **`import_folder_with_cancel`** in **`vault-core`**, **`cancel_import_course_folder`** / **`import_course_folder`** in **`src-tauri/src/library_commands.rs`**
- Next step: prompt **211** (library Cancel control)
- Competitor note: AulaVault-style **`cancel_scan_course_folder`** — [`AULAVAULT-BEHAVIOR.md`](AULAVAULT-BEHAVIOR.md)
