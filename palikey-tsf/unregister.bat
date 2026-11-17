@echo off
:: PaliKey TSF Text Service Unregistration Script
:: Requires Administrator privileges

echo ===================================================
echo  Unregistering PaliKey TSF Text Service (PaliKeyTSF.dll)
echo ===================================================

net session >nul 2>&1
if %errorLevel% neq 0 (
    echo [ERROR] Administrator permissions are required.
    echo Please right-click this batch script and select "Run as administrator".
    pause
    exit /b 1
)

regsvr32.exe /u /s "%~dp0PaliKeyTSF.dll"
if %errorLevel% equ 0 (
    echo [SUCCESS] PaliKeyTSF.dll unregistered successfully.
) else (
    echo [ERROR] Failed to unregister PaliKeyTSF.dll (Error code: %errorLevel%).
    pause
    exit /b %errorLevel%
)

pause
