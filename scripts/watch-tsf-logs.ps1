# Watch TSF debug logs
# Run this BEFORE typing in Notepad

Write-Host "=== Watching buttre TSF Debug Logs ===" -ForegroundColor Cyan
Write-Host "Press Ctrl+C to stop" -ForegroundColor Gray
Write-Host ""

# Monitor Event Log
$lastEvent = (Get-EventLog -LogName Application -Newest 1 -ErrorAction SilentlyContinue).Index

while ($true) {
    $events = Get-EventLog -LogName Application -After (Get-Date).AddSeconds(-2) -ErrorAction SilentlyContinue |
              Where-Object { $_.Message -like "*buttre*" -or $_.Message -like "*buttre*" }
    
    foreach ($event in $events) {
        Write-Host "[$($event.TimeGenerated)] $($event.Message)" -ForegroundColor Yellow
    }
    
    Start-Sleep -Milliseconds 500
}
