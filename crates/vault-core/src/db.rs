//! SQLite access and migrations for `vault.db`.

use std::path::{Path, PathBuf};
use std::sync::Mutex;

use rusqlite::Connection;
use rusqlite::OptionalExtension;
use uuid::Uuid;

mod error;

pub use error::DbError;

const MIGRATIONS: &[(&str, &str)] = &[
    ("001_initial", include_str!("../migrations/001_initial.sql")),
    (
        "002_lesson_metadata",
        include_str!("../migrations/002_lesson_metadata.sql"),
    ),
    (
        "003_order_proposal",
        include_str!("../migrations/003_order_proposal.sql"),
    ),
    ("004_roadmap", include_str!("../migrations/004_roadmap.sql")),
    (
        "005_roadmap_prerequisite",
        include_str!("../migrations/005_roadmap_prerequisite.sql"),
    ),
    (
        "006_playlist_item",
        include_str!("../migrations/006_playlist_item.sql"),
    ),
    (
        "007_anki_card_link",
        include_str!("../migrations/007_anki_card_link.sql"),
    ),
    (
        "008_anki_card_link_lesson_nullable",
        include_str!("../migrations/008_anki_card_link_lesson_nullable.sql"),
    ),
    (
        "009_watched_range",
        include_str!("../migrations/009_watched_range.sql"),
    ),
    (
        "010_lesson_note",
        include_str!("../migrations/010_lesson_note.sql"),
    ),
    (
        "011_document_position",
        include_str!("../migrations/011_document_position.sql"),
    ),
    (
        "012_course_archive",
        include_str!("../migrations/012_course_archive.sql"),
    ),
    (
        "013_vault_settings",
        include_str!("../migrations/013_vault_settings.sql"),
    ),
    (
        "014_lesson_transcription_cue",
        include_str!("../migrations/014_lesson_transcription_cue.sql"),
    ),
    (
        "015_lesson_resource",
        include_str!("../migrations/015_lesson_resource.sql"),
    ),
    (
        "016_lesson_thumbnail",
        include_str!("../migrations/016_lesson_thumbnail.sql"),
    ),
    (
        "017_anki_card_link_front_preview",
        include_str!("../migrations/017_anki_card_link_front_preview.sql"),
    ),
    (
        "018_lesson_chapter",
        include_str!("../migrations/018_lesson_chapter.sql"),
    ),
    (
        "019_ui_theme",
        include_str!("../migrations/019_ui_theme.sql"),
    ),
    (
        "020_study_day",
        include_str!("../migrations/020_study_day.sql"),
    ),
    (
        "021_card_grade_event",
        include_str!("../migrations/021_card_grade_event.sql"),
    ),
    (
        "022_playback_speed",
        include_str!("../migrations/022_playback_speed.sql"),
    ),
    (
        "023_player_volume",
        include_str!("../migrations/023_player_volume.sql"),
    ),
    (
        "024_source_file_inode",
        include_str!("../migrations/024_source_file_inode.sql"),
    ),
];

const DEFAULT_LIBRARY_ID: &str = "00000000-0000-4000-8000-000000000001";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LessonStatus {
    NotStarted,
    InProgress,
    Done,
    Broken,
}

