# Check TSF Language Profile details
$tipPath = "HKLM:\SOFTWARE\Microsoft\CTF\TIP\{E6B8A6C0-1234-5678-9ABC-DEF012345678}"

Write-Host "Checking language profiles..." -ForegroundColor Cyan

$langProfilePath = "$tipPath\LanguageProfile\0x00000409"
if (Test-Path $langProfilePath) {
    Write-Host "English US profile exists" -ForegroundColor Green
    
    $profiles = Get-ChildItem $langProfilePath
    foreach ($profile in $profiles) {
        $guid = $profile.PSChildName
        Write-Host "Profile GUID: $guid" -ForegroundColor Yellow
        
        $props = Get-ItemProperty $profile.PSPath
        Write-Host "  Description: $($props.Description)" -ForegroundColor White
        Write-Host "  IconFile: $($props.IconFile)" -ForegroundColor Gray
        Write-Host "  IconIndex: $($props.IconIndex)" -ForegroundColor Gray
        
        if ($props.PSObject.Properties.Name -contains "Enable") {
            Write-Host "  Enable: $($props.Enable)" -ForegroundColor White
        } else {
            Write-Host "  Enable: (not set - this might be the problem!)" -ForegroundColor Red
        }
    }
} else {
    Write-Host "No English US profile found!" -ForegroundColor Red
}
