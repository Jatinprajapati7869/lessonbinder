//! Folder import: scan disk and upsert course / lessons in `vault.db`.

use std::collections::{HashMap, HashSet};
use std::ffi::OsStr;
use std::fs;
use std::path::{Component, Path, PathBuf};

use crate::db::VaultDb;
use crate::lessonbinderignore::LessonbinderIgnore;
use crate::progress_persist::ProgressPersistenceError;

const PRIMARY_MEDIA_EXTENSIONS: &[&str] = &["mp4", "mkv", "webm", "mp3", "m4a"];
const SUBTITLE_EXTENSIONS: &[&str] = &["srt", "vtt"];
const STANDALONE_LESSON_EXTENSIONS: &[&str] = &["pdf", "md", "txt", "png", "jpg", "jpeg"];
const RESOURCE_ATTACHMENT_EXTENSIONS: &[&str] = &["pdf", "png", "jpg", "jpeg", "txt", "md"];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScannedResource {
    pub relative_path: String,
    pub kind: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScannedLesson {
    pub relative_path: String,
    pub display_title: String,
    pub subtitle_relative_path: Option<String>,
    pub resources: Vec<ScannedResource>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImportReport {
    pub course_id: String,
    pub lesson_ids: Vec<String>,
}

/// Lesson-file progress during folder scan and upsert (`seen` / `planned`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FolderScanProgress {
    pub seen: u32,
    pub planned: u32,
}

#[derive(Debug, thiserror::Error)]
pub enum ImportError {
    #[error(transparent)]
    Db(#[from] crate::db::DbError),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("{0}")]
    InvalidPath(String),
    #[error("risky_import_path")]
    RiskyImportPath,
    #[error("cancelled")]
    Cancelled,
}

#[derive(Debug, thiserror::Error)]
pub enum RescanError {
    #[error(transparent)]
    Db(#[from] crate::db::DbError),
    #[error(transparent)]
    Import(#[from] ImportError),
    #[error(transparent)]
    Progress(#[from] ProgressPersistenceError),
    #[error("rescan_not_supported")]
    NotSupported,
    #[error("course not found")]
    CourseNotFound,
}

impl PartialEq for RescanError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::NotSupported, Self::NotSupported) => true,
            (Self::CourseNotFound, Self::CourseNotFound) => true,
            (Self::Db(left), Self::Db(right)) => left.to_string() == right.to_string(),
            (Self::Import(left), Self::Import(right)) => left.to_string() == right.to_string(),
            (Self::Progress(left), Self::Progress(right)) => left.to_string() == right.to_string(),
            _ => false,
        }
    }
}

/// Rescan a local folder course from disk: upsert current files, mark missing lessons broken.
pub fn rescan(db: &VaultDb, course_id: &str) -> Result<ImportReport, RescanError> {
    rescan_with_cancel(db, course_id, &|| false, None)
}

/// Like [`rescan`], but checks `should_cancel` while scanning and before each vault write.
pub fn rescan_with_cancel(
    db: &VaultDb,
    course_id: &str,
    should_cancel: &dyn Fn() -> bool,
    on_progress: Option<&dyn Fn(FolderScanProgress)>,
) -> Result<ImportReport, RescanError> {
    let source_kind = db.course_source_kind(course_id)?;
    if source_kind != "folder" {
        return Err(RescanError::NotSupported);
    }
    let Some(root_path) = db.course_root_path(course_id)? else {
        return Err(RescanError::CourseNotFound);
    };
    let root = PathBuf::from(&root_path);
    let lessons = scan_folder_with_cancel(&root, should_cancel, on_progress)?;
    if should_cancel() {
        return Err(RescanError::Import(ImportError::Cancelled));
    }
    relink_renamed_local_files(db, course_id, &root, &lessons)?;
    if should_cancel() {
        return Err(RescanError::Import(ImportError::Cancelled));
    }
    let scanned_keys: HashSet<String> = lessons
        .iter()
        .map(|lesson| lesson.relative_path.clone())
        .collect();

    let existing = db.local_file_lessons_for_course(course_id)?;
    upsert_scanned_lessons_with_cancel(db, course_id, &root, &lessons, should_cancel, on_progress)?;

    for lesson in existing {
        if !scanned_keys.contains(&lesson.stable_key) {
            if should_cancel() {
                return Err(RescanError::Import(ImportError::Cancelled));
            }
            db.mark_source_missing(&lesson.lesson_id)?;
        }
    }

    let lesson_ids = db.lesson_ids_in_learning_order(course_id)?;

    Ok(ImportReport {
        course_id: course_id.to_string(),
        lesson_ids,
    })
}

/// Same checks as [`import_folder`] before touching the vault.
pub fn validate_import_folder_path(picked: &Path) -> Result<(), ImportError> {
    resolve_import_root(picked)?;
    Ok(())
}

/// Import or re-import a folder as one course (idempotent on `root_path`).
pub fn import_folder(db: &VaultDb, picked: &Path) -> Result<ImportReport, ImportError> {
    import_folder_with_cancel(db, picked, &|| false, None)
}

/// Import a folder, checking `should_cancel` between directory entries while scanning
/// and again before each lesson upsert (never mid-write for a single lesson).
pub fn import_folder_with_cancel(
    db: &VaultDb,
    picked: &Path,
    should_cancel: &dyn Fn() -> bool,
    on_progress: Option<&dyn Fn(FolderScanProgress)>,
) -> Result<ImportReport, ImportError> {
    let root = resolve_import_root(picked)?;
    let lessons = scan_folder_with_cancel(&root, should_cancel, on_progress)?;
    let title = root
        .file_name()
        .and_then(OsStr::to_str)
        .unwrap_or("Course")
        .to_string();
    let root_path = root.to_string_lossy().to_string();
    let course_id = db
        .ensure_folder_course(&root_path, &title)
        .map_err(ImportError::from)?;
    let lesson_ids = upsert_scanned_lessons_with_cancel(
        db,
        &course_id,
        &root,
        &lessons,
        should_cancel,
        on_progress,
    )?;
    Ok(ImportReport {
        course_id,
        lesson_ids,
    })
}

fn upsert_scanned_lessons_with_cancel(
    db: &VaultDb,
    course_id: &str,
    course_root: &Path,
    lessons: &[ScannedLesson],
    should_cancel: &dyn Fn() -> bool,
    on_progress: Option<&dyn Fn(FolderScanProgress)>,
) -> Result<Vec<String>, ImportError> {
    let planned = u32::try_from(lessons.len()).unwrap_or(u32::MAX);
    if let Some(report) = on_progress {
        report(FolderScanProgress { seen: 0, planned });
    }
    let mut lesson_ids = Vec::with_capacity(lessons.len());
    for (index, lesson) in lessons.iter().enumerate() {
        if should_cancel() {
            return Err(ImportError::Cancelled);
        }
        let lesson_id = db
            .upsert_single_scanned_lesson(course_id, course_root, lesson)
            .map_err(ImportError::from)?;
        lesson_ids.push(lesson_id);
        if let Some(report) = on_progress {
            let seen = u32::try_from(index + 1).unwrap_or(u32::MAX);
            report(FolderScanProgress { seen, planned });
        }
    }
    Ok(lesson_ids)
}

/// Discover lesson and subtitle files under `root` (pure; no database).
pub fn scan_folder(root: &Path) -> Result<Vec<ScannedLesson>, ImportError> {
    scan_folder_with_cancel(root, &|| false, None)
}

/// Like [`scan_folder`], but aborts when `should_cancel` returns true between entries.
pub fn scan_folder_with_cancel(
    root: &Path,
    should_cancel: &dyn Fn() -> bool,
    on_progress: Option<&dyn Fn(FolderScanProgress)>,
) -> Result<Vec<ScannedLesson>, ImportError> {
    let root = fs::canonicalize(root)?;
    let ignore = LessonbinderIgnore::load_from_course_root(&root).map_err(ImportError::from)?;
    let mut primary_media_files: Vec<PathBuf> = Vec::new();
    let mut subtitle_files: Vec<PathBuf> = Vec::new();
    let mut sidecar_files: Vec<PathBuf> = Vec::new();
    let mut lesson_files_seen = 0u32;
    collect_files(
        &root,
        &root,
        &ignore,
        &mut primary_media_files,
        &mut subtitle_files,
        &mut sidecar_files,
        should_cancel,
        &mut lesson_files_seen,
        on_progress,
    )?;

    let mut subtitles_by_stem: HashMap<(String, String), String> = HashMap::new();
    for path in subtitle_files {
        let rel = relative_path(&root, &path)?;
        let parent = Path::new(&rel)
            .parent()
            .map(|p| p.to_string_lossy().into_owned())
            .unwrap_or_default();
        let stem = file_stem(&path).unwrap_or_default();
        subtitles_by_stem.insert((parent, stem), rel);
    }

    let mut sidecars_by_stem: HashMap<(String, String), Vec<PathBuf>> = HashMap::new();
    for path in &sidecar_files {
        let rel = relative_path(&root, path)?;
        let parent = Path::new(&rel)
            .parent()
            .map(|p| p.to_string_lossy().into_owned())
            .unwrap_or_default();
        let stem = file_stem(path).unwrap_or_default();
        sidecars_by_stem
            .entry((parent, stem))
            .or_default()
            .push(path.clone());
    }

    let mut consumed_sidecars: HashSet<String> = HashSet::new();
    let mut lessons = Vec::new();

    for path in primary_media_files {
        let ext = extension_lower(&path);
        if !PRIMARY_MEDIA_EXTENSIONS.contains(&ext.as_str()) {
            continue;
        }
        let rel = relative_path(&root, &path)?;
        let parent = Path::new(&rel)
            .parent()
            .map(|p| p.to_string_lossy().into_owned())
            .unwrap_or_default();
        let stem = file_stem(&path).unwrap_or_default();
        let stem_key = (parent.clone(), stem.clone());
        let subtitle = subtitles_by_stem.get(&stem_key).cloned();
        let resources = resources_for_stem(
            &root,
            sidecars_by_stem.get(&stem_key),
            &mut consumed_sidecars,
        );
        lessons.push(ScannedLesson {
            relative_path: rel,
            display_title: stem,
            subtitle_relative_path: subtitle,
            resources,
        });
    }

    for path in &sidecar_files {
        let rel = relative_path(&root, path)?;
        if consumed_sidecars.contains(&rel) {
            continue;
        }
        let ext = extension_lower(path);
        if !STANDALONE_LESSON_EXTENSIONS.contains(&ext.as_str()) {
            continue;
        }
        let stem = file_stem(path).unwrap_or_default();
        lessons.push(ScannedLesson {
            relative_path: rel,
            display_title: stem,
            subtitle_relative_path: None,
            resources: Vec::new(),
        });
    }

    lessons.sort_by(|a, b| a.relative_path.cmp(&b.relative_path));
    let planned = u32::try_from(lessons.len()).unwrap_or(u32::MAX);
    if planned > 0 {
        if let Some(report) = on_progress {
            report(FolderScanProgress {
                seen: planned,
                planned,
            });
        }
    }
    Ok(lessons)
}

fn resources_for_stem(
    root: &Path,
    sidecars: Option<&Vec<PathBuf>>,
    consumed_sidecars: &mut HashSet<String>,
) -> Vec<ScannedResource> {
    let Some(paths) = sidecars else {
        return Vec::new();
    };
    let mut resources = Vec::new();
    for path in paths {
        let ext = extension_lower(path);
        if !RESOURCE_ATTACHMENT_EXTENSIONS.contains(&ext.as_str()) {
            continue;
        }
        let rel = match relative_path(root, path) {
            Ok(rel) => rel,
            Err(_) => continue,
        };
        consumed_sidecars.insert(rel.clone());
        resources.push(ScannedResource {
            relative_path: rel,
            kind: lesson_resource_kind(&ext).to_string(),
        });
    }
    resources.sort_by(|a, b| a.relative_path.cmp(&b.relative_path));
    resources
}

pub(crate) fn lesson_resource_kind(ext: &str) -> &'static str {
    match ext {
        "pdf" => "pdf",
        "png" | "jpg" | "jpeg" => "image",
        _ => "text",
    }
}

