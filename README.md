# auto-pqdfium-rs

## How to run:

```
  make prepare-assets   - Prepare assets from PDFium workspace
  make build-web        - Build WASM module for web
  make serve            - Start web server (builds if needed)
  make clean            - Clean build artifacts

```

## Simple Diagram
```
WASM (auto_pqdfium_rs.wasm)
  └── Rust (lib.rs)
        └── C Libraries (libpdfium.a, libqpdf.a)

```

## Asset Preparation (prepare-assets.sh)

1. Takes PDFium workspace path (default: `/home/akash/Dev/ironsoft/iron-universal/Universal.PdfEditor/pdfium-workspace/Universal.Pdfium`)
2. Copies `libpdfium.a` (19MB) from `out/emscripten-wasm-release/obj/`
3. Processes `libqpdf.a` (7MB) - converts thin archive to full archive if needed
4. Creates self-contained `assets/` directory with libraries (~26MB total)

## Build Process (build-web.sh)

1. Activates Emscripten SDK (`emsdk_env.sh`)
2. Compiles Rust to WASM: `cargo build --target wasm32-unknown-emscripten --release`
3. Links with `emcc`: Creates `auto_pqdfium_rs.js` (glue) + `auto_pqdfium_rs.wasm` (binary)
4. Exports functions: `pdfium_wasm_initialize`, `pdfium_wasm_extract_text`, `pdfium_wasm_pdf_to_json`
5. Output: 3.7MB WASM + 76KB JS in `web/` directory

