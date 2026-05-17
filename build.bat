@echo off
cd /d "%~dp0"
echo Building Forza DualSense Tauri Release...
echo.
npm run tauri build

echo.
echo Copying backend files to release folder...
xcopy src-tauri\backend src-tauri\target\release\backend /E /I /Y
echo Backend files copied. They will be copied to AppData on first run.

pause
