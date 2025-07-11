@echo off
echo Starting Leptos Frontend Server...
cd /d "E:\CPE\Project\Rust-spfresh\frontend"
echo Backend should be running on http://localhost:8000
echo Frontend will be available at http://127.0.0.1:8080
echo.
echo Press Ctrl+C to stop the server
echo.
trunk serve
pause