#[allow(clippy::too_many_arguments)]
fn collect_files(
    root: &Path,
    dir: &Path,
    ignore: &LessonbinderIgnore,
    primary_media_out: &mut Vec<PathBuf>,
    subtitle_out: &mut Vec<PathBuf>,
    sidecar_out: &mut Vec<PathBuf>,
    should_cancel: &dyn Fn() -> bool,
    lesson_files_seen: &mut u32,
    on_progress: Option<&dyn Fn(FolderScanProgress)>,
) -> Result<(), ImportError> {
    for entry in fs::read_dir(dir)? {
        if should_cancel() {
            return Err(ImportError::Cancelled);
        }
        let entry = entry?;
        let path = entry.path();
        let file_name = entry.file_name();
        if file_name.as_encoded_bytes().first() == Some(&b'.') {
            continue;
        }
        if path.is_dir() {
            if file_name == "node_modules" {
                continue;
            }
            if path_has_ignored_segment(&path, root) {
                continue;
            }
            collect_files(
                root,
                &path,
                ignore,
                primary_media_out,
                subtitle_out,
                sidecar_out,
                should_cancel,
                lesson_files_seen,
                on_progress,
            )?;
            continue;
        }
        if path_has_ignored_segment(&path, root) {
            continue;
        }
        let rel = relative_path(root, &path)?;
        if ignore.is_ignored(&rel) {
            continue;
        }
        let ext = extension_lower(&path);
        if SUBTITLE_EXTENSIONS.contains(&ext.as_str()) {
            subtitle_out.push(path);
        } else if PRIMARY_MEDIA_EXTENSIONS.contains(&ext.as_str()) {
            primary_media_out.push(path);
            report_lesson_file_seen(lesson_files_seen, on_progress);
        } else if STANDALONE_LESSON_EXTENSIONS.contains(&ext.as_str()) {
            sidecar_out.push(path);
            report_lesson_file_seen(lesson_files_seen, on_progress);
        }
    }
    Ok(())
}

