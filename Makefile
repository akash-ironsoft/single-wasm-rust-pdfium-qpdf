# Makefile for auto-pqdfium-rs

.PHONY: help prepare-assets build-web serve clean

# Default target
help:
	@echo "PDFium WASM Build Targets:"
	@echo ""
	@echo "  make prepare-assets   Prepare assets from PDFium workspace"
	@echo "  make build-web        Build WASM module for web"
	@echo "  make serve            Start web server (builds if needed)"
	@echo "  make clean            Clean build artifacts"
	@echo ""

# Prepare assets from PDFium workspace
prepare-assets:
	@./prepare-assets.sh

# Build WASM for web
build-web:
	@./build-web.sh

# Serve web (rebuild if needed)
serve:
	@./serve-web.sh

# Serve with forced rebuild
serve-rebuild:
	@./serve-web.sh --rebuild

# Clean build artifacts
clean:
	cargo clean
	rm -f web/auto_pqdfium_rs.wasm
	rm -f web/auto_pqdfium_rs.js
	@echo "✓ Cleaned build artifacts"