impl LessonStatus {
    pub(crate) fn as_db_str(self) -> &'static str {
        match self {
            LessonStatus::NotStarted => "not_started",
            LessonStatus::InProgress => "in_progress",
            LessonStatus::Done => "done",
            LessonStatus::Broken => "broken",
        }
    }

    fn from_str(value: &str) -> Result<Self, DbError> {
        match value {
            "not_started" => Ok(LessonStatus::NotStarted),
            "in_progress" => Ok(LessonStatus::InProgress),
            "done" => Ok(LessonStatus::Done),
            "broken" => Ok(LessonStatus::Broken),
            other => Err(DbError::InvalidData(format!(
                "unknown lesson status: {other}"
            ))),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LessonProgress {
    pub status: LessonStatus,
    pub resume_position_ms: i64,
    pub max_position_reached_ms: i64,
    pub document_position: i64,
}

#[derive(Debug, Clone)]
pub struct LessonSearchRow {
    pub lesson_id: String,
    pub course_id: String,
    pub course_title: String,
    pub display_title: String,
    pub relative_path: Option<String>,
}

#[derive(Debug, Clone)]
pub struct LessonDetailRow {
    pub lesson_id: String,
    pub display_title: String,
    pub sort_index: i32,
    pub lesson_source_kind: String,
    pub duration_ms: Option<i64>,
    pub status: LessonStatus,
    pub resume_position_ms: i64,
    pub max_position_reached_ms: i64,
    pub document_position: i64,
    pub playlist_item_id: Option<String>,
    pub playlist_url: Option<String>,
    pub course_root_path: Option<String>,
    pub media_relative_path: Option<String>,
    pub thumbnail_path: Option<String>,
}

#[derive(Debug, Clone)]
pub(crate) struct RoadmapStepRow {
    pub id: String,
    pub course_id: Option<String>,
    pub course_title: String,
    pub note: Option<String>,
}

#[derive(Debug, Clone)]
pub(crate) struct RoadmapListRow {
    pub id: String,
    pub title: String,
}

#[derive(Debug, Clone)]
pub(crate) struct CourseListRow {
    pub id: String,
    pub title: String,
    pub source_kind: String,
}

#[derive(Debug, Clone)]
pub(crate) struct RoadmapGraphStepRow {
    pub step_id: String,
    pub course_id: Option<String>,
    pub course_title: String,
}

#[derive(Debug, Clone)]
pub struct LocalLessonSource {
    pub lesson_id: String,
    pub root_path: PathBuf,
    pub relative_path: String,
}

#[derive(Debug, Clone)]
pub(crate) struct LessonNoteRow {
    pub id: String,
    pub lesson_id: String,
    pub body: String,
    pub timestamp_ms: Option<i64>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone)]
pub struct LocalFileLessonRow {
    pub lesson_id: String,
    pub stable_key: String,
    pub sort_index: i32,
}

#[derive(Debug, Clone)]
pub(crate) struct LessonThumbnailContextRow {
    pub source_kind: String,
    pub media_relative_path: Option<String>,
}

#[derive(Debug, Clone)]
pub(crate) struct LessonResourceRow {
    pub resource_id: String,
    pub relative_path: String,
    pub kind: String,
    pub course_root_path: Option<String>,
}

pub struct VaultDb {
    conn: Mutex<Connection>,
}

impl VaultDb {
    pub fn open_in_memory() -> Result<Self, DbError> {
        let conn = Connection::open_in_memory()?;
        migrate(&conn)?;
        ensure_default_library(&conn)?;
        Ok(Self {
            conn: Mutex::new(conn),
        })
    }

    pub fn open(path: impl AsRef<Path>) -> Result<Self, DbError> {
        let conn = Connection::open(path)?;
        migrate(&conn)?;
        ensure_default_library(&conn)?;
        Ok(Self {
            conn: Mutex::new(conn),
        })
    }

    pub fn applied_migration_count(&self) -> usize {
        let conn = self.conn.lock().expect("vault db lock");
        conn.query_row("SELECT COUNT(*) FROM _schema_migrations", [], |row| {
            row.get(0)
        })
        .unwrap_or(0)
    }

    pub fn insert_course(&self, title: &str, root_path: &str) -> Result<String, DbError> {
        let conn = self.conn.lock().expect("vault db lock");
        let id = Uuid::new_v4().to_string();
        let now = now_iso8601();
        conn.execute(
            "INSERT INTO course (id, library_id, title, source_kind, root_path, imported_at, last_scanned_at)
             VALUES (?1, ?2, ?3, 'folder', ?4, ?5, ?5)",
            rusqlite::params![id, DEFAULT_LIBRARY_ID, title, root_path, now],
        )?;
        Ok(id)
    }

    pub fn insert_local_file_lesson(
        &self,
        course_id: &str,
        relative_path: &str,
        display_title: &str,
        sort_index: i32,
    ) -> Result<String, DbError> {
        let conn = self.conn.lock().expect("vault db lock");
        let source_file_id = Uuid::new_v4().to_string();
        let lesson_id = Uuid::new_v4().to_string();
        let now = now_iso8601();
        let extension = Path::new(relative_path)
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_ascii_lowercase();
        let kind = source_kind_for_extension(&extension);

        let tx = conn.unchecked_transaction()?;
        tx.execute(
            "INSERT INTO source_file (id, course_id, relative_path, kind, extension, last_seen_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            rusqlite::params![
                source_file_id,
                course_id,
                relative_path,
                kind,
                extension,
                now
            ],
        )?;
        tx.execute(
            "INSERT INTO lesson (
                id, course_id, lesson_stable_key, display_title, sort_index,
                source_kind, source_file_id
             ) VALUES (?1, ?2, ?3, ?4, ?5, 'local_file', ?6)",
            rusqlite::params![
                lesson_id,
                course_id,
                relative_path,
                display_title,
                sort_index,
                source_file_id
            ],
        )?;
        tx.commit()?;
        Ok(lesson_id)
    }

    pub fn delete_lesson(&self, lesson_id: &str) -> Result<(), DbError> {
        let conn = self.conn.lock().expect("vault db lock");
        let deleted = conn.execute("DELETE FROM lesson WHERE id = ?1", [lesson_id])?;
        if deleted != 1 {
            return Err(DbError::InvalidData(format!(
                "lesson {lesson_id} not found"
            )));
        }
        Ok(())
    }

    pub(crate) fn insert_lesson_note(
        &self,
        lesson_id: &str,
        body: &str,
        timestamp_ms: Option<i64>,
    ) -> Result<LessonNoteRow, DbError> {
        let id = Uuid::new_v4().to_string();
        let now = lesson_note_timestamp();
        {
            let conn = self.conn.lock().expect("vault db lock");
            conn.execute(
                "INSERT INTO lesson_note (id, lesson_id, body, timestamp_ms, created_at, updated_at)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                rusqlite::params![id, lesson_id, body, timestamp_ms, now, now],
            )?;
        }
        self.lesson_note_by_id(&id)?
            .ok_or_else(|| DbError::InvalidData("lesson note insert failed".into()))
    }

    pub(crate) fn update_lesson_note(
        &self,
        note_id: &str,
        body: &str,
        timestamp_ms: Option<i64>,
    ) -> Result<LessonNoteRow, DbError> {
        let now = lesson_note_timestamp();
        {
            let conn = self.conn.lock().expect("vault db lock");
            let updated = conn.execute(
                "UPDATE lesson_note
                 SET body = ?1, timestamp_ms = ?2, updated_at = ?3
                 WHERE id = ?4",
                rusqlite::params![body, timestamp_ms, now, note_id],
            )?;
            if updated != 1 {
                return Err(DbError::InvalidData("lesson note not found".into()));
            }
        }
        self.lesson_note_by_id(note_id)?
            .ok_or_else(|| DbError::InvalidData("lesson note not found".into()))
    }

    pub(crate) fn delete_lesson_note(&self, note_id: &str) -> Result<(), DbError> {
        let conn = self.conn.lock().expect("vault db lock");
        let deleted = conn.execute("DELETE FROM lesson_note WHERE id = ?1", [note_id])?;
        if deleted != 1 {
            return Err(DbError::InvalidData("lesson note not found".into()));
        }
        Ok(())
    }

    pub(crate) fn list_lesson_notes(&self, lesson_id: &str) -> Result<Vec<LessonNoteRow>, DbError> {
        let conn = self.conn.lock().expect("vault db lock");
        let mut stmt = conn.prepare(
            "SELECT id, lesson_id, body, timestamp_ms, created_at, updated_at
             FROM lesson_note
             WHERE lesson_id = ?1
             ORDER BY timestamp_ms IS NULL, timestamp_ms, created_at",
        )?;
        let rows = stmt
            .query_map([lesson_id], |row| {
                Ok(LessonNoteRow {
                    id: row.get(0)?,
                    lesson_id: row.get(1)?,
                    body: row.get(2)?,
                    timestamp_ms: row.get(3)?,
                    created_at: row.get(4)?,
                    updated_at: row.get(5)?,
                })
            })?
            .collect::<Result<Vec<_>, rusqlite::Error>>()?;
        Ok(rows)
    }

    pub(crate) fn lesson_note_by_id(
        &self,
        note_id: &str,
    ) -> Result<Option<LessonNoteRow>, DbError> {
        let conn = self.conn.lock().expect("vault db lock");
        conn.query_row(
            "SELECT id, lesson_id, body, timestamp_ms, created_at, updated_at
             FROM lesson_note WHERE id = ?1",
            [note_id],
            |row| {
                Ok(LessonNoteRow {
                    id: row.get(0)?,
                    lesson_id: row.get(1)?,
                    body: row.get(2)?,
                    timestamp_ms: row.get(3)?,
                    created_at: row.get(4)?,
                    updated_at: row.get(5)?,
                })
            },
        )
        .optional()
        .map_err(DbError::from)
    }

    pub fn lessons_for_search(&self) -> Result<Vec<LessonSearchRow>, DbError> {
        let conn = self.conn.lock().expect("vault db lock");
        let mut stmt = conn.prepare(
            "SELECT l.id, l.course_id, c.title, l.display_title, sf.relative_path
             FROM lesson l
             INNER JOIN course c ON c.id = l.course_id
             LEFT JOIN source_file sf ON sf.id = l.source_file_id
             ORDER BY l.course_id, l.sort_index",
        )?;
        let rows = stmt
            .query_map([], |row| {
                Ok(LessonSearchRow {
                    lesson_id: row.get(0)?,
                    course_id: row.get(1)?,
                    course_title: row.get(2)?,
                    display_title: row.get(3)?,
                    relative_path: row.get(4)?,
                })
            })?
            .collect::<Result<Vec<_>, rusqlite::Error>>()?;
        Ok(rows)
    }

    pub(crate) fn lesson_search_row(&self, lesson_id: &str) -> Result<LessonSearchRow, DbError> {
        let conn = self.conn.lock().expect("vault db lock");
        conn.query_row(
            "SELECT l.id, l.course_id, c.title, l.display_title, sf.relative_path
             FROM lesson l
             INNER JOIN course c ON c.id = l.course_id
             LEFT JOIN source_file sf ON sf.id = l.source_file_id
             WHERE l.id = ?1",
            [lesson_id],
            |row| {
                Ok(LessonSearchRow {
                    lesson_id: row.get(0)?,
                    course_id: row.get(1)?,
                    course_title: row.get(2)?,
                    display_title: row.get(3)?,
                    relative_path: row.get(4)?,
                })
            },
        )
        .map_err(DbError::from)
    }

    pub(crate) fn lessons_matching_title_or_path(
        &self,
        like_pattern: &str,
    ) -> Result<Vec<LessonSearchRow>, DbError> {
        let conn = self.conn.lock().expect("vault db lock");
        let mut stmt = conn.prepare(
            "SELECT l.id, l.course_id, c.title, l.display_title, sf.relative_path
             FROM lesson l
             INNER JOIN course c ON c.id = l.course_id
             LEFT JOIN source_file sf ON sf.id = l.source_file_id
             WHERE LOWER(l.display_title) LIKE ?1 ESCAPE '\\'
                OR LOWER(sf.relative_path) LIKE ?1 ESCAPE '\\'
             ORDER BY l.course_id, l.sort_index",
        )?;
        let rows = stmt
            .query_map([like_pattern], |row| {
                Ok(LessonSearchRow {
                    lesson_id: row.get(0)?,
                    course_id: row.get(1)?,
                    course_title: row.get(2)?,
                    display_title: row.get(3)?,
                    relative_path: row.get(4)?,
                })
            })?
            .collect::<Result<Vec<_>, rusqlite::Error>>()?;
        Ok(rows)
    }

    pub(crate) fn lesson_ids_with_subtitle_sidecar(&self) -> Result<Vec<String>, DbError> {
        let conn = self.conn.lock().expect("vault db lock");
        let mut stmt = conn
            .prepare("SELECT id FROM lesson WHERE subtitle_file_id IS NOT NULL ORDER BY course_id, sort_index")?;
        let rows = stmt
            .query_map([], |row| row.get(0))?
            .collect::<Result<Vec<String>, rusqlite::Error>>()?;
        Ok(rows)
    }

    pub(crate) fn search_transcription_cue_rows(
        &self,
        like_pattern: &str,
    ) -> Result<Vec<(LessonSearchRow, i64, String)>, DbError> {
        let conn = self.conn.lock().expect("vault db lock");
        let mut stmt = conn.prepare(
            "SELECT l.id, l.course_id, c.title, l.display_title, sf.relative_path,
                    tc.start_ms, tc.text
             FROM lesson_transcription_cue tc
             INNER JOIN lesson l ON l.id = tc.lesson_id
             INNER JOIN course c ON c.id = l.course_id
             LEFT JOIN source_file sf ON sf.id = l.source_file_id
             WHERE tc.source = 'transcription'
               AND LOWER(tc.text) LIKE ?1 ESCAPE '\\'
             ORDER BY l.course_id, l.sort_index, tc.sort_index",
        )?;
        let rows = stmt
            .query_map([like_pattern], |row| {
                Ok((
                    LessonSearchRow {
                        lesson_id: row.get(0)?,
                        course_id: row.get(1)?,
                        course_title: row.get(2)?,
                        display_title: row.get(3)?,
                        relative_path: row.get(4)?,
                    },
                    row.get(5)?,
                    row.get(6)?,
                ))
            })?
            .collect::<Result<Vec<_>, rusqlite::Error>>()?;
        Ok(rows)
    }

    pub(crate) fn search_lesson_note_rows(
        &self,
        like_pattern: &str,
    ) -> Result<Vec<(LessonSearchRow, Option<i64>, String)>, DbError> {
        let conn = self.conn.lock().expect("vault db lock");
        let mut stmt = conn.prepare(
            "SELECT l.id, l.course_id, c.title, l.display_title, sf.relative_path,
                    ln.timestamp_ms, ln.body
             FROM lesson_note ln
             INNER JOIN lesson l ON l.id = ln.lesson_id
             INNER JOIN course c ON c.id = l.course_id
             LEFT JOIN source_file sf ON sf.id = l.source_file_id
             WHERE LOWER(ln.body) LIKE ?1 ESCAPE '\\'
             ORDER BY l.course_id, l.sort_index, ln.timestamp_ms, ln.created_at",
        )?;
        let rows = stmt
            .query_map([like_pattern], |row| {
                Ok((
                    LessonSearchRow {
                        lesson_id: row.get(0)?,
                        course_id: row.get(1)?,
                        course_title: row.get(2)?,
                        display_title: row.get(3)?,
                        relative_path: row.get(4)?,
                    },
                    row.get(5)?,
                    row.get(6)?,
                ))
            })?
            .collect::<Result<Vec<_>, rusqlite::Error>>()?;
        Ok(rows)
    }

    pub fn lesson_count_for_course(&self, course_id: &str) -> Result<i64, DbError> {
        let conn = self.conn.lock().expect("vault db lock");
        conn.query_row(
            "SELECT COUNT(*) FROM lesson WHERE course_id = ?1",
            [course_id],
            |row| row.get(0),
        )
        .map_err(DbError::from)
    }

    pub fn course_title(&self, course_id: &str) -> Result<String, DbError> {
        let conn = self.conn.lock().expect("vault db lock");
        conn.query_row(
            "SELECT title FROM course WHERE id = ?1",
            [course_id],
            |row| row.get(0),
        )
        .map_err(DbError::from)
    }

    pub fn lesson_display_title(&self, lesson_id: &str) -> Result<String, DbError> {
        let conn = self.conn.lock().expect("vault db lock");
        conn.query_row(
            "SELECT display_title FROM lesson WHERE id = ?1",
            [lesson_id],
            |row| row.get(0),
        )
        .map_err(DbError::from)
    }

    pub fn lesson_details_for_course(
        &self,
        course_id: &str,
    ) -> Result<Vec<LessonDetailRow>, DbError> {
        let conn = self.conn.lock().expect("vault db lock");
        let mut stmt = conn.prepare(
            "SELECT l.id, l.display_title, l.sort_index, l.source_kind, l.duration_ms,
                    p.status, p.resume_position_ms, p.max_position_reached_ms, p.document_position,
                    pi.id, pi.url,
                    c.root_path, sf.relative_path, l.thumbnail_path
             FROM lesson l
             INNER JOIN progress p ON p.lesson_id = l.id
             INNER JOIN course c ON c.id = l.course_id
             LEFT JOIN playlist_item pi ON pi.id = l.playlist_item_id
             LEFT JOIN source_file sf ON sf.id = l.source_file_id
             WHERE l.course_id = ?1
             ORDER BY l.sort_index",
        )?;
        let rows = stmt
            .query_map([course_id], |row| {
                let status_str: String = row.get(5)?;
                Ok(LessonDetailRow {
                    lesson_id: row.get(0)?,
                    display_title: row.get(1)?,
                    sort_index: row.get(2)?,
                    lesson_source_kind: row.get(3)?,
                    duration_ms: row.get(4)?,
                    status: LessonStatus::from_str(&status_str)
                        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?,
                    resume_position_ms: row.get(6)?,
                    max_position_reached_ms: row.get(7)?,
                    document_position: row.get(8)?,
                    playlist_item_id: row.get(9)?,
                    playlist_url: row.get(10)?,
                    course_root_path: row.get(11)?,
                    media_relative_path: row.get(12)?,
                    thumbnail_path: row.get(13)?,
                })
            })?
            .collect::<Result<Vec<_>, rusqlite::Error>>()?;
        Ok(rows)
    }

    pub fn first_lesson_thumbnail_path_for_course(
        &self,
        course_id: &str,
    ) -> Result<Option<String>, DbError> {
        let conn = self.conn.lock().expect("vault db lock");
        conn.query_row(
            "SELECT thumbnail_path FROM lesson
             WHERE course_id = ?1 AND thumbnail_path IS NOT NULL
             ORDER BY sort_index
             LIMIT 1",
            [course_id],
            |row| row.get(0),
        )
        .optional()
        .map_err(DbError::from)
    }

    pub fn pending_order_proposal(
        &self,
        course_id: &str,
    ) -> Result<Option<(String, Vec<String>)>, DbError> {
        let conn = self.conn.lock().expect("vault db lock");
        let proposal_id: Option<String> = conn
            .query_row(
                "SELECT id FROM order_proposal WHERE course_id = ?1",
                [course_id],
                |row| row.get(0),
            )
            .optional()?;
        let Some(proposal_id) = proposal_id else {
            return Ok(None);
        };
        let mut stmt = conn.prepare(
            "SELECT lesson_id FROM order_proposal_entry
             WHERE proposal_id = ?1
             ORDER BY position",
        )?;
        let lesson_ids = stmt
            .query_map([&proposal_id], |row| row.get(0))?
            .collect::<Result<Vec<String>, rusqlite::Error>>()?;
        Ok(Some((proposal_id, lesson_ids)))
    }

    pub fn course_root_path(&self, course_id: &str) -> Result<Option<String>, DbError> {
        let conn = self.conn.lock().expect("vault db lock");
        conn.query_row(
            "SELECT root_path FROM course WHERE id = ?1",
            [course_id],
            |row| row.get(0),
        )
        .optional()
        .map_err(DbError::from)
    }

    pub fn local_file_lessons_for_course(
        &self,
        course_id: &str,
    ) -> Result<Vec<LocalFileLessonRow>, DbError> {
        let conn = self.conn.lock().expect("vault db lock");
        let mut stmt = conn.prepare(
            "SELECT id, lesson_stable_key, sort_index FROM lesson
             WHERE course_id = ?1 AND source_kind = 'local_file'
             ORDER BY sort_index",
        )?;
        let rows = stmt
            .query_map([course_id], |row| {
                Ok(LocalFileLessonRow {
                    lesson_id: row.get(0)?,
                    stable_key: row.get(1)?,
                    sort_index: row.get(2)?,
                })
            })?
            .collect::<Result<Vec<_>, rusqlite::Error>>()?;
        Ok(rows)
    }

    pub(crate) fn lesson_source_inode_identity(
        &self,
        lesson_id: &str,
    ) -> Result<Option<(u64, u64, u64)>, DbError> {
        let conn = self.conn.lock().expect("vault db lock");
        let row = conn
            .query_row(
                "SELECT sf.file_dev, sf.file_ino, sf.size_bytes
                 FROM lesson l
                 INNER JOIN source_file sf ON sf.id = l.source_file_id
                 WHERE l.id = ?1",
                [lesson_id],
                |row| {
                    Ok((
                        row.get::<_, Option<i64>>(0)?,
                        row.get::<_, Option<i64>>(1)?,
                        row.get::<_, Option<i64>>(2)?,
                    ))
                },
            )
            .optional()?;
        Ok(row.and_then(|(dev, ino, size)| {
            let (dev, ino, size) = (dev?, ino?, size?);
            if dev < 0 || ino < 0 || size < 0 {
                return None;
            }
            Some((dev as u64, ino as u64, size as u64))
        }))
    }

    pub(crate) fn relink_local_file_lesson(
        &self,
        lesson_id: &str,
        new_relative_path: &str,
    ) -> Result<(), DbError> {
        let conn = self.conn.lock().expect("vault db lock");
        let now = now_iso8601();
        let tx = conn.unchecked_transaction()?;
        tx.execute(
            "UPDATE lesson SET lesson_stable_key = ?1 WHERE id = ?2",
            rusqlite::params![new_relative_path, lesson_id],
        )?;
        tx.execute(
            "UPDATE source_file
             SET relative_path = ?1, last_seen_at = ?2, missing_since = NULL
             WHERE id = (SELECT source_file_id FROM lesson WHERE id = ?3)",
            rusqlite::params![new_relative_path, now, lesson_id],
        )?;
        tx.commit()?;
        Ok(())
    }

    pub fn upsert_scanned_lessons(
        &self,
        course_id: &str,
        course_root: &Path,
        lessons: &[crate::import::ScannedLesson],
    ) -> Result<Vec<String>, DbError> {
        let existing = self.local_file_lessons_for_course(course_id)?;
        let sort_for_key: std::collections::HashMap<String, i32> = existing
            .iter()
            .map(|row| (row.stable_key.clone(), row.sort_index))
            .collect();
        let mut next_sort = existing
            .iter()
            .map(|row| row.sort_index)
            .max()
            .unwrap_or(-1)
            + 1;

        {
            let conn = self.conn.lock().expect("vault db lock");
            let now = now_iso8601();
            conn.execute(
                "UPDATE course SET last_scanned_at = ?1 WHERE id = ?2",
                rusqlite::params![now, course_id],
            )?;
            let tx = conn.unchecked_transaction()?;
            for lesson in lessons {
                let sort_index = sort_for_key
                    .get(&lesson.relative_path)
                    .copied()
                    .unwrap_or_else(|| {
                        let index = next_sort;
                        next_sort += 1;
                        index
                    });
                upsert_lesson_from_scan(&tx, course_id, course_root, lesson, sort_index, &now)?;
            }
            tx.commit()?;
        }
        self.lesson_ids_in_learning_order(course_id)
    }

    /// Upsert one scanned lesson in its own transaction (for cancellable folder import).
    pub fn upsert_single_scanned_lesson(
        &self,
        course_id: &str,
        course_root: &Path,
        lesson: &crate::import::ScannedLesson,
    ) -> Result<String, DbError> {
        let existing = self.local_file_lessons_for_course(course_id)?;
        let sort_for_key: std::collections::HashMap<String, i32> = existing
            .iter()
            .map(|row| (row.stable_key.clone(), row.sort_index))
            .collect();
        let next_sort = existing
            .iter()
            .map(|row| row.sort_index)
            .max()
            .unwrap_or(-1)
            + 1;
        let sort_index = sort_for_key
            .get(&lesson.relative_path)
            .copied()
            .unwrap_or(next_sort);

        let conn = self.conn.lock().expect("vault db lock");
        let now = now_iso8601();
        conn.execute(
            "UPDATE course SET last_scanned_at = ?1 WHERE id = ?2",
            rusqlite::params![now, course_id],
        )?;
        let tx = conn.unchecked_transaction()?;
        let lesson_id =
            upsert_lesson_from_scan(&tx, course_id, course_root, lesson, sort_index, &now)?;
        tx.commit()?;
        Ok(lesson_id)
    }

    pub(crate) fn ensure_folder_course(
        &self,
        root_path: &str,
        title: &str,
    ) -> Result<String, DbError> {
        let conn = self.conn.lock().expect("vault db lock");
        let now = now_iso8601();
        match conn
            .query_row(
                "SELECT id FROM course WHERE root_path = ?1",
                [root_path],
                |row| row.get::<_, String>(0),
            )
            .optional()?
        {
            Some(id) => {
                conn.execute(
                    "UPDATE course SET last_scanned_at = ?1, title = ?2 WHERE id = ?3",
                    rusqlite::params![now, title, id],
                )?;
                Ok(id)
            }
            None => {
                let id = Uuid::new_v4().to_string();
                conn.execute(
                    "INSERT INTO course (id, library_id, title, source_kind, root_path, imported_at, last_scanned_at)
                     VALUES (?1, ?2, ?3, 'folder', ?4, ?5, ?5)",
                    rusqlite::params![id, DEFAULT_LIBRARY_ID, title, root_path, now],
                )?;
                Ok(id)
            }
        }
    }

    pub fn course_source_kind(&self, course_id: &str) -> Result<String, DbError> {
        let conn = self.conn.lock().expect("vault db lock");
        conn.query_row(
            "SELECT source_kind FROM course WHERE id = ?1",
            [course_id],
            |row| row.get(0),
        )
        .map_err(DbError::from)
    }

    pub fn playlist_item_id_for_external_id(
        &self,
        course_id: &str,
        external_id: &str,
    ) -> Result<String, DbError> {
        let conn = self.conn.lock().expect("vault db lock");
        conn.query_row(
            "SELECT id FROM playlist_item WHERE course_id = ?1 AND external_id = ?2",
            rusqlite::params![course_id, external_id],
            |row| row.get(0),
        )
        .map_err(DbError::from)
    }

    pub fn lesson_id_for_playlist_item(
        &self,
        playlist_item_id: &str,
    ) -> Result<Option<String>, DbError> {
        let conn = self.conn.lock().expect("vault db lock");
        conn.query_row(
            "SELECT id FROM lesson WHERE playlist_item_id = ?1 AND source_kind = 'playlist_item'",
            [playlist_item_id],
            |row| row.get(0),
        )
        .optional()
        .map_err(DbError::from)
    }

    pub fn lesson_id_for_stable_key(
        &self,
        course_id: &str,
        lesson_stable_key: &str,
    ) -> Result<String, DbError> {
        let conn = self.conn.lock().expect("vault db lock");
        conn.query_row(
            "SELECT id FROM lesson WHERE course_id = ?1 AND lesson_stable_key = ?2",
            rusqlite::params![course_id, lesson_stable_key],
            |row| row.get(0),
        )
        .map_err(DbError::from)
    }

    pub fn lesson_ids_in_learning_order(&self, course_id: &str) -> Result<Vec<String>, DbError> {
        let conn = self.conn.lock().expect("vault db lock");
        let mut stmt =
            conn.prepare("SELECT id FROM lesson WHERE course_id = ?1 ORDER BY sort_index")?;
        let ids = stmt
            .query_map([course_id], |row| row.get(0))?
            .collect::<Result<Vec<String>, rusqlite::Error>>()?;
        Ok(ids)
    }

    pub fn lesson_thumbnail_path(&self, lesson_id: &str) -> Result<Option<String>, DbError> {
        let conn = self.conn.lock().expect("vault db lock");
        conn.query_row(
            "SELECT thumbnail_path FROM lesson WHERE id = ?1",
            [lesson_id],
            |row| row.get(0),
        )
        .optional()
        .map_err(DbError::from)
    }

    pub fn set_lesson_thumbnail_path(&self, lesson_id: &str, path: &str) -> Result<(), DbError> {
        let conn = self.conn.lock().expect("vault db lock");
        conn.execute(
            "UPDATE lesson SET thumbnail_path = ?1 WHERE id = ?2",
            rusqlite::params![path, lesson_id],
        )?;
        Ok(())
    }

    pub(crate) fn lesson_thumbnail_context(
        &self,
        lesson_id: &str,
    ) -> Result<Option<LessonThumbnailContextRow>, DbError> {
        let conn = self.conn.lock().expect("vault db lock");
        conn.query_row(
            "SELECT l.source_kind, sf.relative_path
             FROM lesson l
             LEFT JOIN source_file sf ON sf.id = l.source_file_id
             WHERE l.id = ?1",
            [lesson_id],
            |row| {
                Ok(LessonThumbnailContextRow {
                    source_kind: row.get(0)?,
                    media_relative_path: row.get(1)?,
                })
            },
        )
        .optional()
        .map_err(DbError::from)
    }

    pub fn anki_card_link_count_for_lesson(&self, lesson_id: &str) -> Result<i64, DbError> {
        let conn = self.conn.lock().expect("vault db lock");
        conn.query_row(
            "SELECT COUNT(*) FROM anki_card_link WHERE lesson_id = ?1",
            [lesson_id],
            |row| row.get(0),
        )
        .map_err(DbError::from)
    }

    pub fn lesson_resources_for_lesson(
        &self,
        lesson_id: &str,
    ) -> Result<Vec<(String, String)>, DbError> {
        Ok(self
            .lesson_resource_rows_for_lesson(lesson_id)?
            .into_iter()
            .map(|row| (row.relative_path, row.kind))
            .collect())
    }

    pub(crate) fn lesson_resource_rows_for_lesson(
        &self,
        lesson_id: &str,
    ) -> Result<Vec<LessonResourceRow>, DbError> {
        let conn = self.conn.lock().expect("vault db lock");
        let mut stmt = conn.prepare(
            "SELECT lr.id, sf.relative_path, lr.kind, c.root_path
             FROM lesson_resource lr
             JOIN source_file sf ON sf.id = lr.source_file_id
             JOIN lesson l ON l.id = lr.lesson_id
             JOIN course c ON c.id = l.course_id
             WHERE lr.lesson_id = ?1
             ORDER BY sf.relative_path",
        )?;
        let rows = stmt
            .query_map([lesson_id], |row| {
                Ok(LessonResourceRow {
                    resource_id: row.get(0)?,
                    relative_path: row.get(1)?,
                    kind: row.get(2)?,
                    course_root_path: row.get(3)?,
                })
            })?
            .collect::<Result<Vec<_>, rusqlite::Error>>()?;
        Ok(rows)
    }

    pub fn lesson_stable_keys_for_course(&self, course_id: &str) -> Result<Vec<String>, DbError> {
        let conn = self.conn.lock().expect("vault db lock");
        let mut stmt = conn.prepare(
            "SELECT lesson_stable_key FROM lesson WHERE course_id = ?1 ORDER BY sort_index",
        )?;
        let keys = stmt
            .query_map([course_id], |row| row.get(0))?
            .collect::<Result<Vec<String>, rusqlite::Error>>()?;
        Ok(keys)
    }

    pub fn list_course_ids(&self) -> Result<Vec<String>, DbError> {
        Ok(self
            .list_course_rows(crate::library::CourseLibraryVisibility::Active)?
            .into_iter()
            .map(|row| row.id)
            .collect())
    }

    pub fn set_progress_last_engaged_at(&self, lesson_id: &str, at: &str) -> Result<(), DbError> {
        let conn = self.conn.lock().expect("vault db lock");
        conn.execute(
            "UPDATE progress SET last_engaged_at = ?1 WHERE lesson_id = ?2",
            rusqlite::params![at, lesson_id],
        )?;
        Ok(())
    }

    pub(crate) fn most_recent_in_progress_continue_target(
        &self,
    ) -> Result<Option<crate::continue_learning::ContinueTarget>, DbError> {
        let conn = self.conn.lock().expect("vault db lock");
        let mut stmt = conn.prepare(
            "SELECT l.id, l.course_id
             FROM lesson l
             INNER JOIN progress p ON p.lesson_id = l.id
             INNER JOIN course c ON c.id = l.course_id
             WHERE p.status = 'in_progress'
             ORDER BY (p.last_engaged_at IS NULL), p.last_engaged_at DESC,
                      c.imported_at ASC, l.sort_index ASC
             LIMIT 1",
        )?;
        let row = stmt
            .query_row([], |row| {
                Ok(crate::continue_learning::ContinueTarget {
                    lesson_id: row.get(0)?,
                    course_id: row.get(1)?,
                })
            })
            .optional()?;
        Ok(row)
    }

    pub(crate) fn first_not_started_continue_target(
        &self,
    ) -> Result<Option<crate::continue_learning::ContinueTarget>, DbError> {
        for course_id in self.list_course_ids()? {
            for lesson_id in self.lesson_ids_in_learning_order(&course_id)? {
                let status = self.progress_for_lesson(&lesson_id)?.status;
                if status == LessonStatus::NotStarted {
                    return Ok(Some(crate::continue_learning::ContinueTarget {
                        course_id: course_id.clone(),
                        lesson_id,
                    }));
                }
            }
        }
        Ok(None)
    }

    pub(crate) fn list_course_rows(
        &self,
        visibility: crate::library::CourseLibraryVisibility,
    ) -> Result<Vec<CourseListRow>, DbError> {
        let conn = self.conn.lock().expect("vault db lock");
        let archived_clause = match visibility {
            crate::library::CourseLibraryVisibility::Active => "archived_at IS NULL",
            crate::library::CourseLibraryVisibility::Archived => "archived_at IS NOT NULL",
        };
        let sql = format!(
            "SELECT id, title, source_kind FROM course
             WHERE {archived_clause}
             ORDER BY imported_at"
        );
        let mut stmt = conn.prepare(&sql)?;
        let rows = stmt
            .query_map([], |row| {
                Ok(CourseListRow {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    source_kind: row.get(2)?,
                })
            })?
            .collect::<Result<Vec<CourseListRow>, rusqlite::Error>>()?;
        Ok(rows)
    }

    pub(crate) fn lesson_remaining_inputs_for_course(
        &self,
        course_id: &str,
    ) -> Result<Vec<crate::time::LessonRemainingInput>, DbError> {
        let conn = self.conn.lock().expect("vault db lock");
        let mut stmt = conn.prepare(
            "SELECT p.status, l.duration_ms
             FROM lesson l
             INNER JOIN progress p ON p.lesson_id = l.id
             WHERE l.course_id = ?1
             ORDER BY l.sort_index",
        )?;
        let rows = stmt
            .query_map([course_id], |row| {
                let status: String = row.get(0)?;
                Ok(crate::time::LessonRemainingInput {
                    status: LessonStatus::from_str(&status).map_err(|e| {
                        rusqlite::Error::FromSqlConversionFailure(
                            0,
                            rusqlite::types::Type::Text,
                            e.into(),
                        )
                    })?,
                    duration_ms: row.get(1)?,
                })
            })?
            .collect::<Result<Vec<_>, rusqlite::Error>>()?;
        Ok(rows)
    }

    pub(crate) fn roadmap_step_course_ids_ordered(
        &self,
        roadmap_id: &str,
    ) -> Result<Vec<Option<String>>, DbError> {
        let conn = self.conn.lock().expect("vault db lock");
        let mut stmt = conn.prepare(
            "SELECT course_id FROM roadmap_step
             WHERE roadmap_id = ?1
             ORDER BY sort_index",
        )?;
        let ids = stmt
            .query_map([roadmap_id], |row| row.get(0))?
            .collect::<Result<Vec<Option<String>>, rusqlite::Error>>()?;
        Ok(ids)
    }

    pub fn lesson_statuses_for_course(
        &self,
        course_id: &str,
    ) -> Result<Vec<LessonStatus>, DbError> {
        let conn = self.conn.lock().expect("vault db lock");
        let mut stmt = conn.prepare(
            "SELECT p.status FROM progress p
             INNER JOIN lesson l ON l.id = p.lesson_id
             WHERE l.course_id = ?1
             ORDER BY l.sort_index",
        )?;
        let statuses = stmt
            .query_map([course_id], |row| {
                let s: String = row.get(0)?;
                LessonStatus::from_str(&s).map_err(|e| {
                    rusqlite::Error::FromSqlConversionFailure(
                        0,
                        rusqlite::types::Type::Text,
                        e.into(),
                    )
                })
            })?
            .collect::<Result<Vec<LessonStatus>, rusqlite::Error>>()?;
        Ok(statuses)
    }

    pub fn local_lesson_sources_for_course(
        &self,
        course_id: &str,
    ) -> Result<Vec<LocalLessonSource>, DbError> {
        let conn = self.conn.lock().expect("vault db lock");
        let mut stmt = conn.prepare(
            "SELECT l.id, c.root_path, sf.relative_path
             FROM lesson l
             INNER JOIN course c ON c.id = l.course_id
             INNER JOIN source_file sf ON sf.id = l.source_file_id
             WHERE l.source_kind = 'local_file'
               AND c.root_path IS NOT NULL
               AND l.course_id = ?1",
        )?;
        let rows = stmt
            .query_map([course_id], |row| {
                Ok(LocalLessonSource {
                    lesson_id: row.get(0)?,
                    root_path: PathBuf::from(row.get::<_, String>(1)?),
                    relative_path: row.get(2)?,
                })
            })?
            .collect::<Result<Vec<_>, rusqlite::Error>>()?;
        Ok(rows)
    }

    pub fn local_lesson_source_for_lesson(
        &self,
        lesson_id: &str,
    ) -> Result<Option<LocalLessonSource>, DbError> {
        let conn = self.conn.lock().expect("vault db lock");
        let mut stmt = conn.prepare(
            "SELECT l.id, c.root_path, sf.relative_path
             FROM lesson l
             INNER JOIN course c ON c.id = l.course_id
             INNER JOIN source_file sf ON sf.id = l.source_file_id
             WHERE l.source_kind = 'local_file'
               AND c.root_path IS NOT NULL
               AND l.id = ?1",
        )?;
        stmt.query_row([lesson_id], |row| {
            Ok(LocalLessonSource {
                lesson_id: row.get(0)?,
                root_path: PathBuf::from(row.get::<_, String>(1)?),
                relative_path: row.get(2)?,
            })
        })
        .optional()
        .map_err(DbError::from)
    }

    pub fn local_lesson_sources(&self) -> Result<Vec<LocalLessonSource>, DbError> {
        let conn = self.conn.lock().expect("vault db lock");
        let mut stmt = conn.prepare(
            "SELECT l.id, c.root_path, sf.relative_path
             FROM lesson l
             INNER JOIN course c ON c.id = l.course_id
             INNER JOIN source_file sf ON sf.id = l.source_file_id
             WHERE l.source_kind = 'local_file' AND c.root_path IS NOT NULL",
        )?;
        let rows = stmt
            .query_map([], |row| {
                Ok(LocalLessonSource {
                    lesson_id: row.get(0)?,
                    root_path: PathBuf::from(row.get::<_, String>(1)?),
                    relative_path: row.get(2)?,
                })
            })?
            .collect::<Result<Vec<_>, rusqlite::Error>>()?;
        Ok(rows)
    }

    pub fn save_lesson_metadata(
        &self,
        lesson_id: &str,
        duration_ms: &crate::extract::ExtractorOutcome<i64>,
        page_count: &crate::extract::ExtractorOutcome<i64>,
        heading: &crate::extract::ExtractorOutcome<String>,
        subtitle: &crate::extract::SubtitleExtract,
    ) -> Result<(), DbError> {
        let conn = self.conn.lock().expect("vault db lock");
        let (duration_ms, duration_extractor, duration_error) = i64_outcome_columns(duration_ms);
        let (page_count, page_extractor, page_error) = i64_outcome_columns(page_count);
        let (heading, heading_extractor, heading_error) = string_outcome_columns(heading);
        let (subtitle_cue_count, subtitle_extractor, subtitle_error) =
            i64_outcome_columns(&subtitle.cue_count);
        let (subtitle_first_ms, _, _) = i64_outcome_columns(&subtitle.first_timestamp_ms);
        let (subtitle_last_ms, _, _) = i64_outcome_columns(&subtitle.last_timestamp_ms);

        conn.execute(
            "UPDATE lesson SET
                duration_ms = ?1, duration_extractor = ?2, duration_error = ?3,
                page_count = ?4, page_extractor = ?5, page_error = ?6,
                heading = ?7, heading_extractor = ?8, heading_error = ?9,
                subtitle_cue_count = ?10, subtitle_first_ms = ?11, subtitle_last_ms = ?12,
                subtitle_extractor = ?13, subtitle_error = ?14
             WHERE id = ?15",
            rusqlite::params![
                duration_ms,
                duration_extractor,
                duration_error,
                page_count,
                page_extractor,
                page_error,
                heading,
                heading_extractor,
                heading_error,
                subtitle_cue_count,
                subtitle_first_ms,
                subtitle_last_ms,
                subtitle_extractor,
                subtitle_error,
                lesson_id,
            ],
        )?;
        Ok(())
    }

    pub fn lesson_metadata(
        &self,
        lesson_id: &str,
    ) -> Result<crate::extract::LessonMetadataView, DbError> {
        let conn = self.conn.lock().expect("vault db lock");
        conn.query_row(
            "SELECT
                duration_ms, duration_extractor, duration_error,
                page_count, page_extractor, page_error,
                heading, heading_extractor, heading_error,
                subtitle_cue_count, subtitle_first_ms, subtitle_last_ms,
                subtitle_extractor, subtitle_error
             FROM lesson WHERE id = ?1",
            [lesson_id],
            |row| {
                Ok(crate::extract::LessonMetadataView {
                    duration_ms: read_i64_outcome(row, 0, 1, 2)?,
                    page_count: read_i64_outcome(row, 3, 4, 5)?,
                    heading: read_string_outcome(row, 6, 7, 8)?,
                    subtitle_cue_count: read_i64_outcome(row, 9, 12, 13)?,
                    subtitle_first_ms: read_i64_outcome(row, 10, 12, 13)?,
                    subtitle_last_ms: read_i64_outcome(row, 11, 12, 13)?,
                })
            },
        )
        .map_err(DbError::from)
    }

    pub fn subtitle_relative_path_for_lesson(
        &self,
        lesson_id: &str,
    ) -> Result<Option<String>, DbError> {
        let conn = self.conn.lock().expect("vault db lock");
        conn.query_row(
            "SELECT sf.relative_path
             FROM lesson l
             JOIN source_file sf ON sf.id = l.subtitle_file_id
             WHERE l.id = ?1",
            [lesson_id],
            |row| row.get(0),
        )
        .optional()
        .map_err(DbError::from)
    }

    pub fn course_id_for_lesson(&self, lesson_id: &str) -> Result<Option<String>, DbError> {
        let conn = self.conn.lock().expect("vault db lock");
        conn.query_row(
            "SELECT course_id FROM lesson WHERE id = ?1",
            [lesson_id],
            |row| row.get(0),
        )
        .optional()
        .map_err(DbError::from)
    }

    pub fn local_media_path_for_lesson(&self, lesson_id: &str) -> Result<Option<PathBuf>, DbError> {
        let conn = self.conn.lock().expect("vault db lock");
        let row = conn
            .query_row(
                "SELECT c.root_path, sf.relative_path
                 FROM lesson l
                 INNER JOIN course c ON c.id = l.course_id
                 INNER JOIN source_file sf ON sf.id = l.source_file_id
                 WHERE l.id = ?1 AND l.source_kind = 'local_file' AND c.root_path IS NOT NULL",
                [lesson_id],
                |row| Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?)),
            )
            .optional()?;
        Ok(row.map(|(root, relative)| PathBuf::from(root).join(relative)))
    }

    pub(crate) fn replace_lesson_transcription_cues(
        &self,
        lesson_id: &str,
        cues: &[crate::cues::SubtitleCue],
    ) -> Result<(), DbError> {
        let conn = self.conn.lock().expect("vault db lock");
        let tx = conn.unchecked_transaction()?;
        tx.execute(
            "DELETE FROM lesson_transcription_cue WHERE lesson_id = ?1 AND source = 'transcription'",
            [lesson_id],
        )?;
        for (sort_index, cue) in cues.iter().enumerate() {
            let id = Uuid::new_v4().to_string();
            tx.execute(
                "INSERT INTO lesson_transcription_cue
                 (id, lesson_id, sort_index, start_ms, end_ms, text, source)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, 'transcription')",
                rusqlite::params![
                    id,
                    lesson_id,
                    sort_index,
                    cue.start_ms,
                    cue.end_ms,
                    cue.text,
                ],
            )?;
        }
        tx.commit()?;
        Ok(())
    }

    pub(crate) fn list_lesson_transcription_cues(
        &self,
        lesson_id: &str,
    ) -> Result<Vec<crate::cues::SubtitleCue>, DbError> {
        let conn = self.conn.lock().expect("vault db lock");
        let mut stmt = conn.prepare(
            "SELECT start_ms, end_ms, text
             FROM lesson_transcription_cue
             WHERE lesson_id = ?1 AND source = 'transcription'
             ORDER BY sort_index",
        )?;
        let rows = stmt
            .query_map([lesson_id], |row| {
                Ok(crate::cues::SubtitleCue {
                    start_ms: row.get(0)?,
                    end_ms: row.get(1)?,
                    text: row.get(2)?,
                })
            })?
            .collect::<Result<Vec<_>, rusqlite::Error>>()?;
        Ok(rows)
    }

    pub fn save_lesson_chapters(
        &self,
        lesson_id: &str,
        outcome: &crate::chapters::ChapterIndexOutcome,
    ) -> Result<(), DbError> {
        let conn = self.conn.lock().expect("vault db lock");
        let tx = conn.unchecked_transaction()?;
        tx.execute(
            "DELETE FROM lesson_chapter WHERE lesson_id = ?1",
            [lesson_id],
        )?;
        match outcome {
            crate::chapters::ChapterIndexOutcome::Chapters(chapters) => {
                for (sort_index, chapter) in chapters.iter().enumerate() {
                    let id = Uuid::new_v4().to_string();
                    tx.execute(
                        "INSERT INTO lesson_chapter
                         (id, lesson_id, sort_index, start_ms, title, source)
                         VALUES (?1, ?2, ?3, ?4, ?5, 'ffprobe')",
                        rusqlite::params![
                            id,
                            lesson_id,
                            sort_index,
                            chapter.start_ms,
                            chapter.title,
                        ],
                    )?;
                }
                tx.execute(
                    "UPDATE lesson SET chapters_extractor = 'ffprobe', chapters_error = NULL
                     WHERE id = ?1",
                    [lesson_id],
                )?;
            }
            crate::chapters::ChapterIndexOutcome::Empty { reason } => {
                tx.execute(
                    "UPDATE lesson SET chapters_extractor = 'ffprobe', chapters_error = ?1
                     WHERE id = ?2",
                    rusqlite::params![reason, lesson_id],
                )?;
            }
        }
        tx.commit()?;
        Ok(())
    }

    pub fn list_lesson_chapters(
        &self,
        lesson_id: &str,
    ) -> Result<crate::chapters::LessonChapters, DbError> {
        let conn = self.conn.lock().expect("vault db lock");
        let reason: Option<String> = conn.query_row(
            "SELECT chapters_error FROM lesson WHERE id = ?1",
            [lesson_id],
            |row| row.get(0),
        )?;
        let mut stmt = conn.prepare(
            "SELECT start_ms, title FROM lesson_chapter
             WHERE lesson_id = ?1 ORDER BY sort_index",
        )?;
        let chapters = stmt
            .query_map([lesson_id], |row| {
                Ok(crate::chapters::MediaChapter {
                    start_ms: row.get(0)?,
                    title: row.get(1)?,
                })
            })?
            .collect::<Result<Vec<_>, rusqlite::Error>>()?;
        Ok(crate::chapters::LessonChapters {
            reason: if chapters.is_empty() { reason } else { None },
            chapters,
        })
    }

    pub(crate) fn lesson_subtitle_file(
        &self,
        lesson_id: &str,
    ) -> Result<Option<crate::cues::LessonSubtitleFile>, DbError> {
        let conn = self.conn.lock().expect("vault db lock");
        conn.query_row(
            "SELECT c.root_path, sf.relative_path
             FROM lesson l
             INNER JOIN course c ON c.id = l.course_id
             LEFT JOIN source_file sf ON sf.id = l.subtitle_file_id
             WHERE l.id = ?1",
            [lesson_id],
            |row| {
                Ok(crate::cues::LessonSubtitleFile {
                    course_root: PathBuf::from(row.get::<_, String>(0)?),
                    relative_path: row.get(1)?,
                })
            },
        )
        .optional()
        .map_err(DbError::from)
    }

    pub(crate) fn upsert_playlist_course(
        &self,
        definition_path: &str,
        definition: &crate::playlist::PlaylistDefinition,
    ) -> Result<crate::import::ImportReport, DbError> {
        let conn = self.conn.lock().expect("vault db lock");
        let now = now_iso8601();
        let course_id = match conn
            .query_row(
                "SELECT id FROM course WHERE root_path = ?1",
                [definition_path],
                |row| row.get::<_, String>(0),
            )
            .optional()?
        {
            Some(id) => {
                conn.execute(
                    "UPDATE course SET last_scanned_at = ?1, title = ?2, playlist_url = ?3
                     WHERE id = ?4",
                    rusqlite::params![now, definition.title, definition.playlist_url, id],
                )?;
                id
            }
            None => {
                let id = Uuid::new_v4().to_string();
                conn.execute(
                    "INSERT INTO course (
                        id, library_id, title, source_kind, root_path, playlist_url,
                        imported_at, last_scanned_at
                     ) VALUES (?1, ?2, ?3, 'playlist', ?4, ?5, ?6, ?6)",
                    rusqlite::params![
                        id,
                        DEFAULT_LIBRARY_ID,
                        definition.title,
                        definition_path,
                        definition.playlist_url,
                        now
                    ],
                )?;
                id
            }
        };

        let mut lesson_ids = Vec::new();
        let tx = conn.unchecked_transaction()?;
        for (sort_index, item) in definition.items.iter().enumerate() {
            let lesson_id = upsert_playlist_lesson(&tx, &course_id, item, sort_index as i32, &now)?;
            lesson_ids.push(lesson_id);
        }
        tx.commit()?;

        Ok(crate::import::ImportReport {
            course_id,
            lesson_ids,
        })
    }

    pub(crate) fn playlist_lesson_ids_missing_from_definition(
        &self,
        course_id: &str,
        active_external_ids: &std::collections::HashSet<&str>,
    ) -> Result<Vec<String>, DbError> {
        let conn = self.conn.lock().expect("vault db lock");
        let mut stmt = conn.prepare(
            "SELECT id, lesson_stable_key FROM lesson
             WHERE course_id = ?1 AND source_kind = 'playlist_item'",
        )?;
        let rows = stmt
            .query_map([course_id], |row| {
                Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
            })?
            .collect::<Result<Vec<(String, String)>, rusqlite::Error>>()?;
        Ok(rows
            .into_iter()
            .filter(|(_, key)| !active_external_ids.contains(key.as_str()))
            .map(|(id, _)| id)
            .collect())
    }

    pub fn marked_done_at(&self, lesson_id: &str) -> Result<Option<String>, DbError> {
        let conn = self.conn.lock().expect("vault db lock");
        conn.query_row(
            "SELECT marked_done_at FROM progress WHERE lesson_id = ?1",
            [lesson_id],
            |row| row.get(0),
        )
        .map_err(DbError::from)
    }

    pub(crate) fn set_marked_done_at(
        &self,
        lesson_id: &str,
        at: Option<String>,
    ) -> Result<(), DbError> {
        let conn = self.conn.lock().expect("vault db lock");
        conn.execute(
            "UPDATE progress SET marked_done_at = ?1 WHERE lesson_id = ?2",
            rusqlite::params![at, lesson_id],
        )?;
        Ok(())
    }

    pub(crate) fn save_lesson_progress_state(
        &self,
        lesson_id: &str,
        state: &crate::progress::LessonProgressState,
    ) -> Result<(), DbError> {
        let conn = self.conn.lock().expect("vault db lock");
        conn.execute(
            "UPDATE progress SET status = ?1, resume_position_ms = ?2, max_position_reached_ms = ?3,
             document_position = ?4
             WHERE lesson_id = ?5",
            rusqlite::params![
                state.status.as_db_str(),
                state.resume_position_ms,
                state.max_position_reached_ms,
                state.document_position,
                lesson_id,
            ],
        )?;
        Ok(())
    }

    pub fn watched_ranges_for_lesson(
        &self,
        lesson_id: &str,
    ) -> Result<Vec<crate::watched::WatchedRange>, DbError> {
        let conn = self.conn.lock().expect("vault db lock");
        let mut stmt = conn.prepare(
            "SELECT start_ms, end_ms FROM watched_range
             WHERE lesson_id = ?1
             ORDER BY start_ms",
        )?;
        let rows = stmt
            .query_map([lesson_id], |row| {
                Ok(crate::watched::WatchedRange {
                    start_ms: row.get(0)?,
                    end_ms: row.get(1)?,
                })
            })?
            .collect::<Result<Vec<_>, rusqlite::Error>>()?;
        Ok(rows)
    }

    pub fn watched_ms_for_lesson(&self, lesson_id: &str) -> Result<i64, DbError> {
        let ranges = self.watched_ranges_for_lesson(lesson_id)?;
        Ok(crate::watched::watched_ms(&ranges))
    }

    pub fn watched_percent_for_lesson(&self, lesson_id: &str) -> Result<Option<i64>, DbError> {
        let watched = self.watched_ms_for_lesson(lesson_id)?;
        let duration_ms = self.lesson_duration_ms(lesson_id)?;
        Ok(crate::watched::watched_percent(watched, duration_ms))
    }

    pub(crate) fn replace_watched_ranges(
        &self,
        lesson_id: &str,
        ranges: &[crate::watched::WatchedRange],
    ) -> Result<(), DbError> {
        let conn = self.conn.lock().expect("vault db lock");
        let tx = conn.unchecked_transaction()?;
        tx.execute(
            "DELETE FROM watched_range WHERE lesson_id = ?1",
            [lesson_id],
        )?;
        for range in ranges {
            tx.execute(
                "INSERT INTO watched_range (lesson_id, start_ms, end_ms) VALUES (?1, ?2, ?3)",
                rusqlite::params![lesson_id, range.start_ms, range.end_ms],
            )?;
        }
        tx.commit()?;
        Ok(())
    }

    pub fn lesson_duration_ms(&self, lesson_id: &str) -> Result<Option<i64>, DbError> {
        let conn = self.conn.lock().expect("vault db lock");
        conn.query_row(
            "SELECT duration_ms FROM lesson WHERE id = ?1",
            [lesson_id],
            |row| row.get(0),
        )
        .map_err(DbError::from)
    }

    pub fn progress_for_lesson(&self, lesson_id: &str) -> Result<LessonProgress, DbError> {
        let conn = self.conn.lock().expect("vault db lock");
        let (status, resume_position_ms, max_position_reached_ms, document_position) = conn
            .query_row(
                "SELECT status, resume_position_ms, max_position_reached_ms, document_position
             FROM progress WHERE lesson_id = ?1",
                [lesson_id],
                |row| {
                    Ok((
                        row.get::<_, String>(0)?,
                        row.get::<_, i64>(1)?,
                        row.get::<_, i64>(2)?,
                        row.get::<_, i64>(3)?,
                    ))
                },
            )?;
        Ok(LessonProgress {
            status: LessonStatus::from_str(&status)?,
            resume_position_ms,
            max_position_reached_ms,
            document_position,
        })
    }

    pub(crate) fn replace_order_proposal(
        &self,
        course_id: &str,
        lesson_ids: &[String],
    ) -> Result<String, DbError> {
        let conn = self.conn.lock().expect("vault db lock");
        let tx = conn.unchecked_transaction()?;
        tx.execute(
            "DELETE FROM order_proposal WHERE course_id = ?1",
            [course_id],
        )?;
        let proposal_id = Uuid::new_v4().to_string();
        let now = now_iso8601();
        tx.execute(
            "INSERT INTO order_proposal (id, course_id, created_at) VALUES (?1, ?2, ?3)",
            rusqlite::params![proposal_id, course_id, now],
        )?;
        for (position, lesson_id) in lesson_ids.iter().enumerate() {
            tx.execute(
                "INSERT INTO order_proposal_entry (proposal_id, lesson_id, position)
                 VALUES (?1, ?2, ?3)",
                rusqlite::params![proposal_id, lesson_id, position as i32],
            )?;
        }
        tx.commit()?;
        Ok(proposal_id)
    }

    pub(crate) fn order_proposal_lesson_ids(
        &self,
        proposal_id: &str,
    ) -> Result<Option<(String, Vec<String>)>, DbError> {
        let conn = self.conn.lock().expect("vault db lock");
        let course_id: Option<String> = conn
            .query_row(
                "SELECT course_id FROM order_proposal WHERE id = ?1",
                [proposal_id],
                |row| row.get(0),
            )
            .optional()?;
        let Some(course_id) = course_id else {
            return Ok(None);
        };
        let mut stmt = conn.prepare(
            "SELECT lesson_id FROM order_proposal_entry
             WHERE proposal_id = ?1
             ORDER BY position",
        )?;
        let lesson_ids = stmt
            .query_map([proposal_id], |row| row.get(0))?
            .collect::<Result<Vec<String>, rusqlite::Error>>()?;
        Ok(Some((course_id, lesson_ids)))
    }

    pub(crate) fn apply_learning_order(
        &self,
        course_id: &str,
        lesson_ids: &[String],
    ) -> Result<(), DbError> {
        let conn = self.conn.lock().expect("vault db lock");
        let tx = conn.unchecked_transaction()?;
        for (sort_index, lesson_id) in lesson_ids.iter().enumerate() {
            let updated = tx.execute(
                "UPDATE lesson SET sort_index = ?1 WHERE id = ?2 AND course_id = ?3",
                rusqlite::params![sort_index as i32, lesson_id, course_id],
            )?;
            if updated != 1 {
                return Err(DbError::InvalidData(format!(
                    "lesson {lesson_id} not in course {course_id}"
                )));
            }
        }
        tx.commit()?;
        Ok(())
    }

    pub(crate) fn delete_order_proposal(&self, proposal_id: &str) -> Result<bool, DbError> {
        let conn = self.conn.lock().expect("vault db lock");
        let deleted = conn.execute("DELETE FROM order_proposal WHERE id = ?1", [proposal_id])?;
        Ok(deleted > 0)
    }

    pub(crate) fn load_vault_settings(
        &self,
    ) -> Result<crate::vault_settings::VaultSettings, DbError> {
        let conn = self.conn.lock().expect("vault db lock");
        let settings = conn.query_row(
            "SELECT tesseract_language, transcription_model_path, anki_desktop_binary_path, ui_theme, playback_speed, player_volume, player_muted
             FROM vault_settings WHERE id = 1",
            [],
            |row| {
                Ok(crate::vault_settings::VaultSettings {
                    tesseract_language: row.get(0)?,
                    transcription_model_path: row.get(1)?,
                    anki_desktop_binary_path: row.get(2)?,
                    ui_theme: crate::vault_settings::UiTheme::parse(
                        row.get::<_, String>(3)?.as_str(),
                    ),
                    playback_speed: crate::vault_settings::PlaybackSpeed::parse(
                        row.get::<_, f64>(4)?,
                    ),
                    player_volume: crate::vault_settings::normalize_player_volume(
                        row.get::<_, f64>(5)?,
                    ),
                    player_muted: row.get::<_, i64>(6)? != 0,
                })
            },
        )?;
        Ok(settings)
    }

    pub(crate) fn save_vault_settings(
        &self,
        settings: &crate::vault_settings::VaultSettings,
    ) -> Result<(), DbError> {
        let conn = self.conn.lock().expect("vault db lock");
        let updated = conn.execute(
            "UPDATE vault_settings
             SET tesseract_language = ?1,
                 transcription_model_path = ?2,
                 anki_desktop_binary_path = ?3,
                 ui_theme = ?4,
                 playback_speed = ?5,
                 player_volume = ?6,
                 player_muted = ?7
             WHERE id = 1",
            rusqlite::params![
                settings.tesseract_language,
                settings.transcription_model_path,
                settings.anki_desktop_binary_path,
                settings.ui_theme.as_str(),
                settings.playback_speed.value(),
                settings.player_volume,
                i64::from(settings.player_muted),
            ],
        )?;
        if updated == 0 {
            return Err(DbError::InvalidData("vault_settings row missing".into()));
        }
        Ok(())
    }

    pub(crate) fn insert_study_day(&self, local_date: &str) -> Result<(), DbError> {
        let conn = self.conn.lock().expect("vault db lock");
        conn.execute(
            "INSERT OR IGNORE INTO study_day (local_date) VALUES (?1)",
            [local_date],
        )?;
        Ok(())
    }

    pub(crate) fn study_day_exists(&self, local_date: &str) -> Result<bool, DbError> {
        let conn = self.conn.lock().expect("vault db lock");
        let count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM study_day WHERE local_date = ?1",
            [local_date],
            |row| row.get(0),
        )?;
        Ok(count > 0)
    }

    pub(crate) fn list_study_days(&self) -> Result<Vec<String>, DbError> {
        let conn = self.conn.lock().expect("vault db lock");
        let mut stmt = conn.prepare("SELECT local_date FROM study_day ORDER BY local_date ASC")?;
        let rows = stmt.query_map([], |row| row.get(0))?;
        rows.collect::<Result<Vec<_>, _>>().map_err(DbError::from)
    }

    pub(crate) fn count_lessons_marked_done_between(
        &self,
        start_unix: i64,
        end_unix: i64,
    ) -> Result<u32, DbError> {
        let conn = self.conn.lock().expect("vault db lock");
        let count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM progress
             WHERE marked_done_at IS NOT NULL
               AND CAST(marked_done_at AS INTEGER) >= ?1
               AND CAST(marked_done_at AS INTEGER) <= ?2",
            rusqlite::params![start_unix, end_unix],
            |row| row.get(0),
        )?;
        Ok(u32::try_from(count).unwrap_or(u32::MAX))
    }

    pub(crate) fn insert_card_grade_event(&self, graded_at: &str) -> Result<(), DbError> {
        let conn = self.conn.lock().expect("vault db lock");
        conn.execute(
            "INSERT INTO card_grade_event (graded_at) VALUES (?1)",
            [graded_at],
        )?;
        Ok(())
    }

    pub(crate) fn count_card_grades_between(
        &self,
        start_unix: i64,
        end_unix: i64,
    ) -> Result<u32, DbError> {
        let conn = self.conn.lock().expect("vault db lock");
        let count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM card_grade_event
             WHERE CAST(graded_at AS INTEGER) >= ?1
               AND CAST(graded_at AS INTEGER) <= ?2",
            rusqlite::params![start_unix, end_unix],
            |row| row.get(0),
        )?;
        Ok(u32::try_from(count).unwrap_or(u32::MAX))
    }

    pub(crate) fn total_watched_ms_all_lessons(&self) -> Result<i64, DbError> {
        let conn = self.conn.lock().expect("vault db lock");
        let mut stmt = conn.prepare("SELECT DISTINCT lesson_id FROM watched_range")?;
        let lesson_ids = stmt
            .query_map([], |row| row.get::<_, String>(0))?
            .collect::<Result<Vec<_>, _>>()?;
        let mut total = 0i64;
        for lesson_id in lesson_ids {
            total += self.watched_ms_for_lesson(&lesson_id)?;
        }
        Ok(total)
    }

    pub fn course_exists(&self, course_id: &str) -> Result<bool, DbError> {
        let conn = self.conn.lock().expect("vault db lock");
        let id: Option<String> = conn
            .query_row("SELECT id FROM course WHERE id = ?1", [course_id], |row| {
                row.get(0)
            })
            .optional()?;
        Ok(id.is_some())
    }

    pub(crate) fn course_is_archived(&self, course_id: &str) -> Result<bool, DbError> {
        let conn = self.conn.lock().expect("vault db lock");
        let archived_at = conn
            .query_row(
                "SELECT archived_at FROM course WHERE id = ?1",
                [course_id],
                |row| row.get::<_, Option<String>>(0),
            )
            .optional()?;
        Ok(archived_at.flatten().is_some())
    }

    pub(crate) fn archive_course_row(&self, course_id: &str) -> Result<bool, DbError> {
        let conn = self.conn.lock().expect("vault db lock");
        let updated = conn.execute(
            "UPDATE course SET archived_at = ?1 WHERE id = ?2 AND archived_at IS NULL",
            rusqlite::params![now_iso8601(), course_id],
        )?;
        Ok(updated > 0)
    }

    pub(crate) fn unarchive_course_row(&self, course_id: &str) -> Result<bool, DbError> {
        let conn = self.conn.lock().expect("vault db lock");
        let updated = conn.execute(
            "UPDATE course SET archived_at = NULL WHERE id = ?1 AND archived_at IS NOT NULL",
            [course_id],
        )?;
        Ok(updated > 0)
    }

    pub(crate) fn list_roadmaps(&self) -> Result<Vec<RoadmapListRow>, DbError> {
        let conn = self.conn.lock().expect("vault db lock");
        let mut stmt = conn.prepare(
            "SELECT id, title FROM roadmap ORDER BY updated_at DESC, title COLLATE NOCASE",
        )?;
        let rows = stmt
            .query_map([], |row| {
                Ok(RoadmapListRow {
                    id: row.get(0)?,
                    title: row.get(1)?,
                })
            })?
            .collect::<Result<Vec<_>, rusqlite::Error>>()?;
        Ok(rows)
    }

    pub(crate) fn roadmap_title(&self, roadmap_id: &str) -> Result<Option<String>, DbError> {
        let conn = self.conn.lock().expect("vault db lock");
        conn.query_row(
            "SELECT title FROM roadmap WHERE id = ?1",
            [roadmap_id],
            |row| row.get(0),
        )
        .optional()
        .map_err(DbError::from)
    }

    pub(crate) fn update_roadmap_step_course_id(
        &self,
        step_id: &str,
        course_id: &str,
    ) -> Result<bool, DbError> {
        let conn = self.conn.lock().expect("vault db lock");
        let updated = conn.execute(
            "UPDATE roadmap_step SET course_id = ?1 WHERE id = ?2",
            rusqlite::params![course_id, step_id],
        )?;
        Ok(updated > 0)
    }

    pub(crate) fn update_roadmap_step_note(
        &self,
        step_id: &str,
        note: Option<&str>,
    ) -> Result<bool, DbError> {
        let conn = self.conn.lock().expect("vault db lock");
        let updated = conn.execute(
            "UPDATE roadmap_step SET note = ?1 WHERE id = ?2",
            rusqlite::params![note, step_id],
        )?;
        Ok(updated > 0)
    }

    pub(crate) fn insert_roadmap(&self, title: &str) -> Result<String, DbError> {
        let conn = self.conn.lock().expect("vault db lock");
        let id = Uuid::new_v4().to_string();
        let now = now_iso8601();
        conn.execute(
            "INSERT INTO roadmap (id, library_id, title, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?4)",
            rusqlite::params![id, DEFAULT_LIBRARY_ID, title, now],
        )?;
        Ok(id)
    }

    pub(crate) fn insert_roadmap_step(
        &self,
        roadmap_id: &str,
        course_id: &str,
        note: Option<&str>,
    ) -> Result<String, DbError> {
        let conn = self.conn.lock().expect("vault db lock");
        let next_index: i32 = conn.query_row(
            "SELECT COALESCE(MAX(sort_index), -1) + 1 FROM roadmap_step WHERE roadmap_id = ?1",
            [roadmap_id],
            |row| row.get(0),
        )?;
        let id = Uuid::new_v4().to_string();
        conn.execute(
            "INSERT INTO roadmap_step (id, roadmap_id, course_id, note, sort_index)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            rusqlite::params![id, roadmap_id, course_id, note, next_index],
        )?;
        Ok(id)
    }

    pub(crate) fn roadmap_step_ids_in_order(
        &self,
        roadmap_id: &str,
    ) -> Result<Vec<String>, DbError> {
        let conn = self.conn.lock().expect("vault db lock");
        let mut stmt =
            conn.prepare("SELECT id FROM roadmap_step WHERE roadmap_id = ?1 ORDER BY sort_index")?;
        let ids = stmt
            .query_map([roadmap_id], |row| row.get(0))?
            .collect::<Result<Vec<String>, rusqlite::Error>>()?;
        Ok(ids)
    }

    pub(crate) fn roadmap_steps_ordered(
        &self,
        roadmap_id: &str,
    ) -> Result<Vec<RoadmapStepRow>, DbError> {
        let conn = self.conn.lock().expect("vault db lock");
        let mut stmt = conn.prepare(
            "SELECT rs.id, rs.course_id, COALESCE(c.title, ''), rs.note
             FROM roadmap_step rs
             LEFT JOIN course c ON c.id = rs.course_id
             WHERE rs.roadmap_id = ?1
             ORDER BY rs.sort_index",
        )?;
        let rows = stmt
            .query_map([roadmap_id], |row| {
                Ok(RoadmapStepRow {
                    id: row.get(0)?,
                    course_id: row.get(1)?,
                    course_title: row.get(2)?,
                    note: row.get(3)?,
                })
            })?
            .collect::<Result<Vec<_>, rusqlite::Error>>()?;
        Ok(rows)
    }

    pub(crate) fn roadmap_step_course_id(
        &self,
        step_id: &str,
    ) -> Result<Option<Option<String>>, DbError> {
        let conn = self.conn.lock().expect("vault db lock");
        conn.query_row(
            "SELECT course_id FROM roadmap_step WHERE id = ?1",
            [step_id],
            |row| row.get(0),
        )
        .optional()
        .map_err(DbError::from)
    }

    pub(crate) fn apply_roadmap_step_order(
        &self,
        roadmap_id: &str,
        step_ids: &[String],
    ) -> Result<(), DbError> {
        let conn = self.conn.lock().expect("vault db lock");
        let tx = conn.unchecked_transaction()?;
        for (sort_index, step_id) in step_ids.iter().enumerate() {
            let updated = tx.execute(
                "UPDATE roadmap_step SET sort_index = ?1 WHERE id = ?2 AND roadmap_id = ?3",
                rusqlite::params![sort_index as i32, step_id, roadmap_id],
            )?;
            if updated != 1 {
                return Err(DbError::InvalidData(format!(
                    "step {step_id} not in roadmap {roadmap_id}"
                )));
            }
        }
        tx.commit()?;
        Ok(())
    }

    pub(crate) fn delete_roadmap_step(&self, step_id: &str) -> Result<bool, DbError> {
        let conn = self.conn.lock().expect("vault db lock");
        let deleted = conn.execute("DELETE FROM roadmap_step WHERE id = ?1", [step_id])?;
        Ok(deleted > 0)
    }

    pub(crate) fn delete_course(&self, course_id: &str) -> Result<bool, DbError> {
        let conn = self.conn.lock().expect("vault db lock");
        conn.execute(
            "UPDATE anki_card_link SET link_status = 'broken'
             WHERE lesson_id IN (SELECT id FROM lesson WHERE course_id = ?1)",
            [course_id],
        )?;
        let deleted = conn.execute("DELETE FROM course WHERE id = ?1", [course_id])?;
        Ok(deleted > 0)
    }

    pub(crate) fn roadmap_exists(&self, roadmap_id: &str) -> Result<bool, DbError> {
        let conn = self.conn.lock().expect("vault db lock");
        let id: Option<String> = conn
            .query_row(
                "SELECT id FROM roadmap WHERE id = ?1",
                [roadmap_id],
                |row| row.get(0),
            )
            .optional()?;
        Ok(id.is_some())
    }

    pub(crate) fn roadmap_id_for_step(&self, step_id: &str) -> Result<Option<String>, DbError> {
        let conn = self.conn.lock().expect("vault db lock");
        conn.query_row(
            "SELECT roadmap_id FROM roadmap_step WHERE id = ?1",
            [step_id],
            |row| row.get(0),
        )
        .optional()
        .map_err(DbError::from)
    }

    pub(crate) fn insert_prerequisite_edge(
        &self,
        from_step_id: &str,
        to_step_id: &str,
    ) -> Result<(), DbError> {
        let conn = self.conn.lock().expect("vault db lock");
        conn.execute(
            "INSERT INTO roadmap_prerequisite_edge (from_step_id, to_step_id)
             VALUES (?1, ?2)",
            rusqlite::params![from_step_id, to_step_id],
        )?;
        Ok(())
    }

    pub(crate) fn delete_prerequisite_edge(
        &self,
        from_step_id: &str,
        to_step_id: &str,
    ) -> Result<bool, DbError> {
        let conn = self.conn.lock().expect("vault db lock");
        let changed = conn.execute(
            "DELETE FROM roadmap_prerequisite_edge
             WHERE from_step_id = ?1 AND to_step_id = ?2",
            rusqlite::params![from_step_id, to_step_id],
        )?;
        Ok(changed > 0)
    }

    pub(crate) fn prerequisite_edges_for_roadmap(
        &self,
        roadmap_id: &str,
    ) -> Result<Vec<(String, String)>, DbError> {
        let conn = self.conn.lock().expect("vault db lock");
        let mut stmt = conn.prepare(
            "SELECT e.from_step_id, e.to_step_id
             FROM roadmap_prerequisite_edge e
             INNER JOIN roadmap_step s ON s.id = e.from_step_id
             WHERE s.roadmap_id = ?1",
        )?;
        let rows = stmt
            .query_map([roadmap_id], |row| Ok((row.get(0)?, row.get(1)?)))?
            .collect::<Result<Vec<_>, rusqlite::Error>>()?;
        Ok(rows)
    }

    pub(crate) fn lesson_card_context(
        &self,
        lesson_id: &str,
    ) -> Result<Option<crate::card::LessonCardContextRow>, DbError> {
        let conn = self.conn.lock().expect("vault db lock");
        conn.query_row(
            "SELECT c.title, l.display_title,
                    COALESCE(sf.relative_path, pi.url, '')
             FROM lesson l
             INNER JOIN course c ON c.id = l.course_id
             LEFT JOIN source_file sf ON sf.id = l.source_file_id
             LEFT JOIN playlist_item pi ON pi.id = l.playlist_item_id
             WHERE l.id = ?1",
            [lesson_id],
            |row| {
                Ok(crate::card::LessonCardContextRow {
                    course_title: row.get(0)?,
                    lesson_title: row.get(1)?,
                    source_label: row.get(2)?,
                })
            },
        )
        .optional()
        .map_err(DbError::from)
    }

    pub(crate) fn insert_anki_card_link(
        &self,
        lesson_id: &str,
        timestamp_ms: i64,
        front_hash: &str,
        front_preview: &str,
        anki_note_id: i64,
    ) -> Result<String, DbError> {
        let conn = self.conn.lock().expect("vault db lock");
        let id = Uuid::new_v4().to_string();
        let now = now_iso8601();
        conn.execute(
            "INSERT INTO anki_card_link (
                id, lesson_id, timestamp_ms, front_hash, front_preview, anki_note_id, link_status, created_at
             ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, 'active', ?7)",
            rusqlite::params![
                id,
                lesson_id,
                timestamp_ms,
                front_hash,
                front_preview,
                anki_note_id,
                now
            ],
        )?;
        Ok(id)
    }

    pub(crate) fn list_lesson_card_links(
        &self,
        lesson_id: &str,
    ) -> Result<Vec<crate::anki_link::LessonCardLinkEntry>, DbError> {
        let conn = self.conn.lock().expect("vault db lock");
        let mut stmt = conn.prepare(
            "SELECT anki_note_id, timestamp_ms, front_preview
             FROM anki_card_link
             WHERE lesson_id = ?1
             ORDER BY timestamp_ms ASC",
        )?;
        let rows = stmt
            .query_map([lesson_id], |row| {
                Ok(crate::anki_link::LessonCardLinkEntry {
                    anki_note_id: row.get(0)?,
                    timestamp_ms: row.get(1)?,
                    front_preview: row.get(2)?,
                })
            })?
            .collect::<Result<Vec<_>, rusqlite::Error>>()?;
        Ok(rows)
    }

    pub fn anki_note_ids_for_course(
        &self,
        course_id: &str,
    ) -> Result<std::collections::HashSet<i64>, DbError> {
        let conn = self.conn.lock().expect("vault db lock");
        let mut stmt = conn.prepare(
            "SELECT DISTINCT acl.anki_note_id
             FROM anki_card_link acl
             INNER JOIN lesson l ON l.id = acl.lesson_id
             WHERE l.course_id = ?1 AND acl.link_status = 'active'",
        )?;
        let ids = stmt
            .query_map([course_id], |row| row.get(0))?
            .collect::<Result<std::collections::HashSet<i64>, rusqlite::Error>>()?;
        Ok(ids)
    }

    pub(crate) fn delete_anki_card_link(
        &self,
        lesson_id: &str,
        anki_note_id: i64,
    ) -> Result<bool, DbError> {
        let conn = self.conn.lock().expect("vault db lock");
        let deleted = conn.execute(
            "DELETE FROM anki_card_link WHERE lesson_id = ?1 AND anki_note_id = ?2",
            rusqlite::params![lesson_id, anki_note_id],
        )?;
        Ok(deleted > 0)
    }

    pub(crate) fn anki_card_link_for_selection(
        &self,
        lesson_id: &str,
        timestamp_ms: i64,
        front_hash: &str,
    ) -> Result<Option<crate::anki_link::AnkiCardLinkRecord>, DbError> {
        let conn = self.conn.lock().expect("vault db lock");
        conn.query_row(
            "SELECT id, lesson_id, timestamp_ms, front_hash, front_preview, anki_note_id, link_status
             FROM anki_card_link
             WHERE lesson_id = ?1 AND timestamp_ms = ?2 AND front_hash = ?3",
            rusqlite::params![lesson_id, timestamp_ms, front_hash],
            |row| {
                let status: String = row.get(6)?;
                Ok(crate::anki_link::AnkiCardLinkRecord {
                    id: row.get(0)?,
                    lesson_id: row.get::<_, Option<String>>(1)?,
                    timestamp_ms: row.get(2)?,
                    front_hash: row.get(3)?,
                    front_preview: row.get(4)?,
                    anki_note_id: row.get(5)?,
                    link_status: crate::anki_link::parse_link_status(&status).map_err(|e| {
                        rusqlite::Error::FromSqlConversionFailure(
                            6,
                            rusqlite::types::Type::Text,
                            e.into(),
                        )
                    })?,
                })
            },
        )
        .optional()
        .map_err(DbError::from)
    }

    pub(crate) fn anki_card_link_by_id(
        &self,
        link_id: &str,
    ) -> Result<Option<crate::anki_link::AnkiCardLinkRecord>, DbError> {
        let conn = self.conn.lock().expect("vault db lock");
        conn.query_row(
            "SELECT id, lesson_id, timestamp_ms, front_hash, front_preview, anki_note_id, link_status
             FROM anki_card_link WHERE id = ?1",
            [link_id],
            |row| {
                let status: String = row.get(6)?;
                Ok(crate::anki_link::AnkiCardLinkRecord {
                    id: row.get(0)?,
                    lesson_id: row.get::<_, Option<String>>(1)?,
                    timestamp_ms: row.get(2)?,
                    front_hash: row.get(3)?,
                    front_preview: row.get(4)?,
                    anki_note_id: row.get(5)?,
                    link_status: crate::anki_link::parse_link_status(&status).map_err(|e| {
                        rusqlite::Error::FromSqlConversionFailure(
                            6,
                            rusqlite::types::Type::Text,
                            e.into(),
                        )
                    })?,
                })
            },
        )
        .optional()
        .map_err(DbError::from)
    }

    pub(crate) fn roadmap_graph_step_rows(
        &self,
        roadmap_id: &str,
    ) -> Result<Vec<RoadmapGraphStepRow>, DbError> {
        let conn = self.conn.lock().expect("vault db lock");
        let mut stmt = conn.prepare(
            "SELECT rs.id, rs.course_id, COALESCE(c.title, '')
             FROM roadmap_step rs
             LEFT JOIN course c ON c.id = rs.course_id
             WHERE rs.roadmap_id = ?1
             ORDER BY rs.sort_index",
        )?;
        let rows = stmt
            .query_map([roadmap_id], |row| {
                Ok(RoadmapGraphStepRow {
                    step_id: row.get(0)?,
                    course_id: row.get(1)?,
                    course_title: row.get(2)?,
                })
            })?
            .collect::<Result<Vec<_>, rusqlite::Error>>()?;
        Ok(rows)
    }
}

