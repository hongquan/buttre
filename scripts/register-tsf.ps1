# Register buttre TSF DLL
# Run this script as Administrator

$ErrorActionPreference = "Stop"

Write-Host "=== buttre TSF Registration ===" -ForegroundColor Cyan
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
    Write-Host "ERROR: DLL not found at: $dllPath" -ForegroundColor Red
    Write-Host "Please build the release version first:" -ForegroundColor Yellow
    Write-Host "  cargo build --release" -ForegroundColor Yellow
    exit 1
}

Write-Host "DLL found: $dllPath" -ForegroundColor Green
Write-Host "DLL size: $((Get-Item $dllPath).Length / 1MB) MB" -ForegroundColor Gray
Write-Host ""

# Unregister old version (if any)
Write-Host "Unregistering old version (if exists)..." -ForegroundColor Yellow
& regsvr32.exe /u /s "$dllPath" 2>$null

# Register new version
Write-Host "Registering buttre TSF..." -ForegroundColor Yellow
$result = & regsvr32.exe /s "$dllPath" 2>&1

if ($LASTEXITCODE -eq 0) {
    Write-Host "SUCCESS: buttre TSF registered successfully!" -ForegroundColor Green
    Write-Host ""
    Write-Host "Next steps:" -ForegroundColor Cyan
    Write-Host "1. Open Windows Settings > Time & Language > Language & Region" -ForegroundColor White
    Write-Host "2. Add Vietnamese language (if not already added)" -ForegroundColor White
    Write-Host "3. Click on Vietnamese > Options > Add keyboard" -ForegroundColor White
    Write-Host "4. Select 'buttre - Vietnamese Input'" -ForegroundColor White
    Write-Host "5. Switch to Vietnamese keyboard (Win+Space)" -ForegroundColor White
    Write-Host "6. Open Notepad and test typing!" -ForegroundColor White
    Write-Host ""
    Write-Host "Performance: After optimization, TSF typing should be smooth with no lag!" -ForegroundColor Green
} else {
    Write-Host "ERROR: Registration failed!" -ForegroundColor Red
    Write-Host "Error code: $LASTEXITCODE" -ForegroundColor Red
    Write-Host ""
    Write-Host "Common issues:" -ForegroundColor Yellow
    Write-Host "- Make sure you're running as Administrator" -ForegroundColor White
    Write-Host "- DLL might be in use - restart computer and try again" -ForegroundColor White
    Write-Host "- Check Windows Event Viewer for details" -ForegroundColor White
    exit 1
}
