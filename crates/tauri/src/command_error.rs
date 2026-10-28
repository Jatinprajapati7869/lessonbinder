use serde::Serialize;
use vault_core::course_folder::CourseFolderOpenError;
use vault_core::import::{ImportError, RescanError};
use vault_core::thumbnail::ThumbnailError;

use crate::backup::BackupError;

#[derive(Debug, Clone, Serialize)]
pub struct CommandError {
    pub code: String,
    pub message: String,
}

impl CommandError {
    pub fn to_invoke_string(&self) -> String {
        serde_json::to_string(self).unwrap_or_else(|_| self.message.clone())
    }
}

pub fn from_rescan_error(err: RescanError) -> CommandError {
    if matches!(err, RescanError::Import(ImportError::Cancelled)) {
        return from_import_error(ImportError::Cancelled);
    }
    let code = match &err {
        RescanError::NotSupported => "rescan_not_supported",
        RescanError::CourseNotFound => "rescan_course_not_found",
        RescanError::Db(_) => "rescan_db_error",
        RescanError::Import(_) => "rescan_import_error",
        RescanError::Progress(_) => "rescan_progress_error",
    };
    CommandError {
        code: code.to_string(),
        message: err.to_string(),
    }
}

pub fn from_import_error(err: ImportError) -> CommandError {
    let code = match &err {
        ImportError::RiskyImportPath => "risky_import_path",
        ImportError::InvalidPath(_) => "import_invalid_path",
        ImportError::NoLessonFiles => "no_lesson_files",
        ImportError::Io(_) => "import_io_error",
        ImportError::Db(_) => "import_db_error",
        ImportError::Cancelled => "import_cancelled",
    };
    CommandError {
        code: code.to_string(),
        message: err.to_string(),
    }
}

pub fn from_course_folder_open_error(err: CourseFolderOpenError) -> CommandError {
    match err {
        CourseFolderOpenError::Import(import_err) => from_import_error(import_err),
        CourseFolderOpenError::CourseNotFound => CommandError {
            code: "course_not_found".into(),
            message: "course_not_found".into(),
        },
        CourseFolderOpenError::NotFolderCourse => CommandError {
            code: "not_folder_course".into(),
            message: "not_folder_course".into(),
        },
        CourseFolderOpenError::CourseRootMissing => CommandError {
            code: "course_root_missing".into(),
            message: "course_root_missing".into(),
        },
        CourseFolderOpenError::CourseRootStale => CommandError {
            code: "course_root_stale".into(),
            message: "course_root_stale".into(),
        },
        CourseFolderOpenError::Db(db_err) => CommandError {
            code: "open_course_folder_db_error".into(),
            message: db_err.to_string(),
        },
    }
}

pub fn from_thumbnail_error(err: ThumbnailError) -> CommandError {
    let code = match &err {
        ThumbnailError::RequiresLocalFile => "thumbnail_requires_local_file",
        ThumbnailError::LessonNotFound => "thumbnail_lesson_not_found",
        ThumbnailError::Db(_) => "thumbnail_db_error",
        ThumbnailError::Io(_) => "thumbnail_io_error",
    };
    CommandError {
        code: code.to_string(),
        message: err.to_string(),
    }
}

pub fn from_backup_error(err: BackupError) -> CommandError {
    let code = match &err {
        BackupError::RefusesLiveProfileDestination => "backup_refuses_live_profile",
        BackupError::VaultDbMissing => "backup_vault_db_missing",
        BackupError::IncompleteBackupFolder => "backup_incomplete_folder",
        BackupError::Io(_) => "backup_io_error",
    };
    CommandError {
        code: code.to_string(),
        message: err.to_string(),
    }
}
