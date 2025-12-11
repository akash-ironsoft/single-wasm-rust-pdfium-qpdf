# PDFium WASM Browser Demo

This directory contains a browser-based demo of the auto-pqdfium-rs library compiled to WebAssembly.

## Files

- `auto_pqdfium_rs.wasm` - Compiled WASM module (3.7MB)
- `pdfium-wasm.js` - JavaScript wrapper for easy API usage
- `index.html` - Interactive demo page
- `server.py` - Simple HTTP server for testing
- `sample.pdf` - Test PDF file

## Quick Start

### 1. Start the server

```bash
cd web
python3 server.py
```

### 2. Open in browser

Navigate to: **http://localhost:8080**

### 3. Test the demo

- Click "Click to upload" or drag & drop a PDF file
- Click "Extract Text" to extract all text from the PDF
- Click "Convert to JSON" to get the PDF structure as JSON

## API Usage

### Initialize

```javascript
const pdfium = new PdfiumWasm();
await pdfium.init();
```

### Extract Text

```javascript
// Load PDF file
const file = document.getElementById('fileInput').files[0];
const arrayBuffer = await file.arrayBuffer();
const pdfBytes = new Uint8Array(arrayBuffer);

// Extract text
const text = await pdfium.extractText(pdfBytes);
console.log(text);
```

### Convert to JSON

```javascript
// Convert PDF structure to JSON
const json = await pdfium.pdfToJson(pdfBytes);
console.log(json);
```

### Cleanup

```javascript
pdfium.cleanup();
```

## Complete Example

```html
<!DOCTYPE html>
<html>
<head>
    <title>PDF Text Extractor</title>
</head>
<body>
    <input type="file" id="pdfFile" accept=".pdf">
    <button onclick="extractPDF()">Extract Text</button>
    <pre id="output"></pre>

    <script src="pdfium-wasm.js"></script>
    <script>
        let pdfium;

        // Initialize on page load
        window.onload = async () => {
            pdfium = new PdfiumWasm();
            await pdfium.init();
            console.log('PDFium ready!');
        };

        async function extractPDF() {
            const file = document.getElementById('pdfFile').files[0];
            if (!file) return;

            const arrayBuffer = await file.arrayBuffer();
            const pdfBytes = new Uint8Array(arrayBuffer);

            try {
                const text = await pdfium.extractText(pdfBytes);
                document.getElementById('output').textContent = text;
            } catch (error) {
                console.error('Error:', error);
            }
        }

        // Cleanup on page unload
        window.onbeforeunload = () => pdfium?.cleanup();
    </script>
</body>
</html>
```

## Features

- ✅ **Privacy-First**: All PDF processing happens in the browser - no server upload
- ✅ **Fast**: Native performance with WebAssembly
- ✅ **Offline**: Works without internet connection (after initial load)
- ✅ **Comprehensive**: Extract text and get full PDF structure as JSON
- ✅ **Cross-Platform**: Works in all modern browsers

## Browser Support

- Chrome/Edge 91+ ✅
- Firefox 89+ ✅
- Safari 15+ ✅
- Opera 77+ ✅

## Limitations

- **File Size**: Recommended max 50MB (browser memory limits)
- **Memory**: Large PDFs require more memory
- **Performance**: ~70-85% of native speed

## Building from Source

From the project root:

```bash
# Run the build script
./build-wasm.sh

# Or manually:
source /home/akash/Dev/emsdk/emsdk_env.sh
cargo build --target wasm32-unknown-emscripten --release
cp target/wasm32-unknown-emscripten/release/auto_pqdfium_rs.wasm web/
```

## Troubleshooting

### WASM not loading

- Make sure you're using an HTTP server (not `file://`)
- Check browser console for CORS errors
- Verify WASM file size is ~3.7MB

### Memory errors

- Try a smaller PDF file
- Close other browser tabs
- Increase browser memory limit

### Initialization fails

- Check that all files are served correctly
- Verify MIME types: `.wasm` should be `application/wasm`
- Check browser console for error messages

## Technical Details

### Architecture

```
┌─────────────────────────────────┐
│   Browser JavaScript            │
│   (pdfium-wasm.js)              │
└──────────┬──────────────────────┘
           │ WebAssembly Interface
           ▼
┌─────────────────────────────────┐
│   WASM Module                   │
│   (auto_pqdfium_rs.wasm)        │
│   ┌─────────────────────────┐   │
│   │ Rust Wrapper            │   │
│   └─────────┬───────────────┘   │
│             │                   │
│   ┌─────────▼───────────────┐   │
│   │ C++ Bridge              │   │
│   └─────────┬───────────────┘   │
│             │                   │
│   ┌─────────▼───────────────┐   │
│   │ PDFium Library          │   │
│   │ QPDF Library            │   │
│   └─────────────────────────┘   │
└─────────────────────────────────┘
```

### Exported Functions

- `pdfium_wasm_initialize()` - Initialize PDFium library
- `pdfium_wasm_extract_text(pdf_ptr, pdf_len)` - Extract text from PDF
- `pdfium_wasm_pdf_to_json(pdf_ptr, pdf_len)` - Convert PDF to JSON
- `pdfium_wasm_free_string(ptr)` - Free allocated strings
- `pdfium_wasm_cleanup()` - Cleanup resources

### Memory Management

The JavaScript wrapper handles memory allocation/deallocation:
1. Allocates WASM memory for PDF bytes
2. Calls WASM function
3. Reads result from WASM memory
4. Frees all allocated memory

## License

Same as parent project (MIT OR Apache-2.0)
