#!/usr/bin/env bash
set -e

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

PDFIUM_WORKSPACE="${1:-/home/akash/Dev/ironsoft/iron-universal/Universal.PdfEditor/pdfium-workspace/Universal.Pdfium}"
BUILD_TYPE="${2:-emscripten-wasm-release}"
PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ASSETS_DIR="${PROJECT_ROOT}/assets"

echo -e "${BLUE}PDFium Assets Preparation${NC}"
echo ""

if [ ! -d "${PDFIUM_WORKSPACE}" ]; then
    echo -e "${RED}❌ PDFium workspace not found: ${PDFIUM_WORKSPACE}${NC}"
    echo "Usage: $0 [PDFIUM_WORKSPACE] [BUILD_TYPE]"
    exit 1
fi

BUILD_DIR="${PDFIUM_WORKSPACE}/out/${BUILD_TYPE}"

if [ ! -d "${BUILD_DIR}" ]; then
    echo -e "${RED}❌ Build directory not found: ${BUILD_DIR}${NC}"
    exit 1
fi

echo -e "${GREEN}✓${NC} PDFium workspace: ${PDFIUM_WORKSPACE}"
echo -e "${GREEN}✓${NC} Build type: ${BUILD_TYPE}"
echo ""

mkdir -p "${ASSETS_DIR}"

echo -e "${BLUE}[1/2]${NC} Copying libpdfium.a..."
PDFIUM_LIB="${BUILD_DIR}/obj/libpdfium.a"
if [ ! -f "${PDFIUM_LIB}" ]; then
    echo -e "${RED}❌ libpdfium.a not found${NC}"
    exit 1
fi
cp "${PDFIUM_LIB}" "${ASSETS_DIR}/libpdfium.a"
echo -e "${GREEN}✓${NC} libpdfium.a: $(du -h "${ASSETS_DIR}/libpdfium.a" | cut -f1)"

echo -e "${BLUE}[2/2]${NC} Processing libqpdf.a..."
QPDF_DIR="${BUILD_DIR}/obj/third_party/Universal.Qpdf"
QPDF_LIB="${QPDF_DIR}/libqpdf.a"

if [ ! -f "${QPDF_LIB}" ]; then
    echo -e "${RED}❌ libqpdf.a not found${NC}"
    exit 1
fi

if file "${QPDF_LIB}" | grep -q "thin archive"; then
    echo -e "${YELLOW}⚠${NC}  Converting thin archive to full archive..."
    TEMP_ARCHIVE="/tmp/libqpdf_full_$$.a"
    cd "${QPDF_DIR}"
    ar -crs "${TEMP_ARCHIVE}" libqpdf/*.o
    cp "${TEMP_ARCHIVE}" "${ASSETS_DIR}/libqpdf.a"
    rm -f "${TEMP_ARCHIVE}"
    echo -e "${GREEN}✓${NC} Converted to full archive"
else
    cp "${QPDF_LIB}" "${ASSETS_DIR}/libqpdf.a"
fi
echo -e "${GREEN}✓${NC} libqpdf.a: $(du -h "${ASSETS_DIR}/libqpdf.a" | cut -f1)"

echo ""
echo -e "${GREEN}✅ Assets prepared successfully!${NC}"
echo -e "   Total size: $(du -sh "${ASSETS_DIR}" | cut -f1)"
echo ""
