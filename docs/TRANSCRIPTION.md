# Transcription (v1)

AuraVault **does not** run speech-to-text in v1. Subtitles come from **ingested SRT/VTT** only (see charter Decision 9).

The `vault-core` **`TranscriptionEngine`** port exists so a future adapter can plug in without touching lesson order or progress until it succeeds. The default **`transcription_disabled()`** engine returns **`transcription_disabled`** and performs no database writes.

## Future (not scheduled)

- Local **whisper.cpp** only
- **User-provided** model path on disk
- **No network** download of weights from AuraVault
