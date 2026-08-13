@echo off
setlocal
cd /d "%~dp0"
powershell.exe -NoProfile -ExecutionPolicy Bypass -File "%~dp0scripts\build-release.ps1"
if errorlevel 1 exit /b %errorlevel%
echo.
echo Peeky release is ready in %~dp0dist
