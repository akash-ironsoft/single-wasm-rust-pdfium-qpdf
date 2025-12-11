/**
 * PDFium WASM Wrapper (using Emscripten generated module)
 * Simple JavaScript API for auto-pqdfium-rs WASM module
 */

class PdfiumWasm {
  constructor() {
    this.Module = null;
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
      console.log('Loading PDFium WASM module...');

      // Load the Emscripten-generated module
      this.Module = await createPdfiumModule({
        locateFile: (path) => {
          // Make sure WASM file is loaded from correct location
          if (path.endsWith('.wasm')) {
            return 'auto_pqdfium_rs.wasm';
          }
          return path;
        },
        print: (text) => console.log('[PDFium]', text),
        printErr: (text) => console.error('[PDFium Error]', text),
      });

      // Initialize PDFium library
      const initResult = this.Module._pdfium_wasm_initialize();
      if (initResult !== 1) {
        throw new Error('Failed to initialize PDFium library');
      }

      this.initialized = true;
      console.log('✅ PDFium WASM initialized successfully');
    } catch (error) {
      console.error('❌ Failed to initialize PDFium WASM:', error);
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
      // Allocate memory for PDF data in WASM
      const pdfPtr = this.Module._malloc(pdfBytes.length);
      if (!pdfPtr) {
        throw new Error('Failed to allocate memory');
      }

      // Copy PDF bytes to WASM memory
      // Access heap through HEAP8/HEAPU8 view
      const heapView = new Uint8Array(this.Module.HEAPU8.buffer, pdfPtr, pdfBytes.length);
      heapView.set(pdfBytes);

      // Call extract_text function
      const resultPtr = this.Module._pdfium_wasm_extract_text(pdfPtr, pdfBytes.length);

      // Free the PDF buffer
      this.Module._free(pdfPtr);

      if (!resultPtr) {
        throw new Error('Text extraction failed - PDF may be corrupted or encrypted');
      }

      // Read the result string from WASM memory using Emscripten helper
      const text = this.Module.UTF8ToString(resultPtr);

      // Free the result string
      this.Module._pdfium_wasm_free_string(resultPtr);

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
      // Allocate memory for PDF data
      const pdfPtr = this.Module._malloc(pdfBytes.length);
      if (!pdfPtr) {
        throw new Error('Failed to allocate memory');
      }

      // Copy PDF bytes to WASM memory
      const heapView = new Uint8Array(this.Module.HEAPU8.buffer, pdfPtr, pdfBytes.length);
      heapView.set(pdfBytes);

      // Call pdf_to_json function
      const resultPtr = this.Module._pdfium_wasm_pdf_to_json(pdfPtr, pdfBytes.length);

      // Free the PDF buffer
      this.Module._free(pdfPtr);

      if (!resultPtr) {
        throw new Error('PDF to JSON conversion failed');
      }

      // Read the result JSON string using Emscripten helper
      const jsonStr = this.Module.UTF8ToString(resultPtr);

      // Free the result string
      this.Module._pdfium_wasm_free_string(resultPtr);

      // Parse and return JSON
      return JSON.parse(jsonStr);
    } catch (error) {
      console.error('Error converting PDF to JSON:', error);
      throw error;
    }
  }

  /**
   * Cleanup PDFium resources
   */
  cleanup() {
    if (this.initialized && this.Module) {
      this.Module._pdfium_wasm_cleanup();
      this.initialized = false;
      console.log('PDFium cleaned up');
    }
  }
}

// Export for use in HTML
window.PdfiumWasm = PdfiumWasm;
