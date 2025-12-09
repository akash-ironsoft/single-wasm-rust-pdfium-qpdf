# Build System Summary - Simple Visual Guide

## 🎯 One-Page Overview

### The Big Picture
```
┌─────────────┐
│   cargo     │ ◄── User runs this
│   build     │
└──────┬──────┘
       │
       ├──────────────────────────────┐
       │                              │
       ▼                              ▼
┌──────────────┐              ┌──────────────┐
│   build.rs   │              │  Cargo.toml  │
│   (Script)   │              │  (Config)    │
└──────┬───────┘              └──────────────┘
       │
       ├─────────────┬─────────────┐
       │             │             │
       ▼             ▼             ▼
   ┌──────┐     ┌────────┐    ┌────────┐
   │CMake │     │autocxx │    │ Link   │
   │Build │     │Binding │    │Config  │
   └───┬──┘     └───┬────┘    └───┬────┘
       │            │             │
       ▼            ▼             ▼
   ┌──────┐     ┌────────┐    ┌────────┐
   │ C++  │     │ Rust   │    │Linker  │
   │Bridge│ ──► │ Code   │ ──►│ Flags  │
   └──────┘     └────────┘    └───┬────┘
                                   │
                                   ▼
                              ┌─────────┐
                              │ BINARY  │
                              └─────────┘
```

## 📦 What Each Component Does

### 1. Cargo.toml
```toml
[dependencies]
autocxx = "0.27"     # C++ interop
thiserror = "1.0"    # Error handling

[build-dependencies]
autocxx-build = "0.27"
cmake = "0.1"        # For C++ compilation
```
**Job:** Tell Cargo what we need

---

### 2. build.rs
```rust
fn main() {
    // Step A: Build C++ bridge
    cmake::Config::new(".").build();

    // Step B: Generate Rust↔C++ bindings
    autocxx_build::Builder::new("src/lib.rs").build();

    // Step C: Tell linker about libraries
    println!("cargo:rustc-link-lib=dylib=pdfium");
}
```
**Job:** Coordinate 3 build systems (CMake, autocxx, Cargo)

---

### 3. CMakeLists.txt
```cmake
add_library(pdfium_bridge STATIC src/bridge.cpp)

include_directories(
    ${PDFIUM_ROOT}/public
    ${PDFIUM_ROOT}/third_party/Universal.Qpdf/include
)
```
**Job:** Compile C++ bridge code

---

### 4. src/bridge.cpp + src/bridge.h
```cpp
extern "C" {
    char* pdfium_bridge_extract_text(
        const uint8_t* pdf_data,
        size_t pdf_size
    ) {
        // Call PDFium APIs
        // Convert UTF-16 to UTF-8
        // Return malloc'd string
    }
}
```
**Job:** Wrap PDFium C++ API in simple C functions

---

### 5. src/lib.rs
```rust
autocxx::include_cpp! {
    #include "bridge.h"
    generate!("pdfium_bridge_extract_text")
}

pub fn extract_text(pdf_bytes: &[u8]) -> Result<String> {
    unsafe {
        let ptr = ffi::pdfium_bridge_extract_text(
            pdf_bytes.as_ptr(),
            pdf_bytes.len()
        );
        // Convert to Rust String
        // Free C string
    }
}
```
**Job:** Provide safe Rust API

---

## 🔄 Build Process Flow

```
Step 1: CMake Stage
┌─────────────────────────────────────────┐
│ Input:  src/bridge.cpp, CMakeLists.txt │
│ Tool:   CMake + C++ compiler            │
│ Output: libpdfium_bridge.a              │
└─────────────────────────────────────────┘
             │
             ▼
Step 2: autocxx Stage
┌─────────────────────────────────────────┐
│ Input:  src/lib.rs, src/bridge.h       │
│ Tool:   autocxx + clang parser          │
│ Output: Rust FFI code + glue library    │
└─────────────────────────────────────────┘
             │
             ▼
Step 3: Rust Compilation
┌─────────────────────────────────────────┐
│ Input:  src/lib.rs, src/error.rs       │
│ Tool:   rustc                           │
│ Output: auto_pqdfium_rs.rlib            │
└─────────────────────────────────────────┘
             │
             ▼
Step 4: Linking
┌─────────────────────────────────────────┐
│ Input:  All .rlib, .a, .so files       │
│ Tool:   linker (ld/lld)                 │
│ Output: Final binary                    │
└─────────────────────────────────────────┘
```

## 🗂️ File Transformations

### C++ Side
```
src/bridge.cpp ──[CMake]──► bridge.cpp.o ──► libpdfium_bridge.a
```