fn migrate(conn: &Connection) -> Result<(), DbError> {
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS _schema_migrations (
            version TEXT PRIMARY KEY NOT NULL,
            applied_at TEXT NOT NULL
        );",
    )?;
    for (version, sql) in MIGRATIONS {
        let already: Option<String> = conn
            .query_row(
                "SELECT version FROM _schema_migrations WHERE version = ?1",
                [*version],
                |row| row.get(0),
            )
            .optional()?;
        if already.is_some() {
            continue;
        }
        conn.execute_batch(sql)?;
        conn.execute(
            "INSERT INTO _schema_migrations (version, applied_at) VALUES (?1, ?2)",
            rusqlite::params![version, now_iso8601()],
        )?;
    }
    Ok(())
}

fn ensure_default_library(conn: &Connection) -> Result<(), DbError> {
    let exists: Option<String> = conn
        .query_row(
            "SELECT id FROM library WHERE id = ?1",
            [DEFAULT_LIBRARY_ID],
            |row| row.get(0),
        )
        .optional()?;
    if exists.is_some() {
        return Ok(());
    }
    let now = now_iso8601();
    conn.execute(
        "INSERT INTO library (id, display_name, created_at, updated_at) VALUES (?1, ?2, ?3, ?3)",
        rusqlite::params![DEFAULT_LIBRARY_ID, "Lessonbinder", now],
    )?;
    Ok(())
}

