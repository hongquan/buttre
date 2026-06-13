# Re-register TSF with new Enable flag
# Run as Administrator

Write-Host "=== Re-registering buttre TSF ===" -ForegroundColor Cyan

$repoRoot = Resolve-Path "$PSScriptRoot\.."
$dllPath = Join-Path $repoRoot "target\debug\buttre_platform.dll"
if (-not (Test-Path $dllPath)) {
    $dllPath = Join-Path $repoRoot "target\release\buttre_platform.dll"
}

if (-not (Test-Path $dllPath)) {
    Write-Host "ERROR: DLL not found at $dllPath" -ForegroundColor Red
    exit 1
}

Write-Host "1. Unregistering old TSF..." -ForegroundColor Yellow
& regsvr32.exe /s /u $dllPath
Start-Sleep -Milliseconds 500

Write-Host "2. Registering new TSF with Enable flag..." -ForegroundColor Yellow
& regsvr32.exe /s $dllPath

if ($LASTEXITCODE -eq 0) {
    Write-Host "SUCCESS! TSF registered" -ForegroundColor Green
    
    Write-Host "`n3. Verifying Enable flag..." -ForegroundColor Yellow
    # Check both vi-VN (MSI) and en-US (regsvr32/code path) LCIDs
    $enableValue = Get-ItemProperty "HKLM:\SOFTWARE\Microsoft\CTF\TIP\{E6B8A6C0-1234-5678-9ABC-DEF012345678}\LanguageProfile\0x0000042A\{B7447743-7652-4AB6-8D82-250D935EBCC0}" -Name Enable -ErrorAction SilentlyContinue
    if (-not $enableValue) {
        $enableValue = Get-ItemProperty "HKLM:\SOFTWARE\Microsoft\CTF\TIP\{E6B8A6C0-1234-5678-9ABC-DEF012345678}\LanguageProfile\0x00000409\{B7447743-7652-4AB6-8D82-250D935EBCC0}" -Name Enable -ErrorAction SilentlyContinue
    }
    
    if ($enableValue -and $enableValue.Enable -eq 1) {
        Write-Host "   Enable = 1 (CORRECT!)" -ForegroundColor Green
    } else {
        Write-Host "   Enable not set or wrong value!" -ForegroundColor Red
    }
} else {
    Write-Host "FAILED to register! Exit code: $LASTEXITCODE" -ForegroundColor Red
    exit $LASTEXITCODE
}

Write-Host "`n4. Next steps:" -ForegroundColor Cyan
Write-Host "   - Open Notepad" -ForegroundColor White
Write-Host "   - Press Win+Space to select 'buttre - Vietnamese Input'" -ForegroundColor White
Write-Host "   - Type some text" -ForegroundColor White
Write-Host "   - Check log: `$env:TEMP\buttre_tsf_debug.log" -ForegroundColor White
