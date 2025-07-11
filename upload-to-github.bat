@echo off
echo ========================================
echo  UPLOADING RUST-SPFRESH TO GITHUB
echo ========================================
echo.

cd /d "E:\CPE\Project\Rust-spfresh"

echo Step 1: Initializing Git repository...
git init

echo Step 2: Adding files to Git...
git add .gitignore
git add README.md
git add rust-spfresh.md
git add PROJECT_SUMMARY.md
git add docker-compose.yml
git add *.bat

echo Step 3: Adding backend files...
git add backend/src/
git add backend/Cargo.toml
git add backend/data/

echo Step 4: Adding frontend files...
git add frontend/src/
git add frontend/Cargo.toml
git add frontend/Trunk.toml
git add frontend/index.html

echo Step 5: Adding simple frontend...
git add frontend-simple/

echo Step 6: Creating commit...
git commit -m "Initial commit: Review Semantic Search Platform

- Complete Rust/axum backend with TF-IDF semantic search
- Leptos frontend with modern WASM interface
- Simple HTML/JS frontend alternative
- File-based storage (no database required)
- Append-only review system
- Docker compose configuration
- Batch scripts for easy development
- BRD compliant implementation

Features:
- Semantic search using vector similarity
- Real-time review insertion
- Multiple frontend options
- Cross-platform compatibility
- Production-ready deployment"

echo.
echo ========================================
echo  READY TO PUSH TO GITHUB
echo ========================================
echo.
echo Next steps:
echo 1. Create repository on GitHub
echo 2. Copy and run these commands:
echo.
echo    git remote add origin https://github.com/Hayato360/rust-spfresh.git
echo    git branch -M main
echo    git push -u origin main
echo.
echo Repository prepared successfully!
pause
