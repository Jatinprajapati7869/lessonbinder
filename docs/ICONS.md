# Outline icons (FlutKit candidates)

Source kit (owner-licensed; **not** copied into this repository in prompt **319**):

`/home/xingzi/Documents/FlutKit v17.0/codecanyon-27510289-flutkit-flutter-ui-kit/FlutKit/assets/icons/`

The project owner’s **FlutKit** license covers use of these outline PNGs in Lessonbinder. Only the files listed below are candidates for prompts **320+**; **illustrations**, stock imagery, and the **rest of the FlutKit / ProKit trees stay unused** (typography-only kit files in the app remain the four bundled `.ttf` files — see [`FONTS.md`](FONTS.md)).

## Candidate outline PNGs (verified on disk, 2026-10-28)

| UI control (accessible name or role) | FlutKit filename (outline PNG) |
| ------------------------------------ | ------------------------------ |
| **Show folder**                      | `file-tray-full-outline.png`   |
| **Search** (library search)          | `search-outline.png`           |
| **Settings** (side nav)              | `setting-outline.png`          |
| **Reader** (document / lesson file)  | `reader-outline.png`           |
| **School** (study / course chrome)   | `school-outline.png`           |
| **List** (lesson list)               | `list-outline.png`             |

All six candidate names above exist in the FlutKit icons directory. No other kit icon paths are planned for this pack.

## Repo copies (prompt 320)

Copied into `ui/src/theme/icons/` with the same filenames as the table above (FlutKit source path unchanged in this doc).

**Show folder (prompt 321):** `file-tray-full-outline.png` beside the **Show folder** label on the folder course screen (`CourseScreen`); accessible name remains **Show folder** (`aria-label`).

**Search library (prompt 322):** `search-outline.png` inside the library search field beside the input; visible label **Search library** and `htmlFor` / `id` association unchanged.

**Settings (prompt 323):** `setting-outline.png` beside the side-nav **Settings** label; accessible name remains **Settings** (visible button text).

**Reader and list (prompt 324):** `reader-outline.png` beside **Show lesson file** (`PlayerScreen`) and beside the **Notes** heading (`LessonNotesColumn`); `list-outline.png` beside the **Lessons** heading above the course lesson table (`CourseScreen`). Text labels stay; icons are decorative (`aria-hidden`).

**School:** `school-outline.png` is **unused**—no side-nav or toolbar control is named **School** (or equivalent) in the app; the **Study** button keeps text only.
