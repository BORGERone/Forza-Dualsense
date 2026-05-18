@echo off
cd /d "%~dp0"
echo Building Forza DualSense Tauri Release...
echo.
npm run tauri build

echo.
echo Build complete! Backend files are embedded in the exe.
echo They will be extracted to AppData on first run.

pause
