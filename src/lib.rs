use std::sync::Once;
mod error;
pub use error::{PdfiumError, Result};

mod ffi {
    use std::os::raw::{c_char, c_int, c_uint, c_ulong, c_uchar, c_void};

    // Opaque PDFium types
    #[allow(non_camel_case_types)]
    pub type FPDF_DOCUMENT = *mut c_void;
    #[allow(non_camel_case_types)]
    pub type FPDF_PAGE = *mut c_void;
    #[allow(non_camel_case_types)]
    pub type FPDF_TEXTPAGE = *mut c_void;

    // PDFium config structure
    #[repr(C)]
    #[allow(non_snake_case)]
    pub struct FPDF_LIBRARY_CONFIG {
        pub version: c_int,
        pub m_pUserFontPaths: *mut *const c_char,
        pub m_pIsolate: *mut c_void,
        pub m_v8EmbedderSlot: c_uint,
    }

    extern "C" {
        // Direct PDFium C API calls (no bridge!)
        pub fn FPDF_InitLibraryWithConfig(config: *const FPDF_LIBRARY_CONFIG);
        pub fn FPDF_DestroyLibrary();
        pub fn FPDF_LoadMemDocument(
            data_buf: *const c_void,
            size: c_int,
            password: *const c_char,
        ) -> FPDF_DOCUMENT;
        pub fn FPDF_CloseDocument(document: FPDF_DOCUMENT);
        pub fn FPDF_GetPageCount(document: FPDF_DOCUMENT) -> c_int;
        pub fn FPDF_LoadPage(document: FPDF_DOCUMENT, page_index: c_int) -> FPDF_PAGE;
        pub fn FPDF_ClosePage(page: FPDF_PAGE);
        pub fn FPDFText_LoadPage(page: FPDF_PAGE) -> FPDF_TEXTPAGE;
        pub fn FPDFText_ClosePage(text_page: FPDF_TEXTPAGE);
        pub fn FPDFText_CountChars(text_page: FPDF_TEXTPAGE) -> c_int;
        pub fn FPDFText_GetText(
            text_page: FPDF_TEXTPAGE,
            start_index: c_int,
            count: c_int,
            result: *mut u16,
        ) -> c_int;
        pub fn IPDF_QPDF_PDFToJSON(
            pdf_data: *const c_void,
            pdf_size: usize,
            version: c_int,
        ) -> *mut c_char;
        pub fn IPDF_QPDF_FreeString(str: *mut c_char);

        // Streaming I/O functions (directly from Universal.Pdfium)
        pub fn IPDF_StreamingIO_LoadDocument(
            file_size: c_ulong,
            get_block_callback: Option<
                unsafe extern "C" fn(*mut c_void, c_ulong, *mut c_uchar, c_ulong) -> c_int,
            >,
            user_data: *mut c_void,
            password: *const c_char,
        ) -> FPDF_DOCUMENT;
        pub fn IPDF_StreamingIO_SaveWithCallback(
            document: FPDF_DOCUMENT,
            write_block_callback: Option<
                unsafe extern "C" fn(*mut c_void, *const c_void, c_ulong) -> c_int,
            >,
            user_data: *mut c_void,
            flags: c_int,
        ) -> c_int;

        // Streaming I/O helper functions (reduce JavaScript boilerplate)
        pub fn IPDF_StreamingIO_GetPageCount(document: FPDF_DOCUMENT) -> c_int;
        pub fn IPDF_StreamingIO_GetPageSize(
            document: FPDF_DOCUMENT,
            page_index: c_int,
            width: *mut f64,
            height: *mut f64,
        ) -> c_int;
        pub fn IPDF_StreamingIO_GetPageText(
            document: FPDF_DOCUMENT,
            page_index: c_int,
        ) -> *mut c_char;
        pub fn IPDF_StreamingIO_RenderPage(
            document: FPDF_DOCUMENT,
            page_index: c_int,
            width: c_int,
            height: c_int,
            out_size: *mut c_ulong,
        ) -> *mut c_uchar;
        pub fn IPDF_StreamingIO_FreeString(ptr: *mut c_void);
    }

    // Type aliases for better readability
    pub type GetBlockCallback =
        Option<unsafe extern "C" fn(*mut c_void, c_ulong, *mut c_uchar, c_ulong) -> c_int>;
    pub type WriteBlockCallback =
        Option<unsafe extern "C" fn(*mut c_void, *const c_void, c_ulong) -> c_int>;
}

static INIT: Once = Once::new();