fn source_kind_for_extension(ext: &str) -> &'static str {
    match ext {
        "mp4" | "mkv" | "webm" => "video",
        "mp3" | "m4a" => "audio",
        "pdf" => "pdf",
        "srt" | "vtt" => "subtitle",
        "png" | "jpg" | "jpeg" => "text",
        _ => "text",
    }
}

fn upsert_playlist_lesson(
    conn: &Connection,
    course_id: &str,
    item: &crate::playlist::PlaylistItemDefinition,
    sort_index: i32,
    _now: &str,
) -> Result<String, DbError> {
    let playlist_item_id = upsert_playlist_item(conn, course_id, item, sort_index)?;

    let existing: Option<String> = conn
        .query_row(
            "SELECT id FROM lesson WHERE course_id = ?1 AND lesson_stable_key = ?2",
            rusqlite::params![course_id, item.external_id],
            |row| row.get(0),
        )
        .optional()?;

    if let Some(lesson_id) = existing {
        conn.execute(
            "UPDATE lesson SET display_title = ?1, sort_index = ?2, playlist_item_id = ?3, duration_ms = ?4
             WHERE id = ?5",
            rusqlite::params![
                item.title,
                sort_index,
                playlist_item_id,
                item.duration_ms,
                lesson_id
            ],
        )?;
        Ok(lesson_id)
    } else {
        let lesson_id = Uuid::new_v4().to_string();
        conn.execute(
            "INSERT INTO lesson (
                id, course_id, lesson_stable_key, display_title, sort_index,
                source_kind, playlist_item_id, duration_ms
             ) VALUES (?1, ?2, ?3, ?4, ?5, 'playlist_item', ?6, ?7)",
            rusqlite::params![
                lesson_id,
                course_id,
                item.external_id,
                item.title,
                sort_index,
                playlist_item_id,
                item.duration_ms
            ],
        )?;
        Ok(lesson_id)
    }
}

