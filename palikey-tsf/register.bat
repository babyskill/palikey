@echo off
:: PaliKey TSF Text Service Registration Script
:: Requires Administrator privileges

echo ===================================================
echo  Registering PaliKey TSF Text Service (PaliKeyTSF.dll)
echo ===================================================

net session >nul 2>&1
if %errorLevel% neq 0 (
    echo [ERROR] Administrator permissions are required.
    echo Please right-click this batch script and select "Run as administrator".
    pause
    exit /b 1
)

regsvr32.exe /s "%~dp0PaliKeyTSF.dll"
if %errorLevel% equ 0 (
    echo [SUCCESS] PaliKeyTSF.dll registered successfully!
    echo PaliKey (Pāḷi) is now active in Windows Language / Input Settings (Win + Space).
) else (
    echo [ERROR] Failed to register PaliKeyTSF.dll (Error code: %errorLevel%).
    pause
    exit /b %errorLevel%
)

pause
