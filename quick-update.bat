@echo off
echo Updating GitHub repository...
cd /d "E:\CPE\Project\Rust-spfresh"
git add .
git commit -m "Update: %date% %time%"
git push origin main
echo Update complete!
pause
