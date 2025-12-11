#!/bin/bash
#
# Build script for WASM target
#

set -e

echo "🔨 Building auto-pqdfium-rs for WebAssembly..."
echo ""

# Activate Emscripten SDK
if [ -f "/home/akash/Dev/emsdk/emsdk_env.sh" ]; then
    echo "📦 Activating Emscripten SDK..."
    source /home/akash/Dev/emsdk/emsdk_env.sh 2>/dev/null
else
    echo "❌ Emscripten SDK not found at /home/akash/Dev/emsdk"
    echo "Please install emsdk first: https://emscripten.org/docs/getting_started/downloads.html"
    exit 1
fi

# Build with cargo
echo "🦀 Compiling Rust to WASM..."
cargo build --target wasm32-unknown-emscripten --release

# Generate Emscripten JS glue code
echo "🔗 Generating Emscripten JS glue code..."
mkdir -p web
emcc target/wasm32-unknown-emscripten/release/libauto_pqdfium_rs.a \
    -o web/auto_pqdfium_rs.js \
    -sERROR_ON_UNDEFINED_SYMBOLS=0 \
    -sALLOW_MEMORY_GROWTH=1 \
    -sEXPORTED_FUNCTIONS=_pdfium_wasm_initialize,_pdfium_wasm_extract_text,_pdfium_wasm_pdf_to_json,_pdfium_wasm_free_string,_pdfium_wasm_cleanup,_malloc,_free \
    -sEXPORTED_RUNTIME_METHODS=ccall,cwrap,UTF8ToString,stringToUTF8,lengthBytesUTF8,getValue,setValue,writeArrayToMemory \
    -sINITIAL_MEMORY=67108864 \
    -sMODULARIZE=1 \
    -sEXPORT_NAME=createPdfiumModule \
    -sENVIRONMENT=web \
    -O3

# Get file sizes
WASM_SIZE=$(ls -lh web/auto_pqdfium_rs.wasm | awk '{print $5}')
JS_SIZE=$(ls -lh web/auto_pqdfium_rs.js | awk '{print $5}')

echo ""
echo "✅ Build complete!"
echo "📦 WASM file: web/auto_pqdfium_rs.wasm (${WASM_SIZE})"
echo "📦 JS glue:   web/auto_pqdfium_rs.js (${JS_SIZE})"
echo ""
echo "🚀 To test in browser:"
echo "   cd web"
echo "   python3 server.py"
echo "   Open http://localhost:8080"
echo ""
