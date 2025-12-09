use auto_pqdfium_rs::{extract_text, pdf_to_json, initialize};

/// Sample PDF bytes - a simple "Hello World!" PDF
const SAMPLE_PDF: &[u8] = include_bytes!("sample.pdf");

#[test]
fn test_initialization() {
    // Should initialize successfully
    assert!(initialize().is_ok());
    // Should be idempotent (can call multiple times)
    assert!(initialize().is_ok());
}

#[test]
fn test_extract_text_from_sample() {
    let result = extract_text(SAMPLE_PDF);
    assert!(result.is_ok(), "Text extraction should succeed");

    let text = result.unwrap();
    assert!(!text.is_empty(), "Extracted text should not be empty");
    assert!(text.contains("Hello World!"), "Should contain 'Hello World!'");

    println!("Extracted text: {}", text);
}

#[test]
fn test_pdf_to_json_from_sample() {
    let result = pdf_to_json(SAMPLE_PDF);
    assert!(result.is_ok(), "JSON conversion should succeed");

    let json = result.unwrap();
    assert!(!json.is_empty(), "JSON output should not be empty");

    // JSON should be valid - try to parse it
    let parsed: serde_json::Value = serde_json::from_str(&json)
        .expect("JSON output should be valid JSON");

    // QPDF JSON should have these top-level keys
    assert!(parsed.is_object(), "Root should be an object");

    println!("JSON structure has {} top-level keys",
             parsed.as_object().unwrap().len());
}

#[test]
fn test_extract_text_empty_data() {
    let result = extract_text(&[]);
    assert!(result.is_err(), "Should fail with empty data");
}

#[test]
fn test_pdf_to_json_empty_data() {
    let result = pdf_to_json(&[]);
    assert!(result.is_err(), "Should fail with empty data");
}

#[test]
fn test_extract_text_invalid_pdf() {
    let invalid_data = b"This is not a PDF file";
    let result = extract_text(invalid_data);
    assert!(result.is_err(), "Should fail with invalid PDF data");
}

#[test]
fn test_pdf_to_json_invalid_pdf() {
    let invalid_data = b"This is not a PDF file";
    let result = pdf_to_json(invalid_data);
    assert!(result.is_err(), "Should fail with invalid PDF data");
}
