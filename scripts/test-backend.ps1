# Test buttre Backend Selection
Write-Host "=== Testing buttre Backend Selection ===" -ForegroundColor Cyan
Write-Host ""

# Stop existing buttre process
Write-Host "[1] Stopping existing buttre process..." -ForegroundColor Yellow
Stop-Process -Name "buttre" -Force -ErrorAction SilentlyContinue
Start-Sleep -Seconds 1

# Check Vietnamese language status
Write-Host "[2] Checking Vietnamese language..." -ForegroundColor Yellow
$hasVietnamese = (Get-WinUserLanguageList | Where-Object { $_.LanguageTag -like "vi*" }).Count -gt 0

if ($hasVietnamese) {
    Write-Host "Vietnamese language is INSTALLED" -ForegroundColor Green
    Write-Host "Expected backend: TSF" -ForegroundColor Cyan
} else {
    Write-Host "Vietnamese language is NOT installed" -ForegroundColor Yellow
    Write-Host "Expected backend: Hook (fallback)" -ForegroundColor Cyan
}

Write-Host ""
Write-Host "[3] Starting buttre with logging..." -ForegroundColor Yellow

# Start buttre with logging enabled
$env:RUST_LOG = "info"
$logFile = "$env:TEMP\buttre_backend_test.log"

# Start buttre and capture output
$process = Start-Process -FilePath "target\release\buttre.exe" `
    -RedirectStandardError $logFile `
    -PassThru `
    -WindowStyle Hidden

Write-Host "buttre started (PID: $($process.Id))" -ForegroundColor Green
Write-Host "Waiting for initialization..." -ForegroundColor Gray
Start-Sleep -Seconds 3

# Check log for backend info
if (Test-Path $logFile) {
    Write-Host ""
    Write-Host "[4] Backend initialization log:" -ForegroundColor Yellow
    $logContent = Get-Content $logFile -Raw
    
    if ($logContent -match "TSF backend initialized") {
        Write-Host "RESULT: TSF backend is running" -ForegroundColor Green
    } elseif ($logContent -match "Hook backend initialized") {
        Write-Host "RESULT: Hook backend is running (TSF fallback)" -ForegroundColor Green
    } elseif ($logContent -match "TSF initialization failed") {
        Write-Host "RESULT: TSF failed, Hook backend should be active" -ForegroundColor Yellow
        if ($logContent -match "Hook backend initialized") {
            Write-Host "Confirmed: Hook backend is running" -ForegroundColor Green
        }
    } else {
        Write-Host "WARNING: Could not determine backend from log" -ForegroundColor Red
        Write-Host "Log content:" -ForegroundColor Gray
        Write-Host $logContent
    }
} else {
    Write-Host "WARNING: Log file not found" -ForegroundColor Red
}

Write-Host ""
Write-Host "[5] Testing keyboard input..." -ForegroundColor Yellow
Write-Host "Try typing in Notepad to test if Vietnamese input works" -ForegroundColor White
Write-Host "Press any key to stop buttre and exit..." -ForegroundColor Gray
$null = $Host.UI.RawUI.ReadKey("NoEcho,IncludeKeyDown")

# Cleanup
Stop-Process -Id $process.Id -Force -ErrorAction SilentlyContinue
Write-Host "buttre stopped" -ForegroundColor Yellow
