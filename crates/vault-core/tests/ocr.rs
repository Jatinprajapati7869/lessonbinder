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

#[test]
fn ocr_shells_out_to_local_tesseract_binary_opens_no_url() {
    let source = include_str!("../src/ocr.rs");
    assert!(
        source.contains("Command::new(tesseract)"),
        "OCR must shell out to the local tesseract binary via std::process::Command, not a remote service"
    );
    assert!(
        !source.contains("http://") && !source.contains("https://"),
        "OCR source must not open HTTP(S) URLs"
    );
    for forbidden in ["reqwest", "ureq", "hyper::", "TcpStream", "Url::parse"] {
        assert!(
            !source.contains(forbidden),
            "OCR source must not use network client API {forbidden:?}"
        );
    }
}
