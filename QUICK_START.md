# Quick Start Guide

Get up and running with PDFium WASM in 3 commands.

## 🚀 For Web (WASM)

### Option 1: One Command
```bash
make serve
```
Opens http://localhost:8080 with auto-rebuild if needed.

### Option 2: Build Then Serve
```bash
# Build
./build-web.sh

# Serve
cd web && python3 server.py
```

### Option 3: Force Rebuild
```bash
make serve-rebuild
# or
./serve-web.sh --rebuild
```

## 🖥️ For Native (Development)

### Run Example
```bash
make example
# or
cargo run --example basic_usage --release
```

### Run Tests
```bash
make test
# or
cargo test --release
```

## 📦 Build Targets

| Command | What It Does |
|---------|-------------|
| `make build-web` | Build WASM module only |
| `make serve` | Build (if needed) + serve |
| `make clean` | Clean build artifacts |
| `make example` | Run native example |
| `make test` | Run all tests |

## 🔧 Manual Build Steps

### Native Build
```bash
cargo build --release
```

### WASM Build (Manual)
```bash
# 1. Activate Emscripten
source ~/Dev/emsdk/emsdk_env.sh

# 2. Build Rust
cargo build --target wasm32-unknown-emscripten --release

# 3. Generate JS glue
emcc target/wasm32-unknown-emscripten/release/libauto_pqdfium_rs.a \
    -o web/auto_pqdfium_rs.js \
    [... emscripten flags ...]
```

## 📂 Output Files

After building for web:
```
web/
├── auto_pqdfium_rs.wasm  (3.7MB - WASM module)
├── auto_pqdfium_rs.js    (76KB - Emscripten glue)
├── pdfium-wrapper.js     (API wrapper)
└── index.html            (Demo UI)
```

## 🎯 Common Workflows

### First Time Setup
```bash
# Install Rust target
rustup target add wasm32-unknown-emscripten

# Install Emscripten
git clone https://github.com/emscripten-core/emsdk.git ~/Dev/emsdk
cd ~/Dev/emsdk
./emsdk install latest
./emsdk activate latest

# Build and serve
cd /path/to/auto-pqdfium-rs
make serve
```

### Development Cycle
```bash
# Edit code...

# Test natively (fast)
make example

# Build for web (slower)
make build-web

# Test in browser
make serve
```

### Clean Rebuild
```bash
make clean
make build-web
```

## 🐛 Troubleshooting

### Build Fails
```bash
# Check Emscripten
source ~/Dev/emsdk/emsdk_env.sh
emcc --version

# Clean and retry
make clean
make build-web
```

### Server Port Busy
```bash
# Kill existing server
pkill -f "python3 server.py"

# Try again
make serve
```

### Browser Cache Issues
1. Hard refresh: `Ctrl+Shift+R`
2. Clear cache: `Ctrl+Shift+Delete`
3. Or use incognito: `Ctrl+Shift+N`

## 📖 More Documentation

- [Architecture](docs/ARCHITECTURE.md) - Technical deep-dive
- [Web Integration](web/README.md) - Browser API guide
- [Troubleshooting](web/TROUBLESHOOTING.md) - Common issues

## ⚡ Pro Tips

1. **Use `make serve`** - Handles everything automatically
2. **Test native first** - Much faster than WASM builds
3. **Use incognito mode** - Avoids cache issues during development
4. **Check DevTools** - Console shows initialization status

---

**Build Time:** ~20 seconds for WASM, ~2 seconds for native
**First Run:** May take longer (downloading dependencies)
