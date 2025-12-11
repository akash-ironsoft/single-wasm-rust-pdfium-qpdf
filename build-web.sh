#!/bin/bash
#
# Minimal Web Build Script
# Builds WASM module and prepares web directory
#

set -e  # Exit on error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}================================${NC}"
echo -e "${BLUE}  PDFium WASM Web Builder${NC}"
echo -e "${BLUE}================================${NC}"
echo ""

# Check if Emscripten is available
if ! command -v emcc &> /dev/null; then
    echo -e "${YELLOW}📦 Activating Emscripten SDK...${NC}"
    if [ -f "$HOME/Dev/emsdk/emsdk_env.sh" ]; then
        source "$HOME/Dev/emsdk/emsdk_env.sh" 2>/dev/null
    else
        echo -e "${RED}❌ Emscripten not found at $HOME/Dev/emsdk${NC}"
        echo "Install: https://emscripten.org/docs/getting_started/downloads.html"
        exit 1
    fi
fi

# Verify emcc is now available
if ! command -v emcc &> /dev/null; then
    echo -e "${RED}❌ Failed to activate Emscripten${NC}"
    exit 1
fi

echo -e "${GREEN}✓${NC} Emscripten SDK ready ($(emcc --version | head -1))"
echo ""

# Step 1: Build Rust → WASM
echo -e "${BLUE}[1/4]${NC} Building Rust to WASM..."
cargo build --target wasm32-unknown-emscripten --release --quiet

if [ $? -eq 0 ]; then
    echo -e "${GREEN}✓${NC} Rust compilation complete"
else
    echo -e "${RED}❌ Rust compilation failed${NC}"
    exit 1
fi

# Step 2: Generate Emscripten JS glue
echo -e "${BLUE}[2/4]${NC} Generating JavaScript glue code..."
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
    -O3 2>&1 | grep -v "warning:" || true

if [ $? -eq 0 ]; then
    echo -e "${GREEN}✓${NC} Emscripten linking complete"
else
    echo -e "${RED}❌ Emscripten linking failed${NC}"
    exit 1
fi

# Step 3: Verify output files
echo -e "${BLUE}[3/4]${NC} Verifying output files..."

if [ ! -f "web/auto_pqdfium_rs.wasm" ]; then
    echo -e "${RED}❌ WASM file not found${NC}"
    exit 1
fi

if [ ! -f "web/auto_pqdfium_rs.js" ]; then
    echo -e "${RED}❌ JS glue file not found${NC}"
    exit 1
fi

# Get file sizes
WASM_SIZE=$(du -h web/auto_pqdfium_rs.wasm | cut -f1)
JS_SIZE=$(du -h web/auto_pqdfium_rs.js | cut -f1)

echo -e "${GREEN}✓${NC} WASM module: ${WASM_SIZE}"
echo -e "${GREEN}✓${NC} JS glue:     ${JS_SIZE}"

# Step 4: Check required files
echo -e "${BLUE}[4/4]${NC} Checking web directory..."

REQUIRED_FILES=(
    "web/index.html"
    "web/pdfium-wrapper.js"
    "web/auto_pqdfium_rs.wasm"
    "web/auto_pqdfium_rs.js"
)

MISSING=0
for file in "${REQUIRED_FILES[@]}"; do
    if [ ! -f "$file" ]; then
        echo -e "${RED}✗${NC} Missing: $file"
        MISSING=1
    fi
done

if [ $MISSING -eq 1 ]; then
    echo -e "${RED}❌ Some required files are missing${NC}"
    exit 1
fi

echo -e "${GREEN}✓${NC} All required files present"
echo ""

# Summary
echo -e "${GREEN}================================${NC}"
echo -e "${GREEN}  ✅ Build Complete!${NC}"
echo -e "${GREEN}================================${NC}"
echo ""
echo -e "📦 Output:"
echo -e "   └─ web/"
echo -e "      ├─ auto_pqdfium_rs.wasm  (${WASM_SIZE})"
echo -e "      ├─ auto_pqdfium_rs.js    (${JS_SIZE})"
echo -e "      └─ [ready to serve]"
echo ""
echo ""
