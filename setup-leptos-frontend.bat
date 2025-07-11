@echo off
echo ============================================
echo  LEPTOS FRONTEND SETUP (BRD Requirement)
echo ============================================
echo.

echo Installing required tools for Leptos frontend...
echo.

echo 1. Installing trunk (Leptos build tool)...
cargo install trunk --locked
if %errorlevel% neq 0 (
    echo ❌ Trunk installation failed. Trying without --locked...
    cargo install trunk
)

echo.
echo 2. Adding WASM target...
rustup target add wasm32-unknown-unknown

echo.
echo 3. Building Leptos frontend...
cd frontend
trunk build

if %errorlevel% equ 0 (
    echo ✅ Leptos frontend built successfully!
    echo.
    echo To run the Leptos frontend:
    echo   cd frontend
    echo   trunk serve --port 3000
    echo.
    echo Then open: http://localhost:3000
) else (
    echo ❌ Build failed. Alternative option available:
    echo   cd frontend-simple
    echo   python server.py 3000
)

echo.
echo ============================================
echo  Leptos Frontend Setup Complete
echo ============================================
pause