fn report_lesson_file_seen(
    lesson_files_seen: &mut u32,
    on_progress: Option<&dyn Fn(FolderScanProgress)>,
) {
    *lesson_files_seen = lesson_files_seen.saturating_add(1);
    if let Some(report) = on_progress {
        report(FolderScanProgress {
            seen: *lesson_files_seen,
            planned: 0,
        });
    }
}

fn path_has_ignored_segment(path: &Path, root: &Path) -> bool {
    path.strip_prefix(root)
        .ok()
        .map(|rel| {
            rel.components().any(|c| match c {
                Component::Normal(name) => {
                    name.as_encoded_bytes().first() == Some(&b'.') || name == "node_modules"
                }
                _ => false,
            })
        })
        .unwrap_or(false)
}

fn relative_path(root: &Path, path: &Path) -> Result<String, ImportError> {
    let rel = path
        .strip_prefix(root)
        .map_err(|_| ImportError::InvalidPath("path outside import root".into()))?;
    Ok(rel.to_string_lossy().replace('\\', "/"))
}

fn extension_lower(path: &Path) -> String {
    path.extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_ascii_lowercase()
}

fn file_stem(path: &Path) -> Option<String> {
    path.file_stem()
        .and_then(|s| s.to_str())
        .map(str::to_string)
}

