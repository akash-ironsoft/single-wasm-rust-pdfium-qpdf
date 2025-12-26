// Copyright 2024 PDFium-QPDF Streaming Integration. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef PUBLIC_IPDF_QPDF_STREAMING_H_
#define PUBLIC_IPDF_QPDF_STREAMING_H_

#include "fpdfview.h"

#ifdef __cplusplus
extern "C" {
#endif

// =============================================================================
// QPDF Streaming I/O API
// =============================================================================
//
// This API provides memory-efficient streaming access to QPDF functionality,
// minimizing WASM heap usage by using callbacks instead of full buffer copies.
//
// Design mirrors PDFium's IPDF_StreamingIO_* pattern for consistency.
// =============================================================================

// -----------------------------------------------------------------------------
// Type Definitions
// -----------------------------------------------------------------------------

// Opaque handle to a QPDF document loaded via streaming
typedef void* QPDF_STREAM_HANDLE;

// Read callback function type
// Uses int (32-bit signed) for WASM32 compatibility.
// Supports files up to 2GB which is sufficient for most PDFs.
// Parameters:
//   user_data: User context passed to load function
//   position:  Byte offset to read from (signed for proper seek handling)
//   buffer:    Output buffer to fill
//   size:      Number of bytes to read
// Returns: Number of bytes actually read, or -1 on error
typedef int (*QPDF_ReadBlockCallback)(void* user_data,
                                       int position,
                                       unsigned char* buffer,
                                       int size);

// Write callback function type
// Parameters:
//   user_data: User context passed to save/write function
//   data:      Data to write
//   size:      Number of bytes to write
// Returns: 1 on success, 0 on failure
typedef int (*QPDF_WriteBlockCallback)(void* user_data,
                                        const void* data,
                                        int size);

// -----------------------------------------------------------------------------
// Save/Write Flags
// -----------------------------------------------------------------------------

// No special flags - default behavior
#define QPDF_STREAM_FLAG_NONE               0x0000

// Generate object streams (more compact output)
#define QPDF_STREAM_FLAG_OBJECT_STREAMS     0x0001

// Compress streams with flate
#define QPDF_STREAM_FLAG_COMPRESS           0x0002

// Create linearized (web-optimized) output
#define QPDF_STREAM_FLAG_LINEARIZE          0x0004

// Preserve encryption from source
#define QPDF_STREAM_FLAG_PRESERVE_ENCRYPT   0x0008

// Generate deterministic ID (for reproducible builds)
#define QPDF_STREAM_FLAG_DETERMINISTIC_ID   0x0010

// QDF mode (human-readable, for debugging)
#define QPDF_STREAM_FLAG_QDF                0x0020

// -----------------------------------------------------------------------------
// Document Loading (Streaming Input)
// -----------------------------------------------------------------------------

// Load a PDF document using streaming callbacks.
// This avoids copying the entire PDF into WASM memory.
//
// Parameters:
//   file_size:          Total file size in bytes (supports up to 2GB)
//   read_callback:      Function to read data blocks on-demand
//   user_data:          User context passed to callback
//   password:           Optional password (NULL if not encrypted)
//
// Returns: Handle to loaded document, or NULL on failure
//
// Memory usage: O(1) - only buffers for current operation, not entire file
//
// IMPORTANT: The read_callback may be called with any position within
// [0, file_size). Implementations must handle random access reads.
FPDF_EXPORT QPDF_STREAM_HANDLE FPDF_CALLCONV IPDF_QPDF_StreamingOpen(
    int file_size,
    QPDF_ReadBlockCallback read_callback,
    void* user_data,
    const char* password);

// Close a streaming document and release resources.
// Safe to call with NULL handle.
FPDF_EXPORT void FPDF_CALLCONV IPDF_QPDF_StreamingClose(
    QPDF_STREAM_HANDLE handle);

// -----------------------------------------------------------------------------
// Document Saving (Streaming Output)
// -----------------------------------------------------------------------------

// Save a document using streaming output callback.
// Data is written in chunks via the callback, never fully buffered.
//
// Parameters:
//   handle:          Document handle from IPDF_QPDF_StreamingOpen
//   write_callback:  Function to receive output data chunks
//   user_data:       User context passed to callback
//   flags:           Combination of QPDF_STREAM_FLAG_* values
//
// Returns: 1 on success, 0 on failure
//
// Memory usage: O(chunk_size) - typically 4KB-64KB per write
FPDF_EXPORT int FPDF_CALLCONV IPDF_QPDF_StreamingSave(
    QPDF_STREAM_HANDLE handle,
    QPDF_WriteBlockCallback write_callback,
    void* user_data,
    int flags);

// -----------------------------------------------------------------------------
// JSON Conversion (Streaming Both Input and Output)
// -----------------------------------------------------------------------------

// Convert PDF to QPDF JSON format using streaming I/O for both input and output.
// This is the most memory-efficient way to convert large PDFs to JSON.
//
// Parameters:
//   file_size:        Total PDF file size in bytes (supports up to 2GB)
//   read_callback:    Function to read PDF data blocks
//   read_user_data:   User context for read callback
//   json_version:     JSON format version (1 or 2)
//   write_callback:   Function to receive JSON output chunks
//   write_user_data:  User context for write callback
//
// Returns: 1 on success, 0 on failure
//
// Memory usage: O(max(input_chunk, output_chunk))
FPDF_EXPORT int FPDF_CALLCONV IPDF_QPDF_StreamingToJSON(
    int file_size,
    QPDF_ReadBlockCallback read_callback,
    void* read_user_data,
    int json_version,
    QPDF_WriteBlockCallback write_callback,
    void* write_user_data);

// -----------------------------------------------------------------------------
// Document Information (Query Operations)
// -----------------------------------------------------------------------------

// Get the number of pages in the document.
// Returns: Page count, or 0 on error
FPDF_EXPORT int FPDF_CALLCONV IPDF_QPDF_StreamingGetPageCount(
    QPDF_STREAM_HANDLE handle);

// Get PDF version string (e.g., "1.7", "2.0")
// Returns: Allocated string (caller must free with IPDF_QPDF_StreamingFreeString)
//          or NULL on error
FPDF_EXPORT char* FPDF_CALLCONV IPDF_QPDF_StreamingGetPDFVersion(
    QPDF_STREAM_HANDLE handle);

// Check if document is encrypted
// Returns: 1 if encrypted, 0 if not, -1 on error
FPDF_EXPORT int FPDF_CALLCONV IPDF_QPDF_StreamingIsEncrypted(
    QPDF_STREAM_HANDLE handle);

// Check if document is linearized
// Returns: 1 if linearized, 0 if not, -1 on error
FPDF_EXPORT int FPDF_CALLCONV IPDF_QPDF_StreamingIsLinearized(
    QPDF_STREAM_HANDLE handle);

// -----------------------------------------------------------------------------
// Error Handling
// -----------------------------------------------------------------------------

// Get last error message (thread-local)
// Returns: Error message string or NULL if no error
//          String is valid until next QPDF streaming call on same thread
FPDF_EXPORT const char* FPDF_CALLCONV IPDF_QPDF_StreamingGetLastError(void);

// -----------------------------------------------------------------------------
// Memory Management
// -----------------------------------------------------------------------------

// Free string allocated by streaming functions
FPDF_EXPORT void FPDF_CALLCONV IPDF_QPDF_StreamingFreeString(char* str);

// Free buffer allocated by streaming functions
FPDF_EXPORT void FPDF_CALLCONV IPDF_QPDF_StreamingFreeBuffer(void* buffer);

#ifdef __cplusplus
}
#endif

#endif  // PUBLIC_IPDF_QPDF_STREAMING_H_