fn upsert_playlist_item(
    conn: &Connection,
    course_id: &str,
    item: &crate::playlist::PlaylistItemDefinition,
    ordinal: i32,
) -> Result<String, DbError> {
    let existing: Option<String> = conn
        .query_row(
            "SELECT id FROM playlist_item WHERE course_id = ?1 AND external_id = ?2",
            rusqlite::params![course_id, item.external_id],
            |row| row.get(0),
        )
        .optional()?;

    if let Some(id) = existing {
        conn.execute(
            "UPDATE playlist_item SET ordinal = ?1, url = ?2, title = ?3, duration_ms = ?4
             WHERE id = ?5",
            rusqlite::params![ordinal, item.url, item.title, item.duration_ms, id],
        )?;
        Ok(id)
    } else {
        let id = Uuid::new_v4().to_string();
        conn.execute(
            "INSERT INTO playlist_item (id, course_id, external_id, ordinal, url, title, duration_ms)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            rusqlite::params![
                id,
                course_id,
                item.external_id,
                ordinal,
                item.url,
                item.title,
                item.duration_ms
            ],
        )?;
        Ok(id)
    }
}

fn upsert_lesson_from_scan(
    conn: &Connection,
    course_id: &str,
    course_root: &Path,
    lesson: &crate::import::ScannedLesson,
    sort_index: i32,
    now: &str,
) -> Result<String, DbError> {
    let source_file_id =
        upsert_source_file(conn, course_id, course_root, &lesson.relative_path, now)?;
    let subtitle_file_id = if let Some(sub_path) = &lesson.subtitle_relative_path {
        Some(upsert_source_file(
            conn,
            course_id,
            course_root,
            sub_path,
            now,
        )?)
    } else {
        None
    };

    let existing: Option<String> = conn
        .query_row(
            "SELECT id FROM lesson WHERE course_id = ?1 AND lesson_stable_key = ?2",
            rusqlite::params![course_id, lesson.relative_path],
            |row| row.get(0),
        )
        .optional()?;

    if let Some(lesson_id) = existing {
        conn.execute(
            "UPDATE lesson SET display_title = ?1, sort_index = ?2, source_file_id = ?3, subtitle_file_id = ?4
             WHERE id = ?5",
            rusqlite::params![
                lesson.display_title,
                sort_index,
                source_file_id,
                subtitle_file_id,
                lesson_id
            ],
        )?;
        sync_lesson_resources(conn, &lesson_id, course_id, course_root, lesson, now)?;
        Ok(lesson_id)
    } else {
        let lesson_id = Uuid::new_v4().to_string();
        conn.execute(
            "INSERT INTO lesson (
                id, course_id, lesson_stable_key, display_title, sort_index,
                source_kind, source_file_id, subtitle_file_id
             ) VALUES (?1, ?2, ?3, ?4, ?5, 'local_file', ?6, ?7)",
            rusqlite::params![
                lesson_id,
                course_id,
                lesson.relative_path,
                lesson.display_title,
                sort_index,
                source_file_id,
                subtitle_file_id
            ],
        )?;
        sync_lesson_resources(conn, &lesson_id, course_id, course_root, lesson, now)?;
        Ok(lesson_id)
    }
}

