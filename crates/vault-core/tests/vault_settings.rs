use std::fs;

use tempfile::tempdir;
use vault_core::vault_settings::{
    load_vault_settings, save_vault_settings, SettingsError, VaultSettings,
};
use vault_core::VaultDb;

#[test]
fn fresh_db_has_default_settings() {
    let db = VaultDb::open_in_memory().unwrap();
    let settings = load_vault_settings(&db).unwrap();
    assert_eq!(
        settings,
        VaultSettings {
            tesseract_language: "eng".to_string(),
            transcription_model_path: String::new(),
            anki_desktop_binary_path: "anki".to_string(),
        },
    );
    assert!(!settings.transcription_enabled());
}

#[test]
fn settings_roundtrip() {
    let db = VaultDb::open_in_memory().unwrap();
    let dir = tempdir().unwrap();
    let model_path = dir.path().join("whisper.bin");
    fs::write(&model_path, b"model").unwrap();

    let written = VaultSettings {
        tesseract_language: "deu".to_string(),
        transcription_model_path: model_path.to_string_lossy().into_owned(),
        anki_desktop_binary_path: "/opt/anki/anki".to_string(),
    };
    save_vault_settings(&db, &written).unwrap();
    assert_eq!(load_vault_settings(&db).unwrap(), written);
}

#[test]
fn empty_transcription_model_path_is_allowed() {
    let db = VaultDb::open_in_memory().unwrap();
    let settings = VaultSettings {
        tesseract_language: "eng".to_string(),
        transcription_model_path: String::new(),
        anki_desktop_binary_path: "anki".to_string(),
    };
    save_vault_settings(&db, &settings).unwrap();
    assert!(!load_vault_settings(&db).unwrap().transcription_enabled());
}

#[test]
fn missing_transcription_model_file_rejected() {
    let db = VaultDb::open_in_memory().unwrap();
    let settings = VaultSettings {
        tesseract_language: "eng".to_string(),
        transcription_model_path: "/no/such/whisper-model.bin".to_string(),
        anki_desktop_binary_path: "anki".to_string(),
    };
    let err = save_vault_settings(&db, &settings).unwrap_err();
    assert!(matches!(err, SettingsError::TranscriptionModelNotFound(_)));
    assert_eq!(
        load_vault_settings(&db).unwrap().transcription_model_path,
        "",
    );
}
