# PDFium Linking Guide

Complete guide to where and how libpdfium is linked in both native and WASM builds.

## 📍 Where is libpdfium Linked?

### **TL;DR:**
- **Native Build:** Cargo links `libpdfium.so` (shared/dynamic library)
- **WASM Build:** Cargo links `libpdfium.a` (static library) into Rust archive, then `emcc` links everything into final WASM

---

## 🔄 Native Build (Linux/macOS)

### **Step 1: Cargo Build (build.rs:61-81)**

```rust
// build.rs - Native build configuration
fn build_for_native(pdfium_dir: &str) {
    // PDFium library location
    let pdfium_lib_dir = format!("{}/out/linux-x64-shared", pdfium_dir);

    // Tell rustc where to find libraries
    println!("cargo:rustc-link-search=native={}", pdfium_lib_dir);

    // Link PDFium as dynamic library
    println!("cargo:rustc-link-lib=dylib=pdfium");

    // Also link C++ standard library
    println!("cargo:rustc-link-lib=dylib=c++");

    // Set RPATH for runtime library discovery
    println!("cargo:rustc-link-arg=-Wl,-rpath,{}", pdfium_lib_dir);
}
```

**File Locations:**
```
PDFium Directory:
/home/akash/Dev/ironsoft/iron-universal/Universal.PdfEditor/pdfium-workspace/Universal.Pdfium/
└── out/linux-x64-shared/
    └── libpdfium.so  (8.7 MB - Shared library)
```

**Linking Method:** **Dynamic Linking**
- ✅ Rust binary links to `libpdfium.so` at runtime
- ✅ Uses RPATH to find library
- ✅ Smaller binary size
- ❌ Requires library file at runtime

**Verify:**
```bash
# Check what libraries the binary needs
ldd target/release/examples/basic_usage | grep pdfium
# Output: libpdfium.so => /path/to/libpdfium.so

# Check RPATH
readelf -d target/release/examples/basic_usage | grep RPATH
# Output: (RPATH) Library rpath: [/path/to/pdfium/out/linux-x64-shared]
```

---

## 🌐 WASM Build (Emscripten)

### **Step 1: Cargo Build (build.rs:36-59)**

```rust
// build.rs - WASM build configuration
fn build_for_wasm(pdfium_dir: &str) {
    // PDFium WASM library location
    let wasm_lib_dir = format!("{}/out/emscripten-wasm-release/obj", pdfium_dir);

    // Tell rustc where to find libraries
    println!("cargo:rustc-link-search=native={}", wasm_lib_dir);

    // Link PDFium as static library
    println!("cargo:rustc-link-lib=static=pdfium");

    // Also link QPDF
    println!("cargo:rustc-link-lib=static=qpdf");

    // Compile bridge.cpp
    cc::Build::new()
        .cpp(true)
        .file("src/bridge.cpp")
        .compile("pdfium_bridge");
}
```

**File Locations:**
```
PDFium Directory:
/home/akash/Dev/ironsoft/iron-universal/Universal.PdfEditor/pdfium-workspace/Universal.Pdfium/
└── out/emscripten-wasm-release/obj/
    ├── libpdfium.a  (19 MB - Static library)
    └── third_party/Universal.Qpdf/
        └── libqpdf.a
```

**What Cargo Produces:**
```bash
cargo build --target wasm32-unknown-emscripten --release

# Output:
target/wasm32-unknown-emscripten/release/
└── libauto_pqdfium_rs.a  (38 MB)
    ├── Rust code (compiled to WASM bytecode)
    ├── bridge.cpp (compiled with emcc)
    ├── libpdfium.a (19 MB - PDFium static lib)
    └── libqpdf.a (QPDF static lib)
```

**Linking Method:** **Static Linking Phase 1**
- ✅ PDFium code is included in Rust `.a` archive
- ✅ Everything bundled together
- ⚠️ Not yet a complete WASM module

