use vault_core::ocr::{FakeOcrEngine, OcrEngine, OcrError, TesseractOcrEngine};

#[test]
fn fake_returns_text() {
    let engine = FakeOcrEngine::fixed("recognized text", 0.87);
    let result = engine.recognize(&[0x89, 0x50, 0x4e, 0x47]).unwrap();
    assert_eq!(result.text, "recognized text");
    assert!((result.confidence - 0.87).abs() < f32::EPSILON);
}

#[test]
fn tesseract_unavailable_error() {
    let engine = TesseractOcrEngine::without_tesseract();
    let err = engine.recognize(&[1, 2, 3]).unwrap_err();
    assert_eq!(err, OcrError::OcrUnavailable);
}
