# Build System Flowchart

## Overview: How `auto-pqdfium-rs` Builds

```
┌─────────────────────────────────────────────────────────────────────────┐
│                          USER RUNS: cargo build                         │
└────────────────────────────────┬────────────────────────────────────────┘
                                 │
                                 ▼
┌─────────────────────────────────────────────────────────────────────────┐
│                    CARGO READS: Cargo.toml                              │
│  - Dependencies: autocxx, cxx, thiserror                                │
│  - Build-dependencies: autocxx-build, cmake, cc                         │
└────────────────────────────────┬────────────────────────────────────────┘
                                 │
                                 ▼
┌─────────────────────────────────────────────────────────────────────────┐
│                    CARGO EXECUTES: build.rs                             │
│                   (Before compiling Rust code)                          │
└────────────────────────────────┬────────────────────────────────────────┘
                                 │
                    ┌────────────┴────────────┐
                    │                         │
                    ▼                         ▼
    ┌───────────────────────────┐  ┌──────────────────────────┐
    │   STEP 1: CMAKE BUILD     │  │   STEP 2: AUTOCXX        │
    │   (C++ Bridge)            │  │   (Rust Bindings)        │
    └────────────┬──────────────┘  └────────────┬─────────────┘
                 │                              │
                 │                              │
    ┌────────────▼──────────────────────────────▼──────────────┐
    │                                                           │
    │              STEP 3: CARGO LINK INSTRUCTIONS             │
    │                                                           │
    └────────────────────────────┬──────────────────────────────┘
                                 │
                                 ▼
    ┌─────────────────────────────────────────────────────────┐
    │           STEP 4: COMPILE RUST CODE                     │
    │           (src/lib.rs, src/error.rs)                    │
    └────────────────────────────┬────────────────────────────┘
                                 │
                                 ▼
    ┌─────────────────────────────────────────────────────────┐
    │           STEP 5: LINK EVERYTHING TOGETHER              │
    │           - Rust object files                           │
    │           - libpdfium_bridge.a                          │
    │           - autocxx generated code                      │
    │           - libpdfium.so                                │
    │           - libc++.so                                   │
    │           - System libs (pthread, dl, m)                │
    └────────────────────────────┬────────────────────────────┘
                                 │
                                 ▼
    ┌─────────────────────────────────────────────────────────┐
    │              FINAL BINARY: libauto_pqdfium_rs.so        │
    │              (or executable for tests/examples)         │
    └─────────────────────────────────────────────────────────┘
```

---

## Detailed Step-by-Step Flow

### STEP 1: CMAKE BUILD (C++ Bridge)

```
build.rs (line 14-16)
    │
    ├─> cmake::Config::new(".")
    │       │
    │       ▼
    │   CMakeLists.txt is processed
    │       │
    │       ├─> Set C++17 standard
    │       ├─> Include PDFium headers:
    │       │     - /path/to/pdfium/public/
    │       │     - /path/to/pdfium/third_party/Universal.Qpdf/include/
    │       │
    │       ├─> Compile: src/bridge.cpp
    │       │     │
    │       │     ▼
    │       │   bridge.cpp.o (object file)
    │       │
    │       ├─> Create: libpdfium_bridge.a
    │       │     (static library)
    │       │
    │       └─> Install to:
    │           target/debug/build/auto-pqdfium-rs-<hash>/out/lib/
    │
    └─> Returns: build directory path

build.rs (line 19-21) emits cargo instructions:
    - cargo:rustc-link-search=native=.../out/lib
    - cargo:rustc-link-lib=static=pdfium_bridge
```

**Output:** `libpdfium_bridge.a` containing compiled C++ bridge code

---

### STEP 2: AUTOCXX BINDINGS (Rust ↔ C++)

```
build.rs (line 24-37)
    │
    ├─> autocxx_build::Builder::new("src/lib.rs", include_paths)
    │       │
    │       ▼
    │   Parses src/lib.rs looking for:
    │       autocxx::include_cpp! {
    │           #include "bridge.h"
    │           generate!("pdfium_bridge_initialize")
    │           generate!("pdfium_bridge_extract_text")
    │           ...
    │       }
    │       │
    │       ├─> Runs C++ parser (libclang) on bridge.h
    │       │     │
    │       │     ▼
    │       │   Understands C function signatures:
    │       │     - int pdfium_bridge_initialize()
    │       │     - char* pdfium_bridge_extract_text(...)
    │       │     - void pdfium_bridge_free_string(char*)
    │       │
    │       ├─> Generates Rust FFI code:
    │       │     │
    │       │     ▼
    │       │   mod ffi {
    │       │       extern "C" {
    │       │           pub fn pdfium_bridge_initialize() -> c_int;
    │       │           pub fn pdfium_bridge_extract_text(...) -> *mut c_char;
    │       │           ...
    │       │       }
    │       │   }
    │       │
    │       ├─> Generates C++ glue code (cxx bridge)
    │       │
    │       └─> Compiles generated code into:
    │           libautocxx-pdfium-bridge.a
    │
    └─> Returns: compile configuration

build.rs (line 34-37) configures compiler:
    - -std=c++17
    - -Wno-unused-parameter
    - Compiles: autocxx-pdfium-bridge
```