fn sync_lesson_resources(
    conn: &Connection,
    lesson_id: &str,
    course_id: &str,
    course_root: &Path,
    lesson: &crate::import::ScannedLesson,
    now: &str,
) -> Result<(), DbError> {
    let desired_paths: std::collections::HashSet<String> = lesson
        .resources
        .iter()
        .map(|resource| resource.relative_path.clone())
        .collect();

    let mut stmt = conn.prepare(
        "SELECT lr.id, sf.relative_path
         FROM lesson_resource lr
         JOIN source_file sf ON sf.id = lr.source_file_id
         WHERE lr.lesson_id = ?1",
    )?;
    let existing = stmt
        .query_map([lesson_id], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
        })?
        .collect::<Result<Vec<_>, rusqlite::Error>>()?;

    for (resource_id, relative_path) in existing {
        if !desired_paths.contains(&relative_path) {
            conn.execute("DELETE FROM lesson_resource WHERE id = ?1", [&resource_id])?;
        }
    }

    for resource in &lesson.resources {
        let source_file_id =
            upsert_source_file(conn, course_id, course_root, &resource.relative_path, now)?;
        let existing_id: Option<String> = conn
            .query_row(
                "SELECT id FROM lesson_resource WHERE lesson_id = ?1 AND source_file_id = ?2",
                rusqlite::params![lesson_id, source_file_id],
                |row| row.get(0),
            )
            .optional()?;
        if existing_id.is_some() {
            conn.execute(
                "UPDATE lesson_resource SET kind = ?1 WHERE lesson_id = ?2 AND source_file_id = ?3",
                rusqlite::params![resource.kind, lesson_id, source_file_id],
            )?;
            continue;
        }
        let resource_id = Uuid::new_v4().to_string();
        conn.execute(
            "INSERT INTO lesson_resource (id, lesson_id, source_file_id, kind)
             VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![resource_id, lesson_id, source_file_id, resource.kind],
        )?;
    }

    Ok(())
}

