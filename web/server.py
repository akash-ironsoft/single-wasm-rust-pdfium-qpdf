#!/usr/bin/env python3
"""
Simple HTTP server for testing WASM module in browser
"""

import http.server
import socketserver
import os

PORT = 8080

class MyHTTPRequestHandler(http.server.SimpleHTTPRequestHandler):
    def guess_type(self, path):
        """Override to set correct MIME type for WASM files"""
        if path.endswith('.wasm'):
            return 'application/wasm'
        return super().guess_type(path)

    def end_headers(self):
        # Enable CORS
        self.send_header('Access-Control-Allow-Origin', '*')
        self.send_header('Access-Control-Allow-Methods', 'GET, POST, OPTIONS')
        self.send_header('Access-Control-Allow-Headers', '*')

        # Enable Cross-Origin isolation (required for SharedArrayBuffer)
        self.send_header('Cross-Origin-Opener-Policy', 'same-origin')
        self.send_header('Cross-Origin-Embedder-Policy', 'require-corp')

        super().end_headers()

if __name__ == '__main__':
    os.chdir(os.path.dirname(os.path.abspath(__file__)))

    with socketserver.TCPServer(("", PORT), MyHTTPRequestHandler) as httpd:
        print(f"\n🚀 Server running at http://localhost:{PORT}")
        print(f"📂 Serving files from: {os.getcwd()}")
        print(f"\n✨ Open http://localhost:{PORT} in your browser")
        print(f"Press Ctrl+C to stop\n")

        try:
            httpd.serve_forever()
        except KeyboardInterrupt:
            print("\n\n👋 Server stopped")
