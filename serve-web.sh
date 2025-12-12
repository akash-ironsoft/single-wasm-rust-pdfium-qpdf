#!/bin/bash
#
# Quick Web Server Launch
# Optionally rebuilds before serving
#

set -e

# Parse arguments
REBUILD=false
if [ "$1" == "--rebuild" ] || [ "$1" == "-r" ]; then
    REBUILD=true
fi

echo "PDFium WASM Web Server"
echo ""

# Rebuild if requested
if [ "$REBUILD" = true ]; then
    echo "Rebuilding WASM module..."
    ./build-web.sh
    echo ""
fi

# Check if files exist
if [ ! -f "web/auto_pqdfium_rs.wasm" ]; then
    echo "WASM module not found. Building..."
    ./build-web.sh
    echo ""
fi

# Start server
echo "🌐 Starting server at http://localhost:8080"
echo "Serving: $(pwd)/web"
echo ""

cd web && python -m http.server
