//! Local OCR engine port (fake for tests; Tesseract behind `tesseract` feature).

#[cfg(feature = "tesseract")]
use std::path::Path;
use std::path::PathBuf;
#[cfg(feature = "tesseract")]
use std::process::Command;

#[derive(Debug, Clone, PartialEq)]
pub struct OcrResult {
    pub text: String,
    pub confidence: f32,
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum OcrError {
    #[error("ocr_unavailable")]
    OcrUnavailable,
    #[error("ocr recognition failed: {0}")]
    RecognitionFailed(String),
    #[error("io error: {0}")]
    Io(String),
}

pub trait OcrEngine {
    fn recognize(&self, image: &[u8]) -> Result<OcrResult, OcrError>;
}

/// Deterministic engine for unit tests (ignores image bytes).
#[derive(Debug, Clone)]
pub struct FakeOcrEngine {
    text: String,
    confidence: f32,
}

impl FakeOcrEngine {
    pub fn fixed(text: impl Into<String>, confidence: f32) -> Self {
        Self {
            text: text.into(),
            confidence,
        }
    }
}

impl OcrEngine for FakeOcrEngine {
    fn recognize(&self, _image: &[u8]) -> Result<OcrResult, OcrError> {
        Ok(OcrResult {
            text: self.text.clone(),
            confidence: self.confidence,
        })
    }
}

/// Local Tesseract CLI adapter. Requires `tesseract` on PATH when the `tesseract` feature is on.
#[derive(Debug, Clone)]
pub struct TesseractOcrEngine {
    tesseract: Option<PathBuf>,
}

impl TesseractOcrEngine {
    pub fn discover() -> Self {
        Self {
            tesseract: if cfg!(feature = "tesseract") {
                which("tesseract")
            } else {
                None
            },
        }
    }

    pub fn without_tesseract() -> Self {
        Self { tesseract: None }
    }

    pub fn with_tesseract_path(path: impl Into<PathBuf>) -> Self {
        Self {
            tesseract: Some(path.into()),
        }
    }

    pub fn is_available(&self) -> bool {
        self.tesseract.as_ref().is_some_and(|path| path.is_file())
    }
}

impl OcrEngine for TesseractOcrEngine {
    fn recognize(&self, image: &[u8]) -> Result<OcrResult, OcrError> {
        let path = self.tesseract.as_ref().ok_or(OcrError::OcrUnavailable)?;
        if !path.is_file() {
            return Err(OcrError::OcrUnavailable);
        }
        #[cfg(feature = "tesseract")]
        {
            recognize_with_tesseract(path, image)
        }
        #[cfg(not(feature = "tesseract"))]
        {
            let _ = image;
            Err(OcrError::OcrUnavailable)
        }
    }
}

#[cfg(feature = "tesseract")]
fn recognize_with_tesseract(tesseract: &Path, image: &[u8]) -> Result<OcrResult, OcrError> {
    let work = std::env::temp_dir().join(format!("auravault-ocr-{}", uuid::Uuid::new_v4()));
    std::fs::create_dir_all(&work).map_err(|e| OcrError::Io(e.to_string()))?;
    let image_path = work.join("input.png");
    std::fs::write(&image_path, image).map_err(|e| OcrError::Io(e.to_string()))?;
    let tsv_base = work.join("out");

    let output = Command::new(tesseract)
        .arg(&image_path)
        .arg(&tsv_base)
        .arg("tsv")
        .output()
        .map_err(|e| {
            if e.kind() == std::io::ErrorKind::NotFound {
                OcrError::OcrUnavailable
            } else {
                OcrError::Io(e.to_string())
            }
        })?;

    if !output.status.success() {
        let _ = std::fs::remove_dir_all(&work);
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(OcrError::RecognitionFailed(stderr.trim().to_string()));
    }

    let tsv_path = work.join("out.tsv");
    let tsv = std::fs::read_to_string(&tsv_path).map_err(|e| OcrError::Io(e.to_string()))?;
    let _ = std::fs::remove_dir_all(&work);
    parse_tesseract_tsv(&tsv)
}

#[cfg(feature = "tesseract")]
fn parse_tesseract_tsv(tsv: &str) -> Result<OcrResult, OcrError> {
    let mut text_parts = Vec::new();
    let mut confidences = Vec::new();
    for line in tsv.lines().skip(1) {
        let cols: Vec<&str> = line.split('\t').collect();
        if cols.len() < 12 {
            continue;
        }
        let conf = cols[10].parse::<f32>().unwrap_or(-1.0);
        if conf < 0.0 {
            continue;
        }
        let word = cols[11].trim();
        if word.is_empty() {
            continue;
        }
        confidences.push(conf);
        text_parts.push(word);
    }
    if text_parts.is_empty() {
        return Ok(OcrResult {
            text: String::new(),
            confidence: 0.0,
        });
    }
    let confidence = confidences.iter().sum::<f32>() / confidences.len() as f32 / 100.0;
    Ok(OcrResult {
        text: text_parts.join(" "),
        confidence,
    })
}

fn which(cmd: &str) -> Option<PathBuf> {
    let path_var = std::env::var_os("PATH")?;
    for dir in std::env::split_paths(&path_var) {
        let candidate = dir.join(cmd);
        if candidate.is_file() {
            return Some(candidate);
        }
    }
    None
}
