# Simple Backend Test
Write-Host "=== buttre Backend Test ===" -ForegroundColor Cyan
Write-Host ""

# Check Vietnamese language
$hasVi = (Get-WinUserLanguageList | Where-Object { $_.LanguageTag -like "vi*" }).Count -gt 0
Write-Host "Vietnamese language installed: $hasVi" -ForegroundColor $(if ($hasVi) { "Green" } else { "Yellow" })
Write-Host "Expected backend: $(if ($hasVi) { 'TSF' } else { 'Hook (fallback)' })" -ForegroundColor Cyan
Write-Host ""

# Start buttre
Write-Host "Starting buttre..." -ForegroundColor Yellow
Start-Process "target\release\buttre.exe"
Start-Sleep -Seconds 2

# Open Notepad for testing
Write-Host "Opening Notepad for testing..." -ForegroundColor Yellow
Start-Process "notepad.exe"
Start-Sleep -Seconds 1

Write-Host ""
Write-Host "TEST INSTRUCTIONS:" -ForegroundColor Cyan
Write-Host "1. Click in Notepad window" -ForegroundColor White
Write-Host "2. Try typing: 'aa' (should become 'â' if Hook backend is working)" -ForegroundColor White
Write-Host "3. Try typing: 'viet nam' (should work normally)" -ForegroundColor White
Write-Host ""
Write-Host "If you can type Vietnamese, the backend is working!" -ForegroundColor Green
Write-Host "If nothing happens, check the tray icon and try switching methods" -ForegroundColor Yellow
