@echo off
set "PATH=C:\msys64\mingw64\bin;C:\msys64\usr\bin;%USERPROFILE%\.cargo\bin;%PATH%"
cd /d "%~dp0"
echo Iniciando win-island em modo de desenvolvimento...
npm run tauri dev
pause
