@echo off
echo ========================================
echo  STARTING LEPTOS FRONTEND
echo ========================================
echo.
echo Make sure backend is running on port 8000!
echo Frontend will be available at: http://127.0.0.1:8080
echo.
cd /d "E:\CPE\Project\Rust-spfresh\frontend"
echo Starting Leptos frontend...
echo Press Ctrl+C to stop the server
echo.
trunk serve