**Output:**
- Rust module `ffi` with C function declarations
- `libautocxx-pdfium-bridge.a` (autocxx glue code)

---

### STEP 3: LINK INSTRUCTIONS

```
build.rs emits cargo instructions:
    │
    ├─> cargo:rustc-link-search=native=/path/to/pdfium/out/linux-x64-shared
    │       (Tell linker where to find libraries)
    │
    ├─> cargo:rustc-link-lib=static=pdfium_bridge
    │       (Link our C++ bridge)
    │
    ├─> cargo:rustc-link-lib=dylib=pdfium
    │       (Link PDFium shared library)
    │
    ├─> cargo:rustc-link-lib=dylib=c++
    │       (Link Chromium's libc++)
    │
    ├─> cargo:rustc-link-lib=stdc++
    │       (C++ standard library)
    │
    ├─> cargo:rustc-link-lib=pthread
    │       (POSIX threads)
    │
    ├─> cargo:rustc-link-lib=dl
    │       (Dynamic linking)
    │
    ├─> cargo:rustc-link-lib=m
    │       (Math library)
    │
    └─> cargo:rustc-link-arg=-Wl,-rpath,/path/to/pdfium/out/linux-x64-shared
            (Set runtime library search path)
```

**Output:** Linker configuration for final binary

---

### STEP 4: COMPILE RUST CODE

```
Cargo compiles Rust source files:
    │
    ├─> src/error.rs
    │       │
    │       ▼
    │   Defines: PdfiumError enum
    │   Uses: thiserror for derive(Error)
    │
    ├─> src/lib.rs
    │       │
    │       ├─> Expands autocxx::include_cpp! macro
    │       │       │
    │       │       ▼
    │       │   Creates mod ffi { ... } with FFI bindings
    │       │
    │       ├─> Compiles:
    │       │     - pub fn extract_text(...)
    │       │     - pub fn pdf_to_json(...)
    │       │     - pub fn initialize(...)
    │       │     - pub fn cleanup()
    │       │
    │       └─> Each function calls ffi::pdfium_bridge_* functions
    │
    └─> Output: Rust object files (.rlib)
```

**Output:** Compiled Rust code ready for linking

---

### STEP 5: FINAL LINKING

```
Linker (cc/lld) combines everything:

    Rust Object Files
    ┌──────────────────────┐
    │ auto_pqdfium_rs.rlib │
    │  - extract_text()    │
    │  - pdf_to_json()     │
    │  - initialize()      │
    └──────────┬───────────┘
               │
               │ calls
               ▼
    ┌──────────────────────────┐
    │ libautocxx-pdfium-      │
    │ bridge.a                 │
    │  (autocxx glue code)     │
    └──────────┬───────────────┘
               │
               │ calls
               ▼
    ┌──────────────────────────┐
    │ libpdfium_bridge.a       │
    │  - pdfium_bridge_       │
    │    initialize()          │
    │  - pdfium_bridge_       │
    │    extract_text()        │
    │  - pdfium_bridge_       │
    │    pdf_to_json()         │
    └──────────┬───────────────┘
               │
               │ calls
               ▼
    ┌──────────────────────────┐
    │ libpdfium.so (shared)    │
    │  - FPDF_InitLibrary...   │
    │  - FPDF_LoadMemDocument  │
    │  - FPDFText_LoadPage     │
    │  - IPDF_QPDF_PDFToJSON   │
    └──────────┬───────────────┘
               │
               │ depends on
               ▼
    ┌──────────────────────────┐
    │ libc++.so (shared)       │
    │  (Chromium's C++ lib)    │
    └──────────┬───────────────┘
               │
               │ depends on
               ▼
    ┌──────────────────────────┐
    │ System Libraries         │
    │  - libstdc++.so          │
    │  - libpthread.so         │
    │  - libdl.so              │
    │  - libm.so               │
    └──────────────────────────┘
               │
               ▼
    ┌──────────────────────────┐
    │ FINAL BINARY             │
    │  - libauto_pqdfium_rs.so │
    │  - or test executable    │
    └──────────────────────────┘
```

**Output:** Final library or executable

---

## Call Chain at Runtime

When you call `extract_text(&pdf_bytes)`:

