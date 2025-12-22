// Copyright 2024 PDFium Streaming I/O Extension. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef PUBLIC_IPDF_STREAMINGIO_H_
#define PUBLIC_IPDF_STREAMINGIO_H_

#include "fpdfview.h"

#ifdef __cplusplus
extern "C" {
#endif

// Load PDF with custom read callback for streaming/progressive loading.
// file_size: Total PDF file size in bytes
// get_block_callback: Called to read data blocks (position, buffer, size)
// user_data: User context passed to callbacks
// password: Optional password for encrypted PDFs (NULL if not encrypted)
// Returns: Document handle or NULL on failure
FPDF_EXPORT FPDF_DOCUMENT FPDF_CALLCONV IPDF_StreamingIO_LoadDocument(
    unsigned long file_size,
    int (*get_block_callback)(void* user_data,
                              unsigned long position,
                              unsigned char* buffer,
                              unsigned long size),
    void* user_data,
    const char* password);

// Save PDF with custom write callback for streaming/progressive saving.
// document: PDF document to save
// write_block_callback: Called to write data blocks (data, size)
// user_data: User context passed to callbacks
// flags: Save flags (0, FPDF_INCREMENTAL, FPDF_NO_INCREMENTAL, FPDF_REMOVE_SECURITY)
// Returns: 1 on success, 0 on failure
FPDF_EXPORT int FPDF_CALLCONV IPDF_StreamingIO_SaveWithCallback(
    FPDF_DOCUMENT document,
    int (*write_block_callback)(void* user_data,
                                const void* data,
                                unsigned long size),
    void* user_data,
    int flags);

// ============================================================================
// Document Operation Helpers (reduce JavaScript boilerplate)
// ============================================================================

// Get number of pages in document
// Returns: Page count or 0 on error
FPDF_EXPORT int FPDF_CALLCONV IPDF_StreamingIO_GetPageCount(
    FPDF_DOCUMENT document);

// Get page dimensions in points
// width, height: Pointers to receive dimensions (can be NULL)
// Returns: 1 on success, 0 on failure
FPDF_EXPORT int FPDF_CALLCONV IPDF_StreamingIO_GetPageSize(
    FPDF_DOCUMENT document,
    int page_index,
    double* width,
    double* height);

// Extract all text from a page as UTF-8 string
// Returns: Allocated string or NULL on error (caller must free with IPDF_StreamingIO_FreeString)
FPDF_EXPORT char* FPDF_CALLCONV IPDF_StreamingIO_GetPageText(
    FPDF_DOCUMENT document,
    int page_index);

// Render page to RGBA bitmap buffer
// width, height: Output dimensions in pixels
// out_size: Receives buffer size in bytes (width * height * 4)
// Returns: Allocated RGBA buffer or NULL on error (caller must free with IPDF_StreamingIO_FreeString)
FPDF_EXPORT unsigned char* FPDF_CALLCONV IPDF_StreamingIO_RenderPage(
    FPDF_DOCUMENT document,
    int page_index,
    int width,
    int height,
    unsigned long* out_size);

// Free string or buffer allocated by streaming I/O functions
FPDF_EXPORT void FPDF_CALLCONV IPDF_StreamingIO_FreeString(void* ptr);

#ifdef __cplusplus
}
#endif

#endif  // PUBLIC_IPDF_STREAMINGIO_H_
