#pragma once

#include <cstddef>
#include <cstdint>

#ifdef __cplusplus
extern "C" {
#endif

// Initialize PDFium library (call once at startup)
// Returns 1 on success, 0 on failure
int pdfium_bridge_initialize();

// Cleanup PDFium library (call once at shutdown)
void pdfium_bridge_cleanup();

// Extract text from PDF bytes
// Returns allocated string that must be freed with pdfium_bridge_free_string
// Returns NULL on error
char* pdfium_bridge_extract_text(const uint8_t* pdf_data, size_t pdf_size);

// Convert PDF to JSON using qpdf
// Returns allocated string that must be freed with pdfium_bridge_free_string
// Returns NULL on error
char* pdfium_bridge_pdf_to_json(const uint8_t* pdf_data, size_t pdf_size);

// Free a string returned by the bridge functions
void pdfium_bridge_free_string(char* str);

#ifdef __cplusplus
}
#endif
