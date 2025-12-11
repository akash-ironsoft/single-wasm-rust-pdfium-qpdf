# Assets Directory

Pre-compiled libraries for WASM builds.

## Contents

```
assets/
├── libpdfium.a           (19 MB) - PDFium library (Emscripten-compiled)
├── libqpdf.a             (7 MB)  - QPDF library (Emscripten-compiled)
└── include/              - Header files
    ├── *.h               - PDFium headers
    └── qpdf/             - QPDF headers
        └── *.h
```

## Library Details

### libpdfium.a
- **Size:** 19 MB
- **Source:** Google PDFium
- **Compiled for:** wasm32-unknown-emscripten
- **Version:** Chromium branch
- **Features:** PDF rendering, text extraction, page manipulation

### libqpdf.a
- **Size:** 7 MB
- **Source:** QPDF
- **Compiled for:** wasm32-unknown-emscripten
- **Features:** PDF to JSON conversion

## Building

These libraries are automatically linked during the build process:

```bash
cargo build --target wasm32-unknown-emscripten --release
```

The `build.rs` script:
1. Locates these libraries in `assets/`
2. Links them statically into the Rust code
3. Includes the headers from `assets/include/`

## Updating Libraries

If you need to update these libraries:

1. **Rebuild PDFium for Emscripten:**
   ```bash
   # In PDFium source directory
   gn args out/emscripten-wasm-release
   # Set: target_os = "emscripten", is_debug = false
   ninja -C out/emscripten-wasm-release pdfium
   ```

2. **Copy new libraries:**
   ```bash
   cp path/to/pdfium/out/emscripten-wasm-release/obj/libpdfium.a assets/

   # Note: libqpdf.a is a thin archive, convert to full archive
   cd path/to/pdfium/out/emscripten-wasm-release/obj/third_party/Universal.Qpdf
   ar -crs libqpdf_full.a libqpdf/*.o
   cp libqpdf_full.a /path/to/assets/libqpdf.a
   ```

3. **Update headers if needed:**
   ```bash
   cp -r path/to/pdfium/public/*.h assets/include/
   ```

## Why Local Assets?

**Advantages:**
- ✅ Self-contained project
- ✅ No external dependencies
- ✅ Reproducible builds
- ✅ Version control friendly
- ✅ Easier to distribute

**Trade-offs:**
- Repository size: +26 MB (19 MB PDFium + 7 MB QPDF)
- Git LFS recommended for large files

## Git LFS (Optional)

For better Git performance with large binary files:

```bash
# Install Git LFS
git lfs install

# Track .a files
git lfs track "assets/*.a"
git add .gitattributes

# Commit
git add assets/
git commit -m "Add pre-compiled libraries"
```

## License

These are pre-compiled binaries from:
- **PDFium:** BSD License (Google)
- **QPDF:** Apache 2.0 License

See the respective project licenses for details.

## Support

If you encounter linking errors:
1. Verify files exist: `ls -lh assets/`
2. Check target: Must be `wasm32-unknown-emscripten`
3. Rebuild: `cargo clean && cargo build --target wasm32-unknown-emscripten --release`
