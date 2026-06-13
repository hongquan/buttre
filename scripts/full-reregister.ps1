# Complete unregister and re-register TSF
# Must run as Administrator

Write-Host "=== Full TSF Re-registration ===" -ForegroundColor Cyan
Write-Host ""

$repoRoot = Resolve-Path "$PSScriptRoot\.."
$dllPath = Join-Path $repoRoot "target\debug\buttre_platform.dll"
if (-not (Test-Path $dllPath)) {
    $dllPath = Join-Path $repoRoot "target\release\buttre_platform.dll"
}

# Step 1: Unregister
Write-Host "1. Unregistering TSF..." -ForegroundColor Yellow
regsvr32.exe /s /u $dllPath
Start-Sleep -Milliseconds 500

# Step 2: Delete registry keys manually
Write-Host "2. Cleaning registry..." -ForegroundColor Yellow
$paths = @(
    "HKLM:\SOFTWARE\Classes\CLSID\{E6B8A6C0-1234-5678-9ABC-DEF012345678}",
    "HKLM:\SOFTWARE\Microsoft\CTF\TIP\{E6B8A6C0-1234-5678-9ABC-DEF012345678}"
)

foreach ($path in $paths) {
    if (Test-Path $path) {
        Remove-Item -Path $path -Recurse -Force -ErrorAction SilentlyContinue
        Write-Host "   Deleted: $path" -ForegroundColor Gray
    }
}

# Step 3: Register
Write-Host "3. Registering TSF..." -ForegroundColor Yellow
regsvr32.exe /s $dllPath

if ($LASTEXITCODE -eq 0) {
    Write-Host "   Registration completed" -ForegroundColor Green
} else {
    Write-Host "   Registration FAILED (exit code: $LASTEXITCODE)" -ForegroundColor Red
    exit $LASTEXITCODE
}

Start-Sleep -Milliseconds 500

# Step 4: Verify Enable flag
Write-Host "4. Checking Enable flag..." -ForegroundColor Yellow
# MSI registers under vi-VN (0x0000042A); regsvr32 adds en-US (0x00000409) as well.
$regPath = "HKLM:\SOFTWARE\Microsoft\CTF\TIP\{E6B8A6C0-1234-5678-9ABC-DEF012345678}\LanguageProfile\0x0000042A\{B7447743-7652-4AB6-8D82-250D935EBCC0}"
if (-not (Test-Path $regPath)) {
    $regPath = "HKLM:\SOFTWARE\Microsoft\CTF\TIP\{E6B8A6C0-1234-5678-9ABC-DEF012345678}\LanguageProfile\0x00000409\{B7447743-7652-4AB6-8D82-250D935EBCC0}"
}

if (Test-Path $regPath) {
    $props = Get-ItemProperty $regPath
    Write-Host "   Profile exists" -ForegroundColor Green
    
    if ($props.PSObject.Properties.Name -contains "Enable") {
        Write-Host "   Enable = $($props.Enable)" -ForegroundColor Green
    } else {
        Write-Host "   Enable flag MISSING!" -ForegroundColor Red
        Write-Host "   Manually setting Enable flag..." -ForegroundColor Yellow
        Set-ItemProperty -Path $regPath -Name "Enable" -Value 1 -Type DWord
        Write-Host "   Enable flag set to 1" -ForegroundColor Green
    }
} else {
    Write-Host "   Profile NOT found!" -ForegroundColor Red
}

# Step 5: Restart CTF
Write-Host "5. Restarting CTF..." -ForegroundColor Yellow
taskkill /f /im ctfmon.exe 2>$null
Start-Sleep -Milliseconds 500
Start-Process ctfmon.exe

Write-Host ""
Write-Host "=== Done ===" -ForegroundColor Cyan
Write-Host "Open Notepad, select buttre keyboard (Win+Space), and type." -ForegroundColor White
$logPath = Join-Path $env:TEMP "buttre_tsf_debug.log"
Write-Host "Check log: $logPath" -ForegroundColor Gray
