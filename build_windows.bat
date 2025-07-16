@echo off
REM Windows build script for SPFresh integration (requires WSL or Linux environment)

echo === SPFresh + Rust Backend Build Script for Windows ===
echo.
echo NOTE: This script requires WSL (Windows Subsystem for Linux) or a Linux VM
echo because SPFresh requires Linux build environment.
echo.
echo Alternative: Use Docker Desktop for Windows with WSL2 backend
echo.

echo Step 1: Using Docker Compose (Recommended for Windows)
echo docker-compose up --build
echo.

echo Step 2: Manual build in WSL (if available)
echo.
echo In WSL terminal:
echo   sudo apt update
echo   sudo apt install cmake build-essential libjemalloc-dev libsnappy-dev libgflags-dev
echo   sudo apt install pkg-config libboost-all-dev libtbb-dev libgoogle-perftools-dev gcc-9 g++-9
echo.
echo   chmod +x build_spfresh_integration.sh
echo   ./build_spfresh_integration.sh
echo.

echo Step 3: Or use the provided Docker environment
echo   docker-compose up --build
echo.

echo For development on Windows, we recommend using VS Code with:
echo   - Remote - WSL extension
echo   - Docker extension
echo   - rust-analyzer extension
echo.

pause