/// Canonical import root after risky-path checks. Does not walk lesson files.
pub(crate) fn resolve_import_root(picked: &Path) -> Result<PathBuf, ImportError> {
    let canonical = fs::canonicalize(picked)?;
    if !canonical.is_dir() {
        return Err(ImportError::InvalidPath(
            "import root must be a directory".into(),
        ));
    }
    if is_filesystem_root(&canonical) {
        return Err(ImportError::RiskyImportPath);
    }
    if is_home_directory(&canonical) {
        return Err(ImportError::RiskyImportPath);
    }
    if resolves_outside_picked_directory(picked, &canonical)? {
        return Err(ImportError::RiskyImportPath);
    }
    Ok(canonical)
}

fn is_filesystem_root(path: &Path) -> bool {
    path.parent().is_none() || path == Path::new("/")
}

fn is_home_directory(path: &Path) -> bool {
    let Some(home) = std::env::var_os("HOME") else {
        return false;
    };
    let Ok(home) = fs::canonicalize(Path::new(&home)) else {
        return false;
    };
    path == home
}

fn resolves_outside_picked_directory(picked: &Path, canonical: &Path) -> Result<bool, ImportError> {
    let Some(parent) = picked.parent() else {
        return Ok(false);
    };
    if parent.as_os_str().is_empty() {
        return Ok(false);
    }
    let parent_canonical = fs::canonicalize(parent)?;
    Ok(!canonical.starts_with(&parent_canonical))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct FileInodeIdentity {
    dev: u64,
    ino: u64,
    size_bytes: u64,
}

pub(crate) fn source_file_disk_metadata(path: &Path) -> (Option<i64>, Option<i64>, Option<i64>) {
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt;
        let Ok(meta) = fs::metadata(path) else {
            return (None, None, None);
        };
        (
            i64::try_from(meta.len()).ok(),
            i64::try_from(meta.dev()).ok(),
            i64::try_from(meta.ino()).ok(),
        )
    }
    #[cfg(not(unix))]
    {
        let _ = path;
        (None, None, None)
    }
}

