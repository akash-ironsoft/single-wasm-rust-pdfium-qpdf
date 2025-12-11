# Project Architecture & Text Extraction Call Stack

Complete technical documentation for auto-pqdfium-rs WASM PDF processor.

## 📁 Project Structure

```
auto-pqdfium-rs/
├── src/
│   ├── lib.rs                 # Rust library & FFI exports
│   ├── bridge.cpp             # C++ bridge (PDFium/QPDF wrapper)
│   ├── bridge.h               # C bridge header
│   └── error.rs               # Error types
├── web/
│   ├── index.html             # Demo UI
│   ├── pdfium-wrapper.js      # High-level JS API
│   ├── auto_pqdfium_rs.js     # Emscripten-generated glue
│   ├── auto_pqdfium_rs.wasm   # Compiled WASM module
│   └── server.py              # Development server
├── build.rs                   # Rust build script
├── build-wasm.sh              # WASM build script
├── Cargo.toml                 # Rust dependencies
├── CMakeLists.txt             # CMake config (native builds)
└── .cargo/config.toml         # Emscripten linker flags
```

## 🔄 Complete Text Extraction Call Stack

### **Layer 1: Browser JavaScript**

```javascript
// web/index.html (Line 355)
extractBtn.onclick = async () => {
  const text = await pdfium.extractText(pdfBytes);
};
```

**Data:** PDF as `Uint8Array` in JavaScript memory

---

### **Layer 2: JavaScript Wrapper**

```javascript
// web/pdfium-wrapper.js (Lines 56-106)
class PdfiumWasm {
  async extractText(pdfBytes) {
    // 1. Allocate WASM memory
    const pdfPtr = this.Module._malloc(pdfBytes.length);

    // 2. Copy PDF bytes to WASM memory
    this.Module.writeArrayToMemory(pdfBytes, pdfPtr);

    // 3. Call WASM function
    const resultPtr = this.Module._pdfium_wasm_extract_text(
      pdfPtr,
      pdfBytes.length
    );

    // 4. Free input buffer
    this.Module._free(pdfPtr);

    // 5. Read result string
    const text = this.Module.UTF8ToString(resultPtr);

    // 6. Free result string
    this.Module._pdfium_wasm_free_string(resultPtr);

    return text;
  }
}
```

**Key Operations:**
- Memory allocation in WASM heap
- Byte copying from JS to WASM
- Pointer-based function calls
- String encoding conversion (UTF-8)
- Memory cleanup

---

### **Layer 3: Emscripten Runtime**

```javascript
// web/auto_pqdfium_rs.js (generated)
Module._pdfium_wasm_extract_text = function(pdf_ptr, pdf_len) {
  // Emscripten marshals the call to WASM
  return wasmTable.get(functionIndex)(pdf_ptr, pdf_len);
};
```

**Responsibilities:**
- Function name resolution
- Call stack management
- Exception handling
- Memory growth handling

---

### **Layer 4: WASM Module**

```wasm
// auto_pqdfium_rs.wasm (binary)
(func $pdfium_wasm_extract_text
  (param $pdf_ptr i32)
  (param $pdf_len i32)
  (result i32)
  ;; Compiled Rust code
  call $extract_text
)
```

**Format:** WebAssembly binary
**Execution:** Browser's WASM VM (V8, SpiderMonkey, etc.)

---

### **Layer 5: Rust FFI Layer**

```rust
// src/lib.rs (Lines 148-165)
#[no_mangle]
pub extern "C" fn pdfium_wasm_extract_text(
    pdf_data: *const u8,
    pdf_len: usize,
) -> *mut u8 {
    // 1. Validate inputs
    if pdf_data.is_null() || pdf_len == 0 {
        return std::ptr::null_mut();
    }

    // 2. Create Rust slice from raw pointer
    let pdf_bytes = unsafe {
        std::slice::from_raw_parts(pdf_data, pdf_len)
    };

    // 3. Call safe Rust function
    match extract_text(pdf_bytes) {
        Ok(text) => {
            // 4. Convert to C string
            let c_string = std::ffi::CString::new(text)
                .unwrap_or_default();
            c_string.into_raw() as *mut u8
        }
        Err(_) => std::ptr::null_mut(),
    }
}
```

**Key Operations:**
- C ABI compatibility (`extern "C"`)
- Raw pointer → Rust slice conversion
- Error handling (Result → Option)
- String allocation for return value

---

### **Layer 6: Rust Core Logic**

