@echo off
set "PATH=C:\msys64\mingw64\bin;C:\msys64\usr\bin;%USERPROFILE%\.cargo\bin;%PATH%"
cd /d "%~dp0"
echo =======================================================
echo Compilando instaladores do win-island (.exe e .msi)...
echo =======================================================
npm run tauri build
echo.
echo =======================================================
echo Concluido! Instaladores gerados em:
echo src-tauri\target\release\bundle\
echo =======================================================
pause
