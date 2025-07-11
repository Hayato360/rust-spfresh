@echo off
echo Building and testing the backend...

echo Building backend...
cd backend
cargo build

if %errorlevel% equ 0 (
    echo ✅ Backend built successfully!
    
    echo Starting backend server...
    start /B cargo run -- --port 8000 --data-dir ./data
    
    timeout /t 5
    
    echo Testing health endpoint...
    curl -s http://localhost:8000/health
    
    echo.
    echo Testing stats endpoint...
    curl -s http://localhost:8000/stats
    
    echo.
    echo Testing insert review...
    curl -s -X POST http://localhost:8000/reviews -H "Content-Type: application/json" -d "{\"review_title\":\"Test Review\",\"review_body\":\"This is a test review with good quality and excellent performance\",\"product_id\":\"TEST-001\",\"review_rating\":5}"
    
    echo.
    echo Testing search...
    curl -s -X POST http://localhost:8000/search -H "Content-Type: application/json" -d "{\"query\":\"good quality\",\"limit\":5}"
    
    echo.
    echo ✅ Backend test completed!
    echo Note: You may need to manually stop the server process
) else (
    echo ❌ Backend build failed!
)

pause