fn upsert_source_file(
    conn: &Connection,
    course_id: &str,
    course_root: &Path,
    relative_path: &str,
    now: &str,
) -> Result<String, DbError> {
    let extension = Path::new(relative_path)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_ascii_lowercase();
    let kind = source_kind_for_extension(&extension);
    let disk_path = course_root.join(relative_path);
    let (size_bytes, file_dev, file_ino) = crate::import::source_file_disk_metadata(&disk_path);

    let existing: Option<String> = conn
        .query_row(
            "SELECT id FROM source_file WHERE course_id = ?1 AND relative_path = ?2",
            rusqlite::params![course_id, relative_path],
            |row| row.get(0),
        )
        .optional()?;

    if let Some(id) = existing {
        conn.execute(
            "UPDATE source_file
             SET kind = ?1, extension = ?2, last_seen_at = ?3, missing_since = NULL,
                 size_bytes = ?4, file_dev = ?5, file_ino = ?6
             WHERE id = ?7",
            rusqlite::params![kind, extension, now, size_bytes, file_dev, file_ino, id],
        )?;
        Ok(id)
    } else {
        let id = Uuid::new_v4().to_string();
        conn.execute(
            "INSERT INTO source_file (
                id, course_id, relative_path, kind, extension, last_seen_at,
                size_bytes, file_dev, file_ino
             ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            rusqlite::params![
                id,
                course_id,
                relative_path,
                kind,
                extension,
                now,
                size_bytes,
                file_dev,
                file_ino
            ],
        )?;
        Ok(id)
    }
}

