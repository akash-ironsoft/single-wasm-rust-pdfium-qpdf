# Troubleshooting Guide

## ✅ All Issues Fixed!

The following issues have been resolved:
1. ✅ MIME type now correct (`application/wasm`)
2. ✅ Memory access fixed (using `writeArrayToMemory` or `HEAP8`)
3. ✅ Cache-busting headers added

## 🔄 Clear Your Browser Cache

**IMPORTANT:** You must clear your browser cache to see the fixes!

### Chrome/Edge:
1. Press `Ctrl+Shift+Delete` (or `Cmd+Shift+Delete` on Mac)
2. Select "Cached images and files"
3. Click "Clear data"

**OR** Hard refresh: `Ctrl+Shift+R` (or `Cmd+Shift+R` on Mac)

### Firefox:
1. Press `Ctrl+Shift+Delete` (or `Cmd+Shift+Delete` on Mac)
2. Select "Cache"
3. Click "Clear"

**OR** Hard refresh: `Ctrl+F5`

### Safari:
1. Go to Develop → Empty Caches
2. Or press `Option+Cmd+E`

## 🧪 Test After Clearing Cache

1. **Hard refresh** the page: `Ctrl+Shift+R`
2. Open browser DevTools (F12)
3. Go to Network tab
4. Refresh page and look for `auto_pqdfium_rs.wasm`
5. Verify response headers:
   ```
   Content-Type: application/wasm
   Cache-Control: no-store, no-cache, must-revalidate
   ```

## 🎯 Expected Behavior

After clearing cache, you should see:
- ✅ No MIME type warnings
- ✅ `PDFium WASM initialized successfully`
- ✅ Text extraction works
- ✅ JSON conversion works

## 🐛 Still Having Issues?

### Issue: MIME Type Warning Still Appears

**Solution:** Your browser has aggressively cached the old WASM file.

Try:
```bash
# 1. Close ALL browser windows
# 2. Restart the server
cd web
pkill -9 -f server.py
python3 server.py

# 3. Open browser in incognito/private mode
# 4. Visit http://localhost:8080
```

### Issue: Memory Access Errors

**Error:** `Cannot read properties of undefined`

**Solution:** The issue is now fixed in `pdfium-wrapper.js`. Make sure you've cleared cache!

The fix uses two fallback methods:
1. `Module.writeArrayToMemory()` (if available)
2. Direct `HEAP8` access (fallback)

### Issue: Functions Not Exported

**Error:** `Module._pdfium_wasm_initialize is not a function`

**Check:**
```bash
# Verify WASM exports
cd web
python3 -c "import sys; print(open('auto_pqdfium_rs.js').read()[:1000])"
```

Should see `_pdfium_wasm_initialize` in the exports list.

## 📊 Verify Everything Works

**Test Page:** http://localhost:8080/test.html

Should show:
```
✅ Instance created
✅ PDFium initialized successfully!
✅ Loaded 542 bytes
✅ Extracted [N] characters in [X]ms
✅ Converted in [X]ms
✅ Cleanup complete

🎉 ALL TESTS PASSED!
```

## 🔍 Debug Info

Check what the browser is receiving:
```bash
# Test MIME type
curl -I http://localhost:8080/auto_pqdfium_rs.wasm | grep Content-type
# Should show: Content-type: application/wasm

# Test cache headers
curl -I http://localhost:8080/auto_pqdfium_rs.wasm | grep Cache-Control
# Should show: Cache-Control: no-store, no-cache, must-revalidate, max-age=0
```

## 💡 Development Tips

1. **Always use hard refresh** during development
2. **Use incognito mode** to avoid cache issues
3. **Check DevTools Console** for errors
4. **Check Network tab** to verify files are being reloaded

## 🚀 Quick Reset

If nothing works, do a complete reset:
```bash
# 1. Stop server
pkill -9 -f server.py

# 2. Rebuild WASM
./build-wasm.sh

# 3. Clear browser cache completely
# 4. Restart server
cd web && python3 server.py

# 5. Open in incognito mode
```

## 📞 Server is Running

Current server: http://localhost:8080
- ✅ MIME types: Correct
- ✅ Cache headers: Disabled
- ✅ CORS: Enabled
- ✅ Files: Up to date

**Just clear your browser cache and test!** 🎉
