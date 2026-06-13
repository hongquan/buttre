# buttre TSF - Quick Install Script
# Run as Administrator

param(
    [switch]$Uninstall
)

$ErrorActionPreference = "Stop"

$DllSource = Join-Path $PSScriptRoot "..\target\release\buttre_platform.dll"
$InstallDir = "$env:ProgramFiles\buttre"
$DllDest = Join-Path $InstallDir "buttre_platform.dll"

function Test-Administrator {
    $currentUser = [Security.Principal.WindowsIdentity]::GetCurrent()
    $principal = New-Object Security.Principal.WindowsPrincipal($currentUser)
    return $principal.IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)
}

if (-not (Test-Administrator)) {
    Write-Error "This script must be run as Administrator!"
    Write-Host "Right-click PowerShell and select 'Run as Administrator'"
    exit 1
}

if ($Uninstall) {
    Write-Host "🗑️  Uninstalling buttre TSF..." -ForegroundColor Yellow
    
    # Unregister DLL
    if (Test-Path $DllDest) {
        Write-Host "Unregistering DLL..."
        regsvr32 /u /s $DllDest
        
        Write-Host "Removing files..."
        Remove-Item $InstallDir -Recurse -Force -ErrorAction SilentlyContinue
    }
    
    Write-Host "✅ Uninstall complete!" -ForegroundColor Green
    Write-Host ""
    Write-Host "Please remove buttre from Language Settings manually:"
    Write-Host "Settings → Time & Language → Language → Vietnamese → Options → Remove buttre"
    
} else {
    Write-Host "📦 Installing buttre TSF..." -ForegroundColor Cyan
    
    # Check if DLL exists
    if (-not (Test-Path $DllSource)) {
        Write-Error "DLL not found at: $DllSource"
        Write-Host "Please run: cargo build --release -p buttre-platform"
        exit 1
    }
    
    # Create install directory
    Write-Host "Creating installation directory..."
    New-Item -ItemType Directory -Force -Path $InstallDir | Out-Null
    
    # Copy DLL
    Write-Host "Copying DLL..."
    Copy-Item $DllSource $DllDest -Force
    
    # Register DLL
    Write-Host "Registering COM server..."
    $result = regsvr32 /s $DllDest
    
    if ($LASTEXITCODE -eq 0) {
        Write-Host "✅ Installation successful!" -ForegroundColor Green
        Write-Host ""
        Write-Host "📝 Next Steps:" -ForegroundColor Yellow
        Write-Host "1. Open Settings → Time & Language → Language"
        Write-Host "2. Add Vietnamese language (if not already added)"
        Write-Host "3. Click Vietnamese → Options → Add a keyboard"
        Write-Host "4. Select 'buttre' from the list"
        Write-Host "5. Switch to buttre: Press Windows + Space"
        Write-Host ""
        Write-Host "📖 See docs/MANUAL_TESTING_GUIDE.md for detailed testing instructions"
    } else {
        Write-Error "Registration failed! Check Event Viewer for details."
        exit 1
    }
}
