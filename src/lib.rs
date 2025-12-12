use std::sync::Once;
mod error;
pub use error::{PdfiumError, Result};

mod ffi {
    extern "C" {
        pub fn pdfium_bridge_initialize() -> i32;
        pub fn pdfium_bridge_cleanup();
        pub fn pdfium_bridge_extract_text(
            pdf_data: *const u8,
            pdf_size: usize,
        ) -> *mut std::os::raw::c_char;
        pub fn pdfium_bridge_pdf_to_json(
            pdf_data: *const u8,
            pdf_size: usize,
        ) -> *mut std::os::raw::c_char;
        pub fn pdfium_bridge_free_string(s: *mut std::os::raw::c_char);
    }
}

static INIT: Once = Once::new();

pub fn initialize() -> Result<()> {
    let mut init_result = Ok(());

    INIT.call_once(|| {
        unsafe {
            if ffi::pdfium_bridge_initialize() == 0 {
                init_result = Err(PdfiumError::InitializationFailed);
            }
        }
    });

    init_result
}

/// Initialize PDFium library (C ABI for WASM)
/// Returns 1 on success, 0 on failure
#[no_mangle]
pub extern "C" fn pdfium_wasm_initialize() -> i32 {
    match initialize() {
        Ok(_) => 1,
        Err(_) => 0,
    }
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

/// Extract text from a PDF document (C ABI for WASM)
/// Returns pointer to null-terminated UTF-8 string, or null on error
/// Caller must free the returned string with pdfium_wasm_free_string
#[no_mangle]
pub extern "C" fn pdfium_wasm_extract_text(
    pdf_data: *const u8,
    pdf_len: usize,
) -> *mut u8 {
    if pdf_data.is_null() || pdf_len == 0 {
        return std::ptr::null_mut();
    }

    let pdf_bytes = unsafe { std::slice::from_raw_parts(pdf_data, pdf_len) };

    match extract_text(pdf_bytes) {
        Ok(text) => {
            let c_string = std::ffi::CString::new(text).unwrap_or_default();
            c_string.into_raw() as *mut u8
        }
        Err(_) => std::ptr::null_mut(),
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

/// Convert a PDF document to JSON format using QPDF (C ABI for WASM)
/// Returns pointer to null-terminated UTF-8 string, or null on error
/// Caller must free the returned string with pdfium_wasm_free_string
#[no_mangle]
pub extern "C" fn pdfium_wasm_pdf_to_json(
    pdf_data: *const u8,
    pdf_len: usize,
) -> *mut u8 {
    if pdf_data.is_null() || pdf_len == 0 {
        return std::ptr::null_mut();
    }

    let pdf_bytes = unsafe { std::slice::from_raw_parts(pdf_data, pdf_len) };

    match pdf_to_json(pdf_bytes) {
        Ok(json) => {
            let c_string = std::ffi::CString::new(json).unwrap_or_default();
            c_string.into_raw() as *mut u8
        }
        Err(_) => std::ptr::null_mut(),
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

/// Free a string returned by pdfium_wasm_extract_text or pdfium_wasm_pdf_to_json
#[no_mangle]
pub extern "C" fn pdfium_wasm_free_string(ptr: *mut u8) {
    if !ptr.is_null() {
        unsafe {
            let _ = std::ffi::CString::from_raw(ptr as *mut i8);
        }
    }
}

/// Cleanup PDFium library (C ABI for WASM)
#[no_mangle]
pub extern "C" fn pdfium_wasm_cleanup() {
    cleanup();
}
