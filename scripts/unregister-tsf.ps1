# Unregister buttre TSF DLL
# Run this script as Administrator

$ErrorActionPreference = "Stop"

Write-Host "=== buttre TSF Unregistration ===" -ForegroundColor Cyan
Write-Host ""

# Check if running as Administrator
$isAdmin = ([Security.Principal.WindowsPrincipal] [Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole] "Administrator")
if (-not $isAdmin) {
    Write-Host "ERROR: This script must be run as Administrator!" -ForegroundColor Red
    Write-Host "Right-click PowerShell and select 'Run as Administrator'" -ForegroundColor Yellow
    exit 1
}

# Get DLL path
$dllPath = Join-Path $PSScriptRoot "target\release\buttre_platform.dll"

if (-not (Test-Path $dllPath)) {
    Write-Host "WARNING: DLL not found at: $dllPath" -ForegroundColor Yellow
    Write-Host "Continuing anyway to clean registry..." -ForegroundColor Yellow
    Write-Host ""
}

# Unregister
Write-Host "Unregistering buttre TSF..." -ForegroundColor Yellow
$result = & regsvr32.exe /u /s "$dllPath" 2>&1

if ($LASTEXITCODE -eq 0) {
    Write-Host "SUCCESS: buttre TSF unregistered successfully!" -ForegroundColor Green
} else {
    Write-Host "WARNING: Unregistration may have failed (exit code: $LASTEXITCODE)" -ForegroundColor Yellow
    Write-Host "This is normal if TSF was not registered." -ForegroundColor Gray
}

Write-Host ""
Write-Host "buttre TSF has been removed from the system." -ForegroundColor Green
