@echo off
echo ========================================
echo  REVIEW SEARCH PLATFORM - QUICK START
echo ========================================
echo.
echo This will start both servers automatically!
echo.
echo 1. Backend: http://localhost:8000
echo 2. Frontend: http://127.0.0.1:8080
echo.
echo Press any key to start...
pause >nul
echo.
echo Starting Backend Server...
start "Backend Server" cmd /k "cd /d E:\CPE\Project\Rust-spfresh\backend && cargo run --release"
echo.
echo Waiting 10 seconds for backend to start...
timeout /t 10 /nobreak >nul
echo.
echo Starting Frontend Server...
start "Frontend Server" cmd /k "cd /d E:\CPE\Project\Rust-spfresh\frontend && trunk serve"
echo.
echo ========================================
echo  SERVERS STARTED!
echo ========================================
echo Backend:  http://localhost:8000
echo Frontend: http://127.0.0.1:8080
echo.
echo Both servers are running in separate windows.
echo Close those windows to stop the servers.
echo.
pause