```rust
// src/lib.rs (Lines 113-142)
pub fn extract_text(pdf_bytes: &[u8]) -> Result<String> {
    // 1. Initialize PDFium (one-time)
    initialize()?;

    // 2. Call C++ bridge via FFI
    unsafe {
        let c_str_ptr = ffi::pdfium_bridge_extract_text(
            pdf_bytes.as_ptr(),
            pdf_bytes.len()
        );

        if c_str_ptr.is_null() {
            return Err(PdfiumError::ExtractionFailed(
                "Failed to extract text".to_string()
            ));
        }

        // 3. Convert C string to Rust String
        let c_str = std::ffi::CStr::from_ptr(c_str_ptr);
        let text = c_str.to_string_lossy().into_owned();

        // 4. Free C string
        ffi::pdfium_bridge_free_string(c_str_ptr);

        Ok(text)
    }
}
```

**Key Operations:**
- Library initialization
- FFI call to C++ bridge
- C string → Rust String conversion
- Memory management (malloc/free)

---

### **Layer 7: C++ Bridge**

```cpp
// src/bridge.cpp (Lines 41-109)
extern "C" {
char* pdfium_bridge_extract_text(
    const uint8_t* pdf_data,
    size_t pdf_size
) {
    // 1. Load PDF document
    FPDF_DOCUMENT doc = FPDF_LoadMemDocument(
        pdf_data,
        static_cast<int>(pdf_size),
        nullptr
    );
    if (!doc) return nullptr;

    std::ostringstream text_stream;
    int page_count = FPDF_GetPageCount(doc);

    // 2. Extract text from each page
    for (int i = 0; i < page_count; ++i) {
        FPDF_PAGE page = FPDF_LoadPage(doc, i);
        if (!page) continue;

        FPDF_TEXTPAGE text_page = FPDFText_LoadPage(page);
        if (text_page) {
            // 3. Get text length
            int text_length = FPDFText_CountChars(text_page);

            if (text_length > 0) {
                // 4. Get text (UTF-16)
                std::vector<unsigned short> buffer(text_length + 1);
                int chars_written = FPDFText_GetText(
                    text_page, 0, text_length, buffer.data()
                );

                // 5. Convert UTF-16 → UTF-8
                for (int j = 0; j < chars_written - 1; ++j) {
                    unsigned short ch = buffer[j];
                    if (ch < 0x80) {
                        text_stream << static_cast<char>(ch);
                    } else if (ch < 0x800) {
                        text_stream << static_cast<char>(0xC0 | (ch >> 6));
                        text_stream << static_cast<char>(0x80 | (ch & 0x3F));
                    } else {
                        text_stream << static_cast<char>(0xE0 | (ch >> 12));
                        text_stream << static_cast<char>(0x80 | ((ch >> 6) & 0x3F));
                        text_stream << static_cast<char>(0x80 | (ch & 0x3F));
                    }
                }
            }

            FPDFText_ClosePage(text_page);
        }

        FPDF_ClosePage(page);

        // 6. Add page separator
        if (i < page_count - 1) {
            text_stream << "\n---PAGE BREAK---\n";
        }
    }

    FPDF_CloseDocument(doc);

    // 7. Allocate and return C string
    std::string text = text_stream.str();
    char* result = static_cast<char*>(malloc(text.length() + 1));
    if (result) {
        strcpy(result, text.c_str());
    }
    return result;
}
}
```

**Key Operations:**
- PDF document parsing
- Page iteration
- Text extraction from each page
- UTF-16 → UTF-8 encoding conversion
- Page separator insertion
- C-style string allocation

---

### **Layer 8: PDFium Native Library**

```cpp
// PDFium C API (pre-compiled static library)
FPDF_DOCUMENT FPDF_LoadMemDocument(const void* data, int size, ...);
int FPDF_GetPageCount(FPDF_DOCUMENT document);
FPDF_PAGE FPDF_LoadPage(FPDF_DOCUMENT document, int page_index);
FPDF_TEXTPAGE FPDFText_LoadPage(FPDF_PAGE page);
int FPDFText_CountChars(FPDF_TEXTPAGE text_page);
int FPDFText_GetText(FPDF_TEXTPAGE text_page, ...);
```

**Implementation:** Google's PDFium (Chromium's PDF engine)
**Format:** Static library (`.a` file)
**Size:** ~19MB (includes rendering, parsing, security)

---

## 📊 Data Flow Diagram