```
User Code
    │
    │ extract_text(&[u8])
    ▼
┌─────────────────────────────────────┐
│ src/lib.rs (Rust)                   │
│  - initialize() [if needed]         │
│  - validate input                   │
│  - call ffi::pdfium_bridge_extract_ │
│    text(ptr, len)                   │
└────────────┬────────────────────────┘
             │ FFI boundary (Rust → C)
             ▼
┌─────────────────────────────────────┐
│ autocxx generated glue              │
│  - Type conversions                 │
│  - Safety checks                    │
└────────────┬────────────────────────┘
             │
             ▼
┌─────────────────────────────────────┐
│ src/bridge.cpp (C++)                │
│  pdfium_bridge_extract_text()       │
│    - Load PDF: FPDF_LoadMemDocument │
│    - Get pages: FPDF_GetPageCount   │
│    - For each page:                 │
│      - Load: FPDF_LoadPage          │
│      - Extract: FPDFText_LoadPage   │
│      - Get text: FPDFText_GetText   │
│      - Convert UTF-16 → UTF-8       │
│    - Return malloc'd string         │
└────────────┬────────────────────────┘
             │
             │ calls
             ▼
┌─────────────────────────────────────┐
│ libpdfium.so (PDFium library)       │
│  - PDF parsing                      │
│  - Text extraction                  │
│  - Memory management                │
└─────────────────────────────────────┘
             │
             │ returns
             ▼
┌─────────────────────────────────────┐
│ src/lib.rs (Rust)                   │
│  - Convert C string to Rust String  │
│  - Free C string                    │
│  - Return Result<String>            │
└────────────┬────────────────────────┘
             │
             ▼
User Code receives: Result<String, PdfiumError>
```

---

## Key Build System Components

### 1. **Cargo** (Rust Build System)
- Orchestrates entire build
- Runs build.rs before compiling Rust code
- Manages dependencies
- Invokes linker with instructions from build.rs

### 2. **build.rs** (Build Script)
- Runs CMake to build C++ bridge
- Configures autocxx for binding generation
- Emits linker instructions via `println!("cargo:...")`
- Coordinates between 3 different build systems (Cargo, CMake, autocxx)

### 3. **CMake** (C++ Build System)
- Compiles src/bridge.cpp
- Links headers from PDFium
- Creates libpdfium_bridge.a
- Independent of Rust/Cargo

### 4. **autocxx** (C++/Rust Bridge Generator)
- Parses C++ headers
- Generates safe Rust FFI code
- Handles type conversions
- Creates glue code for C++ interop

### 5. **Linker** (cc/lld)
- Combines all object files
- Resolves symbols
- Links shared libraries
- Sets rpath for runtime library discovery

---

## Build Artifacts Location

```
target/debug/
├── build/
│   └── auto-pqdfium-rs-<hash>/
│       ├── out/
│       │   ├── lib/
│       │   │   └── libpdfium_bridge.a      ← CMake output
│       │   ├── build/                       ← CMake build dir
│       │   └── build.rs.d                   ← Dependency info
│       └── output                            ← Build script logs
│
├── deps/
│   ├── libauto_pqdfium_rs.rlib              ← Compiled Rust library
│   ├── libauto_pqdfium_rs.so                ← Final shared lib
│   ├── libautocxx_pdfium_bridge.a           ← autocxx glue
│   └── integration_test-<hash>              ← Test executable
│
└── examples/
    └── basic_usage                           ← Example binary
```

---

## Environment Variables & Configuration

### Set by build.rs:
```bash
# Search paths for linker
RUSTFLAGS="-L /path/to/pdfium/out/linux-x64-shared"

# Libraries to link
-lpdfium_bridge (static)
-lpdfium (dynamic)
-lc++ (dynamic)
-lstdc++ -lpthread -ldl -lm

# Runtime library path
RPATH=/path/to/pdfium/out/linux-x64-shared
```

### User can override:
```bash
# Use different PDFium location
export PDFIUM_DIR=/custom/path/to/pdfium
cargo build
```

---

## Troubleshooting Build Issues

### Issue 1: "undefined symbol: FPDF_InitLibrary"
**Cause:** libpdfium.so not linked
**Solution:** Check build.rs emits `cargo:rustc-link-lib=dylib=pdfium`

### Issue 2: "libc++.so: cannot open shared object"
**Cause:** Runtime can't find libc++.so
**Solution:** Check rpath is set: `cargo:rustc-link-arg=-Wl,-rpath,...`

### Issue 3: "autocxx failed to parse header"
**Cause:** Include paths wrong or C++ syntax issue
**Solution:** Check include_paths in build.rs, ensure C++17 compatible

### Issue 4: "CMake Error: No rule to make target 'install'"
**Cause:** CMakeLists.txt missing install target
**Solution:** Add `install(TARGETS ...)` to CMakeLists.txt

---

## Summary

The build system is a **3-stage pipeline**:

1. **CMake Stage**: Compiles C++ bridge → `libpdfium_bridge.a`
2. **autocxx Stage**: Generates Rust FFI bindings → glue code
3. **Cargo/Linker Stage**: Compiles Rust + links everything → final binary

Each stage has clear inputs/outputs and they're coordinated by `build.rs`.