pub fn initialize() -> Result<()> {
    INIT.call_once(|| {
        unsafe {
            let config = ffi::FPDF_LIBRARY_CONFIG {
                version: 2,
                m_pUserFontPaths: std::ptr::null_mut(),
                m_pIsolate: std::ptr::null_mut(),
                m_v8EmbedderSlot: 0,
            };
            ffi::FPDF_InitLibraryWithConfig(&config);
        }
    });

    Ok(())
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
        // Load PDF directly with PDFium
        let doc = ffi::FPDF_LoadMemDocument(
            pdf_bytes.as_ptr() as *const std::ffi::c_void,
            pdf_bytes.len() as i32,
            std::ptr::null(),
        );

        if doc.is_null() {
            return Err(PdfiumError::ExtractionFailed(
                "Failed to load PDF document".to_string()
            ));
        }

        let page_count = ffi::FPDF_GetPageCount(doc);
        let mut text = String::new();

        // Extract text from each page
        for i in 0..page_count {
            let page = ffi::FPDF_LoadPage(doc, i);
            if page.is_null() {
                continue;
            }

            let text_page = ffi::FPDFText_LoadPage(page);
            if !text_page.is_null() {
                let text_length = ffi::FPDFText_CountChars(text_page);

                if text_length > 0 {
                    // Allocate buffer for UTF-16 text
                    let mut buffer: Vec<u16> = vec![0; (text_length + 1) as usize];
                    let chars_written = ffi::FPDFText_GetText(
                        text_page,
                        0,
                        text_length,
                        buffer.as_mut_ptr(),
                    );

                    if chars_written > 0 {
                        // Convert UTF-16 to Rust String
                        buffer.truncate((chars_written - 1) as usize);
                        text.push_str(&String::from_utf16_lossy(&buffer));
                    }
                }

                ffi::FPDFText_ClosePage(text_page);
            }

            ffi::FPDF_ClosePage(page);

            // Add page separator
            if i < page_count - 1 {
                text.push_str("\n---PAGE BREAK---\n");
            }
        }

        ffi::FPDF_CloseDocument(doc);
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
        // Call QPDF directly
        let json_ptr = ffi::IPDF_QPDF_PDFToJSON(
            pdf_bytes.as_ptr() as *const std::ffi::c_void,
            pdf_bytes.len(),
            2, // Version 2
        );

        if json_ptr.is_null() {
            return Err(PdfiumError::ConversionFailed(
                "Failed to convert PDF to JSON".to_string()
            ));
        }

        // Convert C string to Rust String
        let c_str = std::ffi::CStr::from_ptr(json_ptr);
        let json = c_str.to_string_lossy().into_owned();

        // Free the C string using QPDF's function
        ffi::IPDF_QPDF_FreeString(json_ptr);

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
        ffi::FPDF_DestroyLibrary();
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

// ============================================================================
// Custom I/O Functions for Page-by-Page PDF Processing
// ============================================================================

/// Load PDF with custom reader callback (C ABI for WASM)
///
/// This allows loading PDFs page-by-page from any source (URL, file, etc.)
///
/// # Arguments
/// * `file_size` - Total size of the PDF file in bytes
/// * `get_block_callback` - Callback function for reading data chunks
/// * `user_data` - User-defined context pointer passed to callback
/// * `password` - Optional password (null or empty string for no password)
///
/// # Returns
/// * FPDF_DOCUMENT handle on success, null on failure
///
/// # Safety
/// The callback will be called multiple times by PDFium to read data.
/// The callback signature: fn(user_data, position, buffer, size) -> success (1/0)
#[no_mangle]
pub unsafe extern "C" fn pdfium_wasm_load_custom_document(
    file_size: std::os::raw::c_ulong,
    get_block_callback: ffi::GetBlockCallback,
    user_data: *mut std::os::raw::c_void,
    password: *const std::os::raw::c_char,
) -> ffi::FPDF_DOCUMENT {
    // Ensure PDFium is initialized
    let _ = initialize();

    // Call PDFium's streaming document loader
    ffi::IPDF_StreamingIO_LoadDocument(file_size, get_block_callback, user_data, password)
}

/// Save PDF with custom writer callback (C ABI for WASM)
///
/// This allows saving PDFs incrementally to any destination (server, memory, etc.)
///
/// # Arguments
/// * `document` - FPDF_DOCUMENT handle
/// * `write_block_callback` - Callback function for writing data chunks
/// * `user_data` - User-defined context pointer passed to callback
/// * `flags` - Save flags (0 for normal, 1 for incremental)
///
/// # Returns
/// * 1 on success, 0 on failure
///
/// # Safety
/// The callback will be called multiple times by PDFium to write data chunks.
/// The callback signature: fn(user_data, data, size) -> success (1/0)
#[no_mangle]
pub unsafe extern "C" fn pdfium_wasm_save_as_copy_custom(
    document: ffi::FPDF_DOCUMENT,
    write_block_callback: ffi::WriteBlockCallback,
    user_data: *mut std::os::raw::c_void,
    flags: std::os::raw::c_int,
) -> std::os::raw::c_int {
    if document.is_null() {
        return 0;
    }

    // Call PDFium's streaming save function
    ffi::IPDF_StreamingIO_SaveWithCallback(document, write_block_callback, user_data, flags)
}
