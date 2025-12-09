//! # auto-pqdfium-rs
//!
//! A minimal, safe Rust wrapper for PDFium + QPDF using autocxx.
//!
//! This crate provides two main functionalities:
//! - Text extraction from PDFs via PDFium
//! - PDF to JSON conversion via QPDF
//!
//! ## Example
//!
//! ```no_run
//! use auto_pqdfium_rs::{extract_text, pdf_to_json};
//!
//! // Load PDF bytes (from file, network, etc.)
//! let pdf_bytes = std::fs::read("sample.pdf").unwrap();
//!
//! // Extract text
//! let text = extract_text(&pdf_bytes).unwrap();
//! println!("Extracted text: {}", text);
//!
//! // Convert to JSON
//! let json = pdf_to_json(&pdf_bytes).unwrap();
//! println!("PDF as JSON: {}", json);
//! ```

use std::sync::Once;

// Error types
mod error;
pub use error::{PdfiumError, Result};

// Include the autocxx-generated bindings for the C bridge
autocxx::include_cpp! {
    #include "bridge.h"

    safety!(unsafe_ffi)

    generate!("pdfium_bridge_initialize")
    generate!("pdfium_bridge_cleanup")
    generate!("pdfium_bridge_extract_text")
    generate!("pdfium_bridge_pdf_to_json")
    generate!("pdfium_bridge_free_string")
}

static INIT: Once = Once::new();

/// Initialize PDFium library
///
/// This is called automatically by the other functions, but you can call it
/// explicitly if you want to ensure initialization happens at a specific time.
pub fn initialize() -> Result<()> {
    let mut init_result = Ok(());

    INIT.call_once(|| {
        unsafe {
            if ffi::pdfium_bridge_initialize() == autocxx::c_int(0) {
                init_result = Err(PdfiumError::InitializationFailed);
            }
        }
    });

    init_result
}

/// Extract text from a PDF document
///
/// # Arguments
///
/// * `pdf_bytes` - The PDF document as a byte slice
///
/// # Returns
///
/// Returns the extracted text as a String. Pages are separated by "---PAGE BREAK---".
///
/// # Errors
///
/// Returns `PdfiumError::InvalidData` if the input is empty.
/// Returns `PdfiumError::ExtractionFailed` if the PDF cannot be processed.
///
/// # Example
///
/// ```no_run
/// use auto_pqdfium_rs::extract_text;
///
/// let pdf_bytes = std::fs::read("document.pdf").unwrap();
/// let text = extract_text(&pdf_bytes).unwrap();
/// println!("Text: {}", text);
/// ```
pub fn extract_text(pdf_bytes: &[u8]) -> Result<String> {
    // Ensure PDFium is initialized
    initialize()?;

    if pdf_bytes.is_empty() {
        return Err(PdfiumError::InvalidData);
    }

    unsafe {
        let c_str_ptr = ffi::pdfium_bridge_extract_text(
            pdf_bytes.as_ptr(),
            pdf_bytes.len()
        );

        if c_str_ptr.is_null() {
            return Err(PdfiumError::ExtractionFailed(
                "Failed to extract text from PDF".to_string()
            ));
        }

        // Convert C string to Rust String
        let c_str = std::ffi::CStr::from_ptr(c_str_ptr);
        let text = c_str.to_string_lossy().into_owned();

        // Free the C string
        ffi::pdfium_bridge_free_string(c_str_ptr);

        Ok(text)
    }
}

/// Convert a PDF document to JSON format using QPDF
///
/// # Arguments
///
/// * `pdf_bytes` - The PDF document as a byte slice
///
/// # Returns
///
/// Returns the PDF structure as a JSON string (version 2 format with comprehensive details).
///
/// # Errors
///
/// Returns `PdfiumError::InvalidData` if the input is empty.
/// Returns `PdfiumError::ConversionFailed` if the PDF cannot be converted.
///
/// # Example
///
/// ```no_run
/// use auto_pqdfium_rs::pdf_to_json;
///
/// let pdf_bytes = std::fs::read("document.pdf").unwrap();
/// let json = pdf_to_json(&pdf_bytes).unwrap();
/// println!("JSON: {}", json);
/// ```
pub fn pdf_to_json(pdf_bytes: &[u8]) -> Result<String> {
    // Ensure PDFium is initialized
    initialize()?;

    if pdf_bytes.is_empty() {
        return Err(PdfiumError::InvalidData);
    }

    unsafe {
        let c_str_ptr = ffi::pdfium_bridge_pdf_to_json(
            pdf_bytes.as_ptr(),
            pdf_bytes.len()
        );

        if c_str_ptr.is_null() {
            return Err(PdfiumError::ConversionFailed(
                "Failed to convert PDF to JSON".to_string()
            ));
        }

        // Convert C string to Rust String
        let c_str = std::ffi::CStr::from_ptr(c_str_ptr);
        let json = c_str.to_string_lossy().into_owned();

        // Free the C string
        ffi::pdfium_bridge_free_string(c_str_ptr);

        Ok(json)
    }
}

/// Cleanup PDFium library
///
/// This should be called at program exit. It's optional as the OS will clean up
/// resources anyway, but it's good practice to call it explicitly.
pub fn cleanup() {
    unsafe {
        ffi::pdfium_bridge_cleanup();
    }
}
