# auto-pqdfium-rs

A safe, minimal Rust wrapper for PDFium + QPDF using autocxx.

## Features

- **Text Extraction**: Extract text from PDF documents via PDFium
- **PDF to JSON**: Convert PDF structure to JSON using QPDF (version 2 format)
- **Safe API**: Memory-safe Rust interface with automatic cleanup
- **In-memory**: Works directly with byte arrays, no filesystem dependency
- **Minimal**: Simple, focused API with just two main functions

## Quick Start

```rust
use auto_pqdfium_rs::{extract_text, pdf_to_json};

// Load PDF bytes (from file, network, etc.)
let pdf_bytes = std::fs::read("document.pdf")?;

// Extract text
let text = extract_text(&pdf_bytes)?;
println!("Extracted text: {}", text);

// Convert to JSON
let json = pdf_to_json(&pdf_bytes)?;
println!("PDF as JSON: {}", json);
```

## API

### `extract_text(pdf_bytes: &[u8]) -> Result<String>`

Extracts text from all pages of a PDF document. Pages are separated by `---PAGE BREAK---`.

**Example:**
```rust
let text = extract_text(&pdf_bytes)?;
```

### `pdf_to_json(pdf_bytes: &[u8]) -> Result<String>`

Converts a PDF document to JSON format using QPDF (version 2 with comprehensive details).

**Example:**
```rust
let json = pdf_to_json(&pdf_bytes)?;
let parsed: serde_json::Value = serde_json::from_str(&json)?;
```

### `initialize() -> Result<()>`

Explicitly initialize PDFium (called automatically by other functions).

### `cleanup()`

Cleanup PDFium resources (optional, called automatically at program exit).

## Architecture

### C++ Bridge Layer

The crate uses a C-style bridge layer (`src/bridge.cpp`, `src/bridge.h`) that wraps:
- PDFium text extraction APIs (`fpdf_text.h`)
- QPDF JSON conversion (`ipdf_qpdf.h`)

Functions:
- `pdfium_bridge_initialize()` - Initialize PDFium
- `pdfium_bridge_extract_text()` - Extract text from PDF bytes
- `pdfium_bridge_pdf_to_json()` - Convert PDF to JSON
- `pdfium_bridge_free_string()` - Free allocated strings
- `pdfium_bridge_cleanup()` - Cleanup PDFium

### Build System

- **CMake** (`CMakeLists.txt`): Compiles the C++ bridge
- **build.rs**: Coordinates:
  - CMake build of bridge
  - autocxx code generation
  - Linking against PDFium shared library

### Dependencies

The crate links against:
- Custom PDFium build (shared library from `pdfium-workspace`)
- Chromium's libc++ (required by PDFium)
- System libraries (pthread, dl, m)

## Building

### Prerequisites

- Rust 1.70+
- CMake 3.15+
- C++17 compiler
- Custom PDFium + QPDF build at:
  `/home/akash/Dev/ironsoft/iron-universal/Universal.PdfEditor/pdfium-workspace/Universal.Pdfium`

### Build

```bash
cargo build --release
```

### Test

```bash
cargo test
```

All tests (7 integration tests + 3 doc tests) verify:
- Text extraction from sample PDF
- JSON conversion
- Error handling for invalid input
- Empty data handling

## Project Structure

```
auto-pqdfium-rs/
├── src/
│   ├── lib.rs           # Main Rust API
│   ├── error.rs         # Error types
│   ├── bridge.h         # C bridge header
│   └── bridge.cpp       # C++ bridge implementation
├── tests/
│   ├── integration_test.rs  # Integration tests
│   └── sample.pdf           # Test PDF
├── build.rs             # Build script
├── CMakeLists.txt       # CMake configuration
├── Cargo.toml           # Rust package manifest
└── README.md            # This file
```

## Technical Details

### Memory Management

- C++ bridge allocates strings with `malloc()` / QPDF allocator
- Rust side converts to owned `String` and immediately frees C strings
- PDFium library initialization uses `std::sync::Once` for thread-safety

### UTF-16 to UTF-8 Conversion

PDFium returns text as UTF-16LE. The bridge converts to UTF-8:
- ASCII (< 0x80): Direct conversion
- 2-byte UTF-8 (< 0x800): Proper encoding
- 3-byte UTF-8 (≥ 0x800): Full BMP support

### Error Handling

Functions return `Result<T, PdfiumError>` with error types:
- `InitializationFailed` - PDFium init error
- `InvalidData` - Empty input
- `ExtractionFailed(String)` - Text extraction error
- `ConversionFailed(String)` - JSON conversion error

## Runtime Requirements

The crate requires these shared libraries at runtime:
- `libpdfium.so` (from pdfium-workspace)
- `libc++.so` (Chromium's libc++)

The build script configures rpath to find these automatically.

## License

MIT OR Apache-2.0

## Acknowledgments

- [PDFium](https://pdfium.googlesource.com/pdfium/) - PDF rendering library by Google
- [QPDF](https://qpdf.sourceforge.io/) - PDF transformation library
- [autocxx](https://github.com/google/autocxx) - C++ interop for Rust