### **Step 2: Emscripten Final Linking (build-web.sh)**

```bash
emcc target/wasm32-unknown-emscripten/release/libauto_pqdfium_rs.a \
    -o web/auto_pqdfium_rs.js \
    -sEXPORTED_FUNCTIONS=... \
    -sEXPORTED_RUNTIME_METHODS=... \
    -O3
```

**What emcc Does:**
1. **Extracts** all object code from `libauto_pqdfium_rs.a`
2. **Links** everything into a single WASM module
3. **Generates** JavaScript glue code
4. **Exports** specified functions
5. **Optimizes** the final binary

**Output:**
```
web/
├── auto_pqdfium_rs.wasm  (3.7 MB - Complete WASM module)
│   ├── Rust code
│   ├── bridge.cpp code
│   ├── PDFium code (from libpdfium.a)
│   └── QPDF code (from libqpdf.a)
└── auto_pqdfium_rs.js    (76 KB - Emscripten runtime)
```

**Linking Method:** **Static Linking Phase 2 (Final)**
- ✅ All code in single `.wasm` file
- ✅ Self-contained module
- ✅ No external dependencies
- ❌ Larger file size

---

## 🔍 Detailed Linking Flow

### **Native Build Flow:**
```
┌─────────────────────────────────────────────────────┐
│ Step 1: Compile C++ Bridge                          │
│ CMake: bridge.cpp → libpdfium_bridge.a              │
└──────────────────┬──────────────────────────────────┘
                   │
                   ▼
┌─────────────────────────────────────────────────────┐
│ Step 2: Compile Rust Code                           │
│ rustc: src/lib.rs → Rust object files               │
└──────────────────┬──────────────────────────────────┘
                   │
                   ▼
┌─────────────────────────────────────────────────────┐
│ Step 3: Link Everything                             │
│ rustc/ld:                                            │
│   ├─ Rust objects                                   │
│   ├─ libpdfium_bridge.a (static)                    │
│   ├─ libpdfium.so (dynamic) ◄── References only    │
│   ├─ libc++.so (dynamic)                            │
│   └─ System libs (pthread, dl, m)                   │
└──────────────────┬──────────────────────────────────┘
                   │
                   ▼
┌─────────────────────────────────────────────────────┐
│ Output: Binary with dynamic dependencies            │
│ - Contains: Rust + bridge code                      │
│ - Links to: libpdfium.so (loaded at runtime)        │
└─────────────────────────────────────────────────────┘
```

### **WASM Build Flow:**
```
┌─────────────────────────────────────────────────────┐
│ Step 1: Compile C++ Bridge                          │
│ emcc: bridge.cpp → object files                     │
└──────────────────┬──────────────────────────────────┘
                   │
                   ▼
┌─────────────────────────────────────────────────────┐
│ Step 2: Compile Rust Code                           │
│ rustc: src/lib.rs → WASM bytecode                   │
└──────────────────┬──────────────────────────────────┘
                   │
                   ▼
┌─────────────────────────────────────────────────────┐
│ Step 3: Cargo Links Static Libraries                │
│ rustc/emcc:                                          │
│   ├─ Rust WASM objects                              │
│   ├─ bridge.o objects                               │
│   ├─ libpdfium.a (19 MB) ◄── Statically linked     │
│   └─ libqpdf.a ◄── Statically linked               │
│                                                      │
│ Output: libauto_pqdfium_rs.a (38 MB archive)        │
└──────────────────┬──────────────────────────────────┘
                   │
                   ▼
┌─────────────────────────────────────────────────────┐
│ Step 4: Final Link with Emscripten                  │
│ emcc libauto_pqdfium_rs.a:                          │
│   ├─ Extracts all objects                           │
│   ├─ Links into single WASM module                  │
│   ├─ Generates JS glue code                         │
│   ├─ Exports functions                              │
│   └─ Applies optimizations (-O3)                    │
└──────────────────┬──────────────────────────────────┘
                   │
                   ▼
┌─────────────────────────────────────────────────────┐
│ Output: Self-contained WASM module                  │
│ - auto_pqdfium_rs.wasm (3.7 MB)                     │
│   └─ Contains: Rust + bridge + PDFium + QPDF       │
│ - auto_pqdfium_rs.js (76 KB - Runtime)             │
└─────────────────────────────────────────────────────┘
```