fn file_inode_identity(path: &Path) -> Option<FileInodeIdentity> {
    let (size_bytes, file_dev, file_ino) = source_file_disk_metadata(path);
    Some(FileInodeIdentity {
        dev: u64::try_from(file_dev?).ok()?,
        ino: u64::try_from(file_ino?).ok()?,
        size_bytes: u64::try_from(size_bytes?).ok()?,
    })
}

fn relink_renamed_local_files(
    db: &VaultDb,
    course_id: &str,
    course_root: &Path,
    scanned: &[ScannedLesson],
) -> Result<(), RescanError> {
    #[cfg(not(unix))]
    {
        let _ = (db, course_id, course_root, scanned);
        return Ok(());
    }
    #[cfg(unix)]
    {
        let mut inode_to_path: HashMap<FileInodeIdentity, String> = HashMap::new();
        for lesson in scanned {
            let path = course_root.join(&lesson.relative_path);
            if let Some(identity) = file_inode_identity(&path) {
                inode_to_path.insert(identity, lesson.relative_path.clone());
            }
        }

        for row in db.local_file_lessons_for_course(course_id)? {
            if course_root.join(&row.stable_key).exists() {
                continue;
            }
            let Some(stored) = db.lesson_source_inode_identity(&row.lesson_id)? else {
                continue;
            };
            let identity = FileInodeIdentity {
                dev: stored.0,
                ino: stored.1,
                size_bytes: stored.2,
            };
            let Some(new_relative_path) = inode_to_path.get(&identity) else {
                continue;
            };
            if new_relative_path.as_str() == row.stable_key {
                continue;
            }
            db.relink_local_file_lesson(&row.lesson_id, new_relative_path)?;
        }
        Ok(())
    }
}
