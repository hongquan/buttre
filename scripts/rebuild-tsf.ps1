# Rebuild and re-register TSF DLL
# Run as Administrator

$ErrorActionPreference = "Stop"

Write-Host "=== Rebuild buttre TSF ===" -ForegroundColor Cyan
Write-Host ""

# Check admin
$isAdmin = ([Security.Principal.WindowsPrincipal] [Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole] "Administrator")
if (-not $isAdmin) {
    Write-Host "ERROR: Must run as Administrator!" -ForegroundColor Red
    exit 1
}

$dllPath = Join-Path $PSScriptRoot "target\release\buttre_platform.dll"

# Step 1: Unregister old DLL
Write-Host "Step 1: Unregistering old DLL..." -ForegroundColor Yellow
if (Test-Path $dllPath) {
    & regsvr32.exe /u /s "$dllPath" 2>$null
    Write-Host "  Unregistered" -ForegroundColor Gray
}

# Step 2: Kill processes that might be using DLL
Write-Host "Step 2: Stopping TSF processes..." -ForegroundColor Yellow
Get-Process -Name "TextInputHost" -ErrorAction SilentlyContinue | Stop-Process -Force -ErrorAction SilentlyContinue
Stop-Process -Name "ctfmon" -Force -ErrorAction SilentlyContinue
Stop-Process -Name "SearchHost" -Force -ErrorAction SilentlyContinue

# Wait for processes to release DLL
Start-Sleep -Seconds 2

# If DLL still locked, try to rename it
if (Test-Path $dllPath) {
    try {
        $backupPath = $dllPath + ".old"
        if (Test-Path $backupPath) { Remove-Item $backupPath -Force }
        Move-Item $dllPath $backupPath -Force -ErrorAction Stop
        Write-Host "  Old DLL moved to .old" -ForegroundColor Gray
    } catch {
        Write-Host "  WARNING: Could not move old DLL, will try to overwrite" -ForegroundColor Yellow
    }
}

Write-Host "  Done" -ForegroundColor Gray

# Step 3: Rebuild
Write-Host "Step 3: Rebuilding DLL..." -ForegroundColor Yellow
& cargo build --release -p buttre-platform
if ($LASTEXITCODE -ne 0) {
    Write-Host "ERROR: Build failed!" -ForegroundColor Red
    exit 1
}
Write-Host "  Build successful" -ForegroundColor Green

# Step 4: Register new DLL
Write-Host "Step 4: Registering new DLL..." -ForegroundColor Yellow
& regsvr32.exe /s "$dllPath"
if ($LASTEXITCODE -eq 0) {
    Write-Host "  Registered successfully!" -ForegroundColor Green
} else {
    Write-Host "  Registration failed (code: $LASTEXITCODE)" -ForegroundColor Red
    exit 1
}

Write-Host ""
Write-Host "SUCCESS: TSF rebuilt and registered!" -ForegroundColor Green
Write-Host ""
Write-Host "Next: Test typing in Notepad with buttre keyboard selected" -ForegroundColor Cyan
Write-Host "To view debug logs: DebugView or check Event Viewer" -ForegroundColor Gray
