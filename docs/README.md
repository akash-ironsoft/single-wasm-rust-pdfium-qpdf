# Documentation Index

Complete documentation for auto-pqdfium-rs - A Rust WASM wrapper for PDFium + QPDF.

## 📚 Documentation

### **Architecture & Design**
- [**ARCHITECTURE.md**](./ARCHITECTURE.md) - Complete system architecture, call stack, and data flow
  - Project structure
  - 8-layer call stack (Browser → PDFium)
  - Memory management
  - Build process
  - Performance characteristics

### **Web Integration**
- [**Web README**](../web/README.md) - Browser usage guide
  - Quick start
  - API reference
  - Browser support
  - Deployment guide

### **Troubleshooting**
- [**TROUBLESHOOTING.md**](../web/TROUBLESHOOTING.md) - Common issues and solutions
  - Cache problems
  - MIME type errors
  - Memory errors
  - Debug tips

### **Main README**
- [**README.md**](../README.md) - Project overview and getting started

## 🎯 Quick Links

### For Developers
- [Architecture Overview](./ARCHITECTURE.md#-project-structure)
- [Call Stack Explanation](./ARCHITECTURE.md#-complete-text-extraction-call-stack)
- [Build Process](./ARCHITECTURE.md#-build-process)
- [Memory Management](./ARCHITECTURE.md#-memory-management)

### For Users
- [Getting Started](../README.md#getting-started)
- [Browser Demo](../web/README.md#quick-start)
- [API Usage](../web/README.md#api-usage)
- [Troubleshooting](../web/TROUBLESHOOTING.md)

## 📖 Document Structure

```
docs/
├── README.md           (This file - Documentation index)
└── ARCHITECTURE.md     (Technical architecture deep-dive)

web/
├── README.md           (Browser integration guide)
└── TROUBLESHOOTING.md  (Browser-specific issues)

.
├── README.md           (Main project README)
└── build-wasm.sh       (Build instructions)
```

## 🔍 What to Read First

### **New to the Project?**
1. Start with [main README.md](../README.md)
2. Run the example: `cargo run --example basic_usage`
3. Try the web demo: `cd web && python3 server.py`

### **Want to Build for Web?**
1. Read [Web README](../web/README.md)
2. Run `./build-wasm.sh`
3. Test at http://localhost:8080

### **Understanding the Architecture?**
1. Read [ARCHITECTURE.md](./ARCHITECTURE.md)
2. Follow the call stack diagrams
3. Study the memory management section

### **Having Issues?**
1. Check [TROUBLESHOOTING.md](../web/TROUBLESHOOTING.md)
2. Clear browser cache
3. Verify build outputs

## 🎓 Learning Path

### **Level 1: User**
- ✅ Run the examples
- ✅ Try the web demo
- ✅ Upload your own PDFs

### **Level 2: Integrator**
- ✅ Read Web README
- ✅ Study API reference
- ✅ Integrate into your app

### **Level 3: Contributor**
- ✅ Read ARCHITECTURE.md
- ✅ Understand the call stack
- ✅ Study memory management
- ✅ Review build process

## 🛠️ Development Workflow

### **Making Changes to Core Logic**
1. Modify Rust code in `src/`
2. Update FFI exports if needed
3. Rebuild: `cargo build --release`
4. Test native: `cargo run --example basic_usage`
5. Test WASM: `./build-wasm.sh && cd web && python3 server.py`

### **Making Changes to Web UI**
1. Modify `web/index.html` or `web/pdfium-wrapper.js`
2. Refresh browser (Ctrl+Shift+R)
3. Check DevTools console

### **Making Changes to C++ Bridge**
1. Modify `src/bridge.cpp`
2. Rebuild: `cargo clean && cargo build --release`
3. For WASM: `./build-wasm.sh`

## 📝 Documentation Standards

When updating documentation:
- ✅ Keep code examples up to date
- ✅ Include file paths and line numbers
- ✅ Add diagrams for complex flows
- ✅ Update "Last Updated" dates
- ✅ Cross-reference related docs

## 🤝 Contributing

Found an issue or want to improve docs?
1. Check existing documentation
2. Verify the issue/improvement
3. Update relevant files
4. Test your changes
5. Submit a PR

## 📞 Getting Help

- **Issues:** Check TROUBLESHOOTING.md first
- **Architecture Questions:** See ARCHITECTURE.md
- **Web Integration:** See web/README.md
- **General:** See main README.md

---

**Documentation Version:** 1.0.0
**Last Updated:** December 11, 2025
**Project Version:** 0.1.0