### Rust Side
```
src/lib.rs ──[autocxx]──► FFI bindings ──[rustc]──► .rlib
```

### Final Assembly
```
libpdfium_bridge.a  ┐
autocxx glue        ├──[linker]──► final binary
auto_pqdfium_rs.rlib│
libpdfium.so        │
libc++.so           ┘
```

## 📍 Key Locations

### During Build
```
target/debug/build/auto-pqdfium-rs-<hash>/
├── out/
│   ├── lib/libpdfium_bridge.a       ← CMake creates this
│   └── build/                        ← CMake work dir
└── output                            ← build.rs logs
```

### After Build
```
target/debug/
├── deps/
│   ├── libauto_pqdfium_rs.so        ← Your library
│   └── integration_test             ← Test executable
└── examples/
    └── basic_usage                   ← Example binary
```

### External (pre-built)
```
/path/to/pdfium-workspace/Universal.Pdfium/out/linux-x64-shared/
├── libpdfium.so                      ← PDFium library
└── libc++.so                         ← Chromium C++ stdlib
```

## ⚡ Runtime Linking

### What happens when you run the binary:

```
1. Binary starts
   │
   ├─► Loads libpdfium.so
   │       │
   │       └─► Loads libc++.so
   │               │
   │               └─► Loads system libs (pthread, dl, etc.)
   │
   └─► All symbols resolved
       │
       └─► Your code runs!
```

### Where does it find libraries?

```
1. rpath (set by build.rs)
   ├─► /path/to/pdfium/.../linux-x64-shared

2. LD_LIBRARY_PATH (if set)

3. System paths
   └─► /usr/lib, /lib, etc.
```

## 🎛️ Environment Variables

### User Can Set
```bash
# Use custom PDFium location
export PDFIUM_DIR=/my/custom/pdfium/path
cargo build

# Add library search paths at runtime
export LD_LIBRARY_PATH=/path/to/libs:$LD_LIBRARY_PATH
./target/debug/examples/basic_usage
```

### build.rs Sets (internally)
```bash
# Passed to rustc via println!
RUSTFLAGS += -L /path/to/libs
RUSTFLAGS += -l pdfium
RUSTFLAGS += -Wl,-rpath,/path/to/libs
```

## 🔍 Dependency Chain

### Compile-time (headers)
```
bridge.cpp needs:
├── fpdfview.h
├── fpdf_text.h
└── ipdf_qpdf.h
```

### Link-time (libraries)
```
final binary needs:
├── libpdfium_bridge.a (static)
├── libautocxx-*.a (static)
├── libpdfium.so (dynamic)
├── libc++.so (dynamic)
└── system libs (dynamic)
```

### Runtime (shared libraries)
```
./basic_usage needs at runtime:
├── libpdfium.so ──► must be findable
└── libc++.so    ──► must be findable
```

## 🐛 Common Issues & Solutions

| Error | Cause | Solution |
|-------|-------|----------|
| "undefined symbol: FPDF_*" | PDFium not linked | Check `cargo:rustc-link-lib=dylib=pdfium` |
| "libc++.so: not found" | Can't find at runtime | Check rpath or set LD_LIBRARY_PATH |
| "CMake Error" | CMake config issue | Check CMakeLists.txt syntax |
| "autocxx parse error" | Header parse fail | Check include paths, C++17 compatible |

## 📊 Size Breakdown (typical)

```
libpdfium_bridge.a      ~50 KB    (our C++ bridge)
libautocxx-*.a          ~100 KB   (autocxx glue)
auto_pqdfium_rs.rlib    ~200 KB   (Rust code)
libpdfium.so            ~8.7 MB   (PDFium + QPDF)
libc++.so               ~1.5 MB   (Chromium C++ lib)
─────────────────────────────────
Final binary            ~10 MB    (with all deps)
```

## ✅ Verification Commands

```bash
# Check build output
cargo build --verbose

# See what libraries are linked
ldd target/debug/examples/basic_usage

# Check rpath
readelf -d target/debug/examples/basic_usage | grep RPATH

# Run tests
cargo test

# Build docs
cargo doc --open
```

---

## 🎓 Key Takeaways

1. **build.rs is the conductor** - it orchestrates CMake and autocxx
2. **Three build systems work together** - CMake (C++), autocxx (bindings), Cargo (Rust)
3. **Static linking for bridge** - libpdfium_bridge.a is statically linked
4. **Dynamic linking for PDFium** - libpdfium.so and libc++.so are dynamically linked
5. **rpath ensures runtime finding** - No need to set LD_LIBRARY_PATH manually

The system is designed to be **automatic and reproducible** - just run `cargo build` and everything happens in the right order!