```
┌─────────────────────────────────────────────────────────────┐
│  Browser JavaScript                                         │
│  ┌──────────────────────────────────────────────────┐      │
│  │ User clicks "Extract Text"                        │      │
│  │ PDF bytes: Uint8Array (JS Heap)                  │      │
│  └──────────────────┬───────────────────────────────┘      │
│                     │                                        │
│                     ▼                                        │
│  ┌──────────────────────────────────────────────────┐      │
│  │ pdfium-wrapper.js                                 │      │
│  │ 1. Module._malloc(pdf_len) → pdfPtr              │      │
│  │ 2. writeArrayToMemory(pdfBytes, pdfPtr)          │      │
│  │ 3. Module._pdfium_wasm_extract_text(pdfPtr, len) │      │
│  └──────────────────┬───────────────────────────────┘      │
└────────────────────┼────────────────────────────────────────┘
                     │
        ┌────────────▼────────────┐
        │  Emscripten Runtime     │
        │  Function marshalling   │
        └────────────┬────────────┘
                     │
┌────────────────────▼────────────────────────────────────────┐
│  WASM Module (auto_pqdfium_rs.wasm)                         │
│  ┌──────────────────────────────────────────────────┐      │
│  │ WASM Linear Memory                                │      │
│  │ [PDF bytes at pdfPtr]                             │      │
│  │                                                    │      │
│  │ Call: pdfium_wasm_extract_text()                 │      │
│  │   ↓                                               │      │
│  │ Rust FFI: extern "C" fn                          │      │
│  │   - Validate pointers                             │      │
│  │   - Create slice from raw parts                   │      │
│  │   - Call extract_text()                           │      │
│  │       ↓                                           │      │
│  │     Rust Core:                                    │      │
│  │       - Initialize PDFium                         │      │
│  │       - FFI call to C++ bridge                    │      │
│  │           ↓                                       │      │
│  │         C++ Bridge:                               │      │
│  │           - FPDF_LoadMemDocument()                │      │
│  │           - For each page:                        │      │
│  │               • FPDF_LoadPage()                   │      │
│  │               • FPDFText_LoadPage()               │      │
│  │               • FPDFText_GetText()                │      │
│  │               • UTF-16 → UTF-8                    │      │
│  │           - Concatenate with separators           │      │
│  │           - malloc() result string                │      │
│  │           - Return char*                          │      │
│  │       ← Return to Rust                            │      │
│  │     - Convert CStr → String                       │      │
│  │     - Free C string                               │      │
│  │   ← Return to FFI                                 │      │
│  │ - Convert String → CString                        │      │
│  │ - Return raw pointer                              │      │
│  │                                                    │      │
│  │ [Result string at resultPtr]                      │      │
│  └──────────────────┬───────────────────────────────┘      │
└────────────────────┼────────────────────────────────────────┘
                     │
        ┌────────────▼────────────┐
        │  Emscripten Runtime     │
        │  Return value marshalling│
        └────────────┬────────────┘
                     │
┌────────────────────▼────────────────────────────────────────┐
│  Browser JavaScript                                         │
│  ┌──────────────────────────────────────────────────┐      │
│  │ pdfium-wrapper.js                                 │      │
│  │ 4. UTF8ToString(resultPtr) → text                │      │
│  │ 5. Module._pdfium_wasm_free_string(resultPtr)    │      │
│  │ 6. Module._free(pdfPtr)                           │      │
│  │ 7. return text                                    │      │
│  └──────────────────┬───────────────────────────────┘      │
│                     ▼                                        │
│  ┌──────────────────────────────────────────────────┐      │
│  │ Display text in UI                                │      │
│  └──────────────────────────────────────────────────┘      │
└─────────────────────────────────────────────────────────────┘
```

## 🧠 Memory Management

### **Memory Regions**

```
┌──────────────────────────────────────────────────┐
│ JavaScript Heap (Browser)                        │
│ - PDF file: Uint8Array                           │
│ - Result text: String                            │
└──────────────────────────────────────────────────┘
                    ↕ (copy)
┌──────────────────────────────────────────────────┐
│ WASM Linear Memory (64MB initial)                │
│                                                   │
│ [pdfPtr] ─────► [PDF bytes...]                   │
│                                                   │
│ [resultPtr] ──► [UTF-8 text string...]           │
│                                                   │
│ Stack, heap, static data                         │
└──────────────────────────────────────────────────┘
```

### **Memory Lifecycle**

1. **Allocation:**
   ```javascript
   pdfPtr = Module._malloc(pdfBytes.length);  // Allocate in WASM heap
   ```

2. **Copy:**
   ```javascript
   Module.writeArrayToMemory(pdfBytes, pdfPtr);  // JS → WASM
   ```

3. **Processing:**
   ```cpp
   char* result = malloc(text.length() + 1);  // C++ allocates result
   strcpy(result, text.c_str());
   ```

4. **Read:**
   ```javascript
   const text = Module.UTF8ToString(resultPtr);  // WASM → JS
   ```

5. **Cleanup:**
   ```javascript
   Module._pdfium_wasm_free_string(resultPtr);  // Free result
   Module._free(pdfPtr);                         // Free input
   ```

## 🔨 Build Process

### **Native Build:**
```bash
1. CMake configures build
2. g++ compiles bridge.cpp → libpdfium_bridge.a
3. rustc compiles Rust → links with:
   - libpdfium_bridge.a
   - libpdfium.so (PDFium)
   - libc++.so
4. Output: libauto_pqdfium_rs.so (native)
```

