# Debugging Guide: Rust → C++ (PDFium/QPDF) in VSCode

This guide explains how to debug across the Rust/C++ boundary in this project.

## Prerequisites

### 1. Install VSCode Extensions

Install these required extensions:
- **rust-analyzer** (rust-lang.rust-analyzer)
- **CodeLLDB** (vadimcn.vscode-lldb) - for debugging

```bash
code --install-extension rust-lang.rust-analyzer
code --install-extension vadimcn.vscode-lldb
```

### 2. Install Debugging Tools

```bash
# Install LLDB debugger
sudo apt-get install lldb

# Verify installation
lldb --version
```

## Debugging Challenge with WASM Target

Your project targets `wasm32-unknown-emscripten`, which makes native debugging difficult. You have two options:

### Option A: Enable Native Build for Debugging (Recommended)

Temporarily modify `build.rs` to allow native builds during development:

```rust
// In build.rs, comment out the panic:
let target = std::env::var("TARGET").unwrap();
if target != "wasm32-unknown-emscripten" {
    // panic!("This project only supports..."); // Comment this out
    println!("cargo:warning=Building for native target (debugging mode)");
}
```

Then you'll need native versions of libpdfium.a and libqpdf.a for your platform.

### Option B: Debug WASM with Browser DevTools

1. Build with debug info:
```bash
RUSTFLAGS="-g" ./build-web.sh
```

2. Use Chrome/Firefox DevTools:
   - Chrome: Enable WASM debugging in DevTools experiments
   - Firefox: Built-in WASM debugging support
   - Set breakpoints in Sources tab
   - Inspect WASM memory and call stack

## Debugging Workflow

### Step 1: Build with Debug Symbols

```bash
# For WASM (with debug symbols)
RUSTFLAGS="-g" cargo build --target wasm32-unknown-emscripten

# For native (if you modified build.rs)
RUSTFLAGS="-g" cargo build
```

### Step 2: Set Breakpoints

In VSCode:
1. Open your Rust source file (e.g., `src/lib.rs`)
2. Click in the gutter to set breakpoints at:
   - `extract_text()` function (line 100)
   - FFI calls like `ffi::FPDF_LoadMemDocument()` (line 110)
   - `pdf_to_json()` function (line 207)

### Step 3: Start Debugging

#### For Test Functions:
1. Add a test to your code:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_text() {
        let pdf_data = include_bytes!("../test.pdf");
        let result = extract_text(pdf_data);
        assert!(result.is_ok());
    }
}
```

2. Press F5 or click "Run and Debug" → "Debug Rust Tests"

#### For Binary/Example:
1. Convert your library to have a binary target in Cargo.toml:
```toml
[[bin]]
name = "test-debug"
path = "src/main.rs"
```

2. Create `src/main.rs`:
```rust
use auto_pqdfium_rs::{initialize, extract_text, pdf_to_json};

fn main() {
    initialize().unwrap();

    let pdf_data = std::fs::read("test.pdf").unwrap();

    // Set breakpoint here
    let text = extract_text(&pdf_data).unwrap();
    println!("Extracted text: {}", text);

    // Set breakpoint here
    let json = pdf_to_json(&pdf_data).unwrap();
    println!("JSON: {}", json);
}
```

3. Press F5 or click "Debug Example/Binary"

## Debugging into C++ Code

To step into PDFium/QPDF C++ code, you need:

1. **Source code** for the libraries:
   ```bash
   # Clone PDFium
   git clone https://pdfium.googlesource.com/pdfium

   # Clone QPDF
   git clone https://github.com/qpdf/qpdf
   ```

2. **Debug symbols**: Rebuild libraries with `-g` flag:
   ```bash
   # For PDFium
   cd pdfium
   # Build with debug symbols...

   # For QPDF
   cd qpdf
   cmake -DCMAKE_BUILD_TYPE=Debug ...
   make
   ```

3. **Configure source paths** in `.vscode/launch.json`:
   ```json
   {
       "sourceMap": {
           "/path/in/library": "${workspaceFolder}/../pdfium/src",
           "/qpdf/source": "${workspaceFolder}/../qpdf/libqpdf"
       }
   }
   ```

## Debugging Tips

### View C++ Call Stack
When stopped at a Rust breakpoint that's about to call C++:
1. Open Debug Console
2. Type: `bt` (backtrace) to see full stack
3. Type: `frame select N` to switch to a C++ frame

### Inspect C++ Variables
```lldb
# In Debug Console:
p *document          # Dereference C++ pointer
p document->field    # Access C++ struct field
```

### Step Through FFI Boundary
- **Step Into** (F11): Steps into C++ code if debug symbols available
- **Step Over** (F10): Executes C++ function and returns to Rust

### Watch Memory
```lldb
memory read 0x7fff...  # Read raw memory
x/10x $rsp            # Examine stack
```

## Common Issues

### Issue: "Cannot find debug symbols"
**Solution**: Rebuild PDFium/QPDF with debug symbols enabled

### Issue: "Source file not found"
**Solution**: Configure `sourceMap` in launch.json to point to C++ source locations

### Issue: "Cannot step into C++ code"
**Solution**:
1. Verify LLDB can find the source: `(lldb) source list`
2. Add source paths: `(lldb) settings set target.source-map /old/path /new/path`

### Issue: WASM debugging is too difficult
**Solution**:
1. Create native builds for debugging
2. Use printf/logging debugging in C++ code
3. Use browser DevTools for WASM-specific debugging

## Alternative: Printf Debugging

If source-level debugging is too complex:

1. Add logging to your C++ libraries:
```cpp
// In PDFium/QPDF source
fprintf(stderr, "DEBUG: Loading page %d\n", page_index);
```

2. Rebuild libraries with logging

3. Run and view output:
```bash
RUST_LOG=debug cargo run 2>&1 | grep DEBUG
```

## Resources

- [LLDB Tutorial](https://lldb.llvm.org/use/tutorial.html)
- [VSCode LLDB Extension](https://github.com/vadimcn/vscode-lldb/blob/master/MANUAL.md)
- [Debugging WASM](https://developer.chrome.com/blog/wasm-debugging-2020/)
- [PDFium Documentation](https://pdfium.googlesource.com/pdfium/)
- [QPDF Documentation](https://qpdf.readthedocs.io/)