---

## 📊 Library Size Comparison

| Component | Native | WASM |
|-----------|--------|------|
| **PDFium Library** | 8.7 MB (shared) | 19 MB (static) |
| **QPDF Library** | Included in PDFium | Separate |
| **Rust Code** | ~500 KB | ~500 KB |
| **Bridge Code** | ~50 KB | ~50 KB |
| **Final Size** | 600 KB exe + 8.7 MB .so | 3.7 MB .wasm |
| **Runtime Deps** | libpdfium.so, libc++.so | None |

**Why WASM is smaller:** Emscripten applies aggressive optimization and dead code elimination.

---

## 🔧 Customizing PDFium Location

### **Environment Variable:**
```bash
export PDFIUM_DIR="/path/to/your/pdfium"
cargo build --release
```

### **Check Current Location:**
```bash
# From build.rs:
echo $PDFIUM_DIR

# Default:
# /home/akash/Dev/ironsoft/iron-universal/Universal.PdfEditor/pdfium-workspace/Universal.Pdfium
```

---

## 🐛 Troubleshooting Linking Issues

### **Native Build: Library Not Found**
```bash
# Error: cannot find -lpdfium
# Solution: Check PDFium library exists
ls $PDFIUM_DIR/out/linux-x64-shared/libpdfium.so

# Or set PDFIUM_DIR:
export PDFIUM_DIR="/correct/path/to/pdfium"
```

### **WASM Build: Library Not Found**
```bash
# Error: cannot find -lpdfium
# Solution: Check WASM library exists
ls $PDFIUM_DIR/out/emscripten-wasm-release/obj/libpdfium.a

# Rebuild PDFium for WASM if needed
```

### **Runtime: Shared Library Not Found**
```bash
# Error: error while loading shared libraries: libpdfium.so
# Solution: Add to LD_LIBRARY_PATH
export LD_LIBRARY_PATH=$PDFIUM_DIR/out/linux-x64-shared:$LD_LIBRARY_PATH

# Or use the built-in RPATH (should work automatically)
```

### **WASM: Undefined Symbols**
```bash
# Error: undefined symbol: FPDF_LoadDocument
# Solution: Ensure PDFium was linked in cargo build
cargo clean
cargo build --target wasm32-unknown-emscripten --release

# Then link with emcc
./build-web.sh
```

---

## 📝 Summary

### **Key Points:**

1. **Native builds use dynamic linking**
   - Smaller binary
   - Requires .so files at runtime
   - Uses RPATH for discovery

2. **WASM builds use static linking**
   - Self-contained module
   - Larger but portable
   - No runtime dependencies

3. **Two-stage WASM linking**
   - Stage 1: Cargo links static libs into .a
   - Stage 2: emcc creates final .wasm

4. **PDFium is always linked**
   - Native: As shared library (.so)
   - WASM: As static library (.a)

### **Commands to Verify:**

```bash
# Native: Check linked libraries
ldd target/release/examples/basic_usage

# WASM: Check .a contents
ar t target/wasm32-unknown-emscripten/release/libauto_pqdfium_rs.a | head -20

# WASM: Check final WASM exports
wasm-objdump -x web/auto_pqdfium_rs.wasm | grep -A 5 "Export"
```

---

**See Also:**
- [ARCHITECTURE.md](./ARCHITECTURE.md) - Complete system architecture
- [build.rs](../build.rs) - Actual linking configuration
- [build-web.sh](../build-web.sh) - WASM build script
