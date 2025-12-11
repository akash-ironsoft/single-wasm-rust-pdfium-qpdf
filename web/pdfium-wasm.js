/**
 * PDFium WASM Wrapper
 * Simple JavaScript API for auto-pqdfium-rs WASM module
 */

class PdfiumWasm {
  constructor() {
    this.module = null;
    this.initialized = false;
  }

  /**
   * Initialize the WASM module
   * @returns {Promise<void>}
   */
  async init() {
    if (this.initialized) {
      return;
    }

    try {
      // Load the WASM module
      const response = await fetch('auto_pqdfium_rs.wasm');
      const wasmBytes = await response.arrayBuffer();

      // Define imports for the WASM module
      const imports = {
        env: {
          // Memory will be provided by the module
        },
        wasi_snapshot_preview1: {
          // Stub WASI functions (PDFium doesn't use them)
          proc_exit: () => {},
          fd_write: () => 0,
          fd_close: () => 0,
          fd_seek: () => 0,
        }
      };

      // Instantiate the WASM module
      const result = await WebAssembly.instantiate(wasmBytes, imports);
      this.module = result.instance;

      // Initialize PDFium library
      const initResult = this.module.exports.pdfium_wasm_initialize();
      if (initResult !== 1) {
        throw new Error('Failed to initialize PDFium');
      }

      this.initialized = true;
      console.log('PDFium WASM initialized successfully');
    } catch (error) {
      console.error('Failed to initialize PDFium WASM:', error);
      throw error;
    }
  }

  /**
   * Extract text from a PDF document
   * @param {Uint8Array} pdfBytes - PDF file as byte array
   * @returns {Promise<string>} Extracted text
   */
  async extractText(pdfBytes) {
    if (!this.initialized) {
      throw new Error('PDFium not initialized. Call init() first.');
    }

    try {
      const exports = this.module.exports;

      // Allocate memory for PDF data in WASM
      const pdfPtr = exports.malloc(pdfBytes.length);
      if (!pdfPtr) {
        throw new Error('Failed to allocate memory');
      }

      // Copy PDF bytes to WASM memory
      const memory = new Uint8Array(exports.memory.buffer);
      memory.set(pdfBytes, pdfPtr);

      // Call extract_text function
      const resultPtr = exports.pdfium_wasm_extract_text(pdfPtr, pdfBytes.length);

      // Free the PDF buffer
      exports.free(pdfPtr);

      if (!resultPtr) {
        throw new Error('Text extraction failed');
      }

      // Read the result string from WASM memory
      const text = this.readCString(resultPtr);

      // Free the result string
      exports.pdfium_wasm_free_string(resultPtr);

      return text;
    } catch (error) {
      console.error('Error extracting text:', error);
      throw error;
    }
  }

  /**
   * Convert PDF to JSON format
   * @param {Uint8Array} pdfBytes - PDF file as byte array
   * @returns {Promise<object>} PDF structure as JSON object
   */
  async pdfToJson(pdfBytes) {
    if (!this.initialized) {
      throw new Error('PDFium not initialized. Call init() first.');
    }

    try {
      const exports = this.module.exports;

      // Allocate memory for PDF data
      const pdfPtr = exports.malloc(pdfBytes.length);
      if (!pdfPtr) {
        throw new Error('Failed to allocate memory');
      }

      // Copy PDF bytes to WASM memory
      const memory = new Uint8Array(exports.memory.buffer);
      memory.set(pdfBytes, pdfPtr);

      // Call pdf_to_json function
      const resultPtr = exports.pdfium_wasm_pdf_to_json(pdfPtr, pdfBytes.length);

      // Free the PDF buffer
      exports.free(pdfPtr);

      if (!resultPtr) {
        throw new Error('PDF to JSON conversion failed');
      }

      // Read the result JSON string
      const jsonStr = this.readCString(resultPtr);

      // Free the result string
      exports.pdfium_wasm_free_string(resultPtr);

      // Parse and return JSON
      return JSON.parse(jsonStr);
    } catch (error) {
      console.error('Error converting PDF to JSON:', error);
      throw error;
    }
  }

  /**
   * Read a null-terminated C string from WASM memory
   * @param {number} ptr - Pointer to string in WASM memory
   * @returns {string} JavaScript string
   */
  readCString(ptr) {
    const memory = new Uint8Array(this.module.exports.memory.buffer);
    let end = ptr;

    // Find null terminator
    while (memory[end] !== 0) {
      end++;
    }

    // Convert to string
    const bytes = memory.subarray(ptr, end);
    return new TextDecoder('utf-8').decode(bytes);
  }

  /**
   * Cleanup PDFium resources
   */
  cleanup() {
    if (this.initialized && this.module) {
      this.module.exports.pdfium_wasm_cleanup();
      this.initialized = false;
      console.log('PDFium cleaned up');
    }
  }
}

// Export for use in HTML
window.PdfiumWasm = PdfiumWasm;