fn i64_outcome_columns(
    outcome: &crate::extract::ExtractorOutcome<i64>,
) -> (Option<i64>, Option<String>, Option<String>) {
    match outcome {
        crate::extract::ExtractorOutcome::Known { extractor, value } => {
            (Some(*value), Some(extractor.clone()), None)
        }
        crate::extract::ExtractorOutcome::Unknown { extractor, reason } => {
            (None, Some(extractor.clone()), Some(reason.clone()))
        }
    }
}

fn string_outcome_columns(
    outcome: &crate::extract::ExtractorOutcome<String>,
) -> (Option<String>, Option<String>, Option<String>) {
    match outcome {
        crate::extract::ExtractorOutcome::Known { extractor, value } => {
            (Some(value.clone()), Some(extractor.clone()), None)
        }
        crate::extract::ExtractorOutcome::Unknown { extractor, reason } => {
            (None, Some(extractor.clone()), Some(reason.clone()))
        }
    }
}

fn read_i64_outcome(
    row: &rusqlite::Row,
    value_idx: usize,
    extractor_idx: usize,
    error_idx: usize,
) -> Result<crate::extract::ExtractorOutcome<i64>, rusqlite::Error> {
    let extractor: Option<String> = row.get(extractor_idx)?;
    let error: Option<String> = row.get(error_idx)?;
    let value: Option<i64> = row.get(value_idx)?;
    Ok(match (value, extractor, error) {
        (Some(v), Some(ext), None) => crate::extract::ExtractorOutcome::Known {
            extractor: ext,
            value: v,
        },
        (None, Some(ext), Some(reason)) => crate::extract::ExtractorOutcome::Unknown {
            extractor: ext,
            reason,
        },
        _ => crate::extract::ExtractorOutcome::Unknown {
            extractor: "vault".into(),
            reason: "metadata not indexed".into(),
        },
    })
}

fn read_string_outcome(
    row: &rusqlite::Row,
    value_idx: usize,
    extractor_idx: usize,
    error_idx: usize,
) -> Result<crate::extract::ExtractorOutcome<String>, rusqlite::Error> {
    let extractor: Option<String> = row.get(extractor_idx)?;
    let error: Option<String> = row.get(error_idx)?;
    let value: Option<String> = row.get(value_idx)?;
    Ok(match (value, extractor, error) {
        (Some(v), Some(ext), None) => crate::extract::ExtractorOutcome::Known {
            extractor: ext,
            value: v,
        },
        (None, Some(ext), Some(reason)) => crate::extract::ExtractorOutcome::Unknown {
            extractor: ext,
            reason,
        },
        _ => crate::extract::ExtractorOutcome::Unknown {
            extractor: "vault".into(),
            reason: "metadata not indexed".into(),
        },
    })
}

fn now_iso8601() -> String {
    // Tests only need stable ordering, not wall-clock accuracy.
    format!(
        "{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    )
}

fn lesson_note_timestamp() -> String {
    let dur = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap();
    format!("{}.{:06}", dur.as_secs(), dur.subsec_micros())
}
