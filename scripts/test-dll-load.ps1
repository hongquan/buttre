# Test if DLL can be loaded and get exports
# Usage: .\test-dll-load.ps1 [path\to\buttre_platform.dll]
param([string]$DllOverride = "")
$repoRoot = Resolve-Path "$PSScriptRoot\.."
if ($DllOverride) {
    $dllPath = $DllOverride
} elseif (Test-Path (Join-Path $repoRoot "target\debug\buttre_platform.dll")) {
    $dllPath = Join-Path $repoRoot "target\debug\buttre_platform.dll"
} else {
    $dllPath = Join-Path $repoRoot "target\release\buttre_platform.dll"
}

Write-Host "Testing DLL load..." -ForegroundColor Cyan
Write-Host "DLL: $dllPath" -ForegroundColor Gray
Write-Host ""

# Test 1: Check if file exists
if (Test-Path $dllPath) {
    Write-Host "1. File exists: OK" -ForegroundColor Green
    $dll = Get-Item $dllPath
    Write-Host "   Size: $($dll.Length) bytes" -ForegroundColor Gray
} else {
    Write-Host "1. File NOT found!" -ForegroundColor Red
    exit 1
}

# Test 2: Try to load with LoadLibrary
Write-Host "2. Testing LoadLibrary..." -ForegroundColor Yellow

$code = @"
using System;
using System.Runtime.InteropServices;

public class DllTest {
    [DllImport("kernel32.dll", CharSet = CharSet.Auto, SetLastError = true)]
    public static extern IntPtr LoadLibrary(string lpFileName);
    
    [DllImport("kernel32.dll", CharSet = CharSet.Ansi, SetLastError = true)]
    public static extern IntPtr GetProcAddress(IntPtr hModule, string lpProcName);
    
    [DllImport("kernel32.dll", SetLastError = true)]
    public static extern bool FreeLibrary(IntPtr hModule);
}
"@

Add-Type -TypeDefinition $code

$handle = [DllTest]::LoadLibrary($dllPath)

if ($handle -eq [IntPtr]::Zero) {
    $err = [Runtime.InteropServices.Marshal]::GetLastWin32Error()
    Write-Host "   LoadLibrary FAILED! Error code: $err" -ForegroundColor Red
    
    # Common error codes
    switch ($err) {
        126 { Write-Host "   ERROR 126: Module not found (missing dependencies)" -ForegroundColor Yellow }
        127 { Write-Host "   ERROR 127: Procedure not found" -ForegroundColor Yellow }
        193 { Write-Host "   ERROR 193: Not a valid Win32 application" -ForegroundColor Yellow }
        default { Write-Host "   See: https://learn.microsoft.com/en-us/windows/win32/debug/system-error-codes" -ForegroundColor Gray }
    }
    exit 1
}

Write-Host "   LoadLibrary: OK (handle: $handle)" -ForegroundColor Green

# Test 3: Check for DllRegisterServer export
Write-Host "3. Checking DllRegisterServer export..." -ForegroundColor Yellow
$proc = [DllTest]::GetProcAddress($handle, "DllRegisterServer")

if ($proc -eq [IntPtr]::Zero) {
    Write-Host "   DllRegisterServer NOT found in exports!" -ForegroundColor Red
} else {
    Write-Host "   DllRegisterServer found at: $proc" -ForegroundColor Green
}

# Cleanup
[DllTest]::FreeLibrary($handle) | Out-Null

Write-Host ""
Write-Host "If LoadLibrary failed with error 126, check dependencies with:" -ForegroundColor Cyan
Write-Host "  Dependencies.exe (Download from https://github.com/lucasg/Dependencies)" -ForegroundColor Gray
