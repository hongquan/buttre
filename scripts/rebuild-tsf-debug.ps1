# Rebuild TSF with DEBUG logging
# Run as Administrator

$ErrorActionPreference = "Stop"

Write-Host "=== Rebuild buttre TSF (DEBUG MODE) ===" -ForegroundColor Cyan
Write-Host ""

# Check admin
$isAdmin = ([Security.Principal.WindowsPrincipal] [Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole] "Administrator")
if (-not $isAdmin) {
    Write-Host "ERROR: Must run as Administrator!" -ForegroundColor Red
    exit 1
}

$dllPath = Join-Path $PSScriptRoot "target\debug\buttre_platform.dll"
$releaseDll = Join-Path $PSScriptRoot "target\release\buttre_platform.dll"

# Step 1: Unregister release DLL
Write-Host "Step 1: Unregistering release DLL..." -ForegroundColor Yellow
if (Test-Path $releaseDll) {
    & regsvr32.exe /u /s "$releaseDll" 2>$null
}
Write-Host "  Done" -ForegroundColor Gray

# Step 2: Stop processes
Write-Host "Step 2: Stopping TSF processes..." -ForegroundColor Yellow
Get-Process -Name "TextInputHost" -ErrorAction SilentlyContinue | Stop-Process -Force -ErrorAction SilentlyContinue
Stop-Process -Name "ctfmon" -Force -ErrorAction SilentlyContinue
Start-Sleep -Seconds 2

# Rename old debug DLL
if (Test-Path $dllPath) {
    try {
        Move-Item $dllPath ($dllPath + ".old") -Force
        Write-Host "  Old debug DLL backed up" -ForegroundColor Gray
    } catch {
        Write-Host "  WARNING: Could not backup old DLL" -ForegroundColor Yellow
    }
}
Write-Host "  Done" -ForegroundColor Gray

# Step 3: Build DEBUG version
Write-Host "Step 3: Building DEBUG DLL (with logging)..." -ForegroundColor Yellow
& cargo build -p buttre-platform
if ($LASTEXITCODE -ne 0) {
    Write-Host "ERROR: Build failed!" -ForegroundColor Red
    exit 1
}
Write-Host "  Build successful" -ForegroundColor Green

# Step 4: Register debug DLL
Write-Host "Step 4: Registering DEBUG DLL..." -ForegroundColor Yellow
& regsvr32.exe /s "$dllPath"
if ($LASTEXITCODE -eq 0) {
    Write-Host "  Registered successfully!" -ForegroundColor Green
} else {
    Write-Host "  Registration failed (code: $LASTEXITCODE)" -ForegroundColor Red
    exit 1
}

Write-Host ""
Write-Host "SUCCESS: DEBUG TSF ready!" -ForegroundColor Green
Write-Host ""
Write-Host "Next steps:" -ForegroundColor Cyan
Write-Host "1. Download DebugView: https://learn.microsoft.com/sysinternals/downloads/debugview" -ForegroundColor White
Write-Host "2. Run DebugView as Administrator" -ForegroundColor White
Write-Host "3. Enable: Capture > Capture Global Win32" -ForegroundColor White
Write-Host "4. Type in Notepad with buttre TSF" -ForegroundColor White
Write-Host "5. Watch logs in DebugView!" -ForegroundColor White
Write-Host ""
Write-Host "Alternative: Check Event Viewer > Windows Logs > Application" -ForegroundColor Gray
