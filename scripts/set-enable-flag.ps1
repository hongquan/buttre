# Manually set Enable flag in registry
# Run as Administrator

$tipKey = "HKLM:\SOFTWARE\Microsoft\CTF\TIP\{E6B8A6C0-1234-5678-9ABC-DEF012345678}\LanguageProfile"
$profileGuid = "{B7447743-7652-4AB6-8D82-250D935EBCC0}"
# MSI registers under vi-VN (0x0000042A); regsvr32/code path adds en-US (0x00000409) as well.
$lcids = @("0x0000042A", "0x00000409")

Write-Host "Setting Enable flag in registry..." -ForegroundColor Yellow

$found = $false
foreach ($lcid in $lcids) {
    $regPath = "$tipKey\$lcid\$profileGuid"
    if (Test-Path $regPath) {
        Set-ItemProperty -Path $regPath -Name "Enable" -Value 1 -Type DWord
        $value = (Get-ItemProperty $regPath -Name Enable).Enable
        if ($value -eq 1) {
            Write-Host "SUCCESS ($lcid): Enable flag set to 1" -ForegroundColor Green
            $found = $true
        }
    }
}
if (-not $found) {
    Write-Host "ERROR: Registry profile not found under any LCID ($($lcids -join ', '))" -ForegroundColor Red
    Write-Host "Run: regsvr32 /s target\release\buttre_platform.dll  (or reinstall via MSI)" -ForegroundColor Yellow
}

Write-Host ""
Write-Host "Restarting CTF service..." -ForegroundColor Yellow
taskkill /f /im ctfmon.exe 2>$null
Start-Sleep -Milliseconds 500
Start-Process ctfmon.exe

Write-Host ""
Write-Host "Done! Now try typing in Notepad with buttre keyboard selected." -ForegroundColor Cyan
$logPath = Join-Path $env:TEMP "buttre_tsf_debug.log"
Write-Host "Check log: $logPath" -ForegroundColor Gray
