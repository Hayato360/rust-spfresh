@echo off
echo ========================================
echo  STARTING REVIEW SEARCH PLATFORM
echo ========================================
echo.
echo Starting Backend Server...
cd /d "E:\CPE\Project\Rust-spfresh\backend"
echo Backend will be available at: http://localhost:8000
echo.
echo Starting in 3 seconds...
timeout /t 3 /nobreak >nul
cargo run --release
