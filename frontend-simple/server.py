#!/usr/bin/env python3
"""
Simple HTTP server for the frontend.
Usage: python server.py [port]
"""

import sys
import os
import http.server
import socketserver
from pathlib import Path

# Default port
PORT = 3000

# Check if port is provided as argument
if len(sys.argv) > 1:
    try:
        PORT = int(sys.argv[1])
    except ValueError:
        print("Invalid port number. Using default port 3000.")

# Change to the directory containing this script
script_dir = Path(__file__).parent
os.chdir(script_dir)

# Create server
handler = http.server.SimpleHTTPRequestHandler

# Add CORS headers for API calls
class CORSRequestHandler(http.server.SimpleHTTPRequestHandler):
    def end_headers(self):
        self.send_header('Access-Control-Allow-Origin', '*')
        self.send_header('Access-Control-Allow-Methods', 'GET, POST, OPTIONS')
        self.send_header('Access-Control-Allow-Headers', 'Content-Type')
        super().end_headers()

with socketserver.TCPServer(("", PORT), CORSRequestHandler) as httpd:
    print(f"🚀 Frontend server running at http://localhost:{PORT}")
    print(f"📁 Serving files from: {script_dir}")
    print("Press Ctrl+C to stop the server")
    
    try:
        httpd.serve_forever()
    except KeyboardInterrupt:
        print("\n👋 Server stopped")
        httpd.shutdown()
