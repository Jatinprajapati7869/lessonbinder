# Study and review

**In-app review is the default.** **Anki Desktop** is an optional fallback when you choose **Open in Anki Desktop**—you do not need Anki Desktop installed to use **Study**.

Lessonbinder tracks **due counts** in-process via `anki-bridge` (`due_summary()` on the Lessonbinder Anki collection). **Default study** is the in-app **Review** screen (`next_review_card`, `answer_card`, `note_study_text`, `read_media`).

## Flow

**Default — in-app Review**

1. The shell shows new / learning / review counts for `Lessonbinder::` decks (`due_summary()`).
2. **Study** opens Lessonbinder’s **Review** screen (not Anki Desktop).
3. Review loads the next due card from the Lessonbinder deck subtree (`next_review_card()`), shows the front and any **local** screenshot bytes via `read_media()` (no remote images).
4. The user taps **Show answer**; the back comes from `note_study_text()` as plain text.
5. Grading uses Again / Hard / Good / Easy **only when** the pinned `rslib` scheduler offers that button for the current card (`answer_card()`). Lessonbinder does not ship its own FSRS or scheduler implementation; it calls the same collection APIs Anki uses for queue and grades.

**Fallback — Anki Desktop**

6. **Open in Anki Desktop** on the Review screen launches the separate Anki application (we do not embed Anki’s Qt reviewer). Lessonbinder spawns the system **`anki`** binary from `PATH` with:
   - `-b` → parent of the Lessonbinder profile folder (`{app_data}/`)
   - `-p` → profile folder name (`anki-profile`)
7. Desktop Anki opens the same profile as in-app review: `collection.anki2` under `{app_data}/anki-profile/`. Use this when you want the full desktop reviewer or in-app review is unavailable.

## Errors

If `anki` is not on `PATH`, desktop launch fails with **`LaunchAnkiError::AnkiNotFound`**. In-app review still works; the Desktop button stays disabled with an explanation.