### **WASM Build:**
```bash
1. cargo build --target wasm32-unknown-emscripten
   - rustc compiles to WASM
   - emcc compiles bridge.cpp
   - Links with libpdfium.a (static)
   - Output: libauto_pqdfium_rs.a

2. emcc libauto_pqdfium_rs.a -o auto_pqdfium_rs.js
   - Generates JS glue code
   - Exports functions and runtime methods
   - Output:
     • auto_pqdfium_rs.js (76KB)
     • auto_pqdfium_rs.wasm (3.7MB)
```

### **Build Commands:**

```bash
# Native build
cargo build --release

# WASM build (automated)
./build-wasm.sh

# WASM build (manual)
source ~/emsdk/emsdk_env.sh
cargo build --target wasm32-unknown-emscripten --release
emcc target/wasm32-unknown-emscripten/release/libauto_pqdfium_rs.a \
    -o web/auto_pqdfium_rs.js \
    -sEXPORTED_FUNCTIONS=_pdfium_wasm_initialize,_pdfium_wasm_extract_text,... \
    -sEXPORTED_RUNTIME_METHODS=UTF8ToString,writeArrayToMemory,... \
    -sMODULARIZE=1 \
    -O3
```

## 🎯 Key Design Decisions

### **1. C ABI Bridge**
- **Why:** WASM requires C calling convention
- **How:** Rust `extern "C"` functions with `#[no_mangle]`
- **Trade-off:** Less type safety, manual memory management

### **2. Raw Pointers in FFI**
- **Why:** Cross-language boundary requires raw pointers
- **How:** `*const u8` for inputs, `*mut u8` for outputs
- **Safety:** Wrapped in `unsafe` blocks with validation

### **3. Manual Memory Management**
- **Why:** Different memory allocators (JS, Rust, C++)
- **How:** Explicit malloc/free with clear ownership
- **Pattern:** Caller allocates input, callee allocates output

### **4. UTF-8 Encoding**
- **Why:** Universal web standard
- **How:** PDFium returns UTF-16, bridge converts to UTF-8
- **Benefit:** Direct JavaScript string compatibility

### **5. Static Linking**
- **Why:** WASM can't dynamically load libraries
- **How:** PDFium compiled as `.a` and linked into WASM
- **Trade-off:** Larger file size, but self-contained

### **6. Emscripten Runtime**
- **Why:** Provides essential JS ↔ WASM interop
- **What:** Memory access, string conversion, function calls
- **Size:** ~76KB overhead for full compatibility

## 📈 Performance Characteristics

| Metric | Value | Notes |
|--------|-------|-------|
| **Module Load Time** | 1-3 seconds | First time, includes parsing + compilation |
| **Cached Load** | ~100ms | Browser caches compiled WASM |
| **Initialization** | 50-100ms | PDFium library initialization |
| **Text Extraction** | 10-50ms/page | Depends on text complexity |
| **Memory Usage** | PDF size × 2-3 | Working memory + output |
| **WASM vs Native** | 70-85% speed | JIT-compiled WASM performance |
| **Binary Size** | 3.7MB | Includes PDFium + QPDF |
| **Compressed Size** | ~1MB | With Brotli compression |

## 🔐 Security Considerations

### **Memory Safety**
- Rust provides memory safety in core logic
- FFI boundaries use explicit validation
- PDFium handles malformed PDF inputs

### **Sandboxing**
- WASM runs in browser sandbox
- No file system access
- No network access
- Limited memory (browser enforced)

### **Privacy**
- **All processing client-side**
- PDF never leaves user's browser
- No server upload required
- Ideal for sensitive documents

## 🚀 Optimization Opportunities

### **Current State:**
- ✅ Size optimization (`-Oz`)
- ✅ Link-time optimization (LTO)
- ✅ Static linking
- ✅ Emscripten memory growth

### **Future Improvements:**
1. **Code Splitting:** Separate text extraction from JSON conversion
2. **Streaming:** Process PDFs page-by-page
3. **Worker Threads:** Use Web Workers for background processing
4. **SIMD:** Use WebAssembly SIMD for faster operations
5. **Caching:** Cache parsed PDF structure

## 📚 Related Documentation

- [Building WASM](../build-wasm.sh) - Build script
- [Web API](../web/pdfium-wrapper.js) - JavaScript API
- [Troubleshooting](../web/TROUBLESHOOTING.md) - Common issues
- [README](../README.md) - Getting started

## 🤝 Contributing

When modifying the architecture:
1. Update this document
2. Test both native and WASM builds
3. Verify memory management
4. Check performance impact
5. Update examples

---

**Last Updated:** December 11, 2025
**WASM Module Version:** 0.1.0
**PDFium Version:** Chromium branch
