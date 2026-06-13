# Quick fix: Force unlock DLL by restarting Explorer
# Run as Administrator

Write-Host "=== Force Unlock TSF DLL ===" -ForegroundColor Cyan

# Stop Explorer (releases all shell extensions and DLLs)
Write-Host "Stopping Explorer..." -ForegroundColor Yellow
Stop-Process -Name "explorer" -Force -ErrorAction SilentlyContinue

# Kill TSF processes
Get-Process -Name "TextInputHost" -ErrorAction SilentlyContinue | Stop-Process -Force
Stop-Process -Name "ctfmon" -Force -ErrorAction SilentlyContinue
Stop-Process -Name "SearchHost" -Force -ErrorAction SilentlyContinue

Start-Sleep -Seconds 2

# Restart Explorer
Write-Host "Restarting Explorer..." -ForegroundColor Yellow
Start-Process "explorer.exe"

Write-Host ""
Write-Host "Done! Now you can rebuild:" -ForegroundColor Green
Write-Host "  .\rebuild-tsf.ps1" -ForegroundColor White
