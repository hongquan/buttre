# buttre TSF Auto-Install Script
# Automatically builds and installs TSF for testing
# Run as Administrator

param(
    [switch]$Uninstall,
    [switch]$SkipBuild
)

$ErrorActionPreference = "Stop"

# Colors
function Write-Info { param($msg) Write-Host "[INFO] $msg" -ForegroundColor Cyan }
function Write-Success { param($msg) Write-Host "[OK] $msg" -ForegroundColor Green }
function Write-Warning { param($msg) Write-Host "[WARN] $msg" -ForegroundColor Yellow }
function Write-Error-Custom { param($msg) Write-Host "[ERROR] $msg" -ForegroundColor Red }

# Check Administrator
function Test-Administrator {
    $currentUser = [Security.Principal.WindowsIdentity]::GetCurrent()
    $principal = New-Object Security.Principal.WindowsPrincipal($currentUser)
    return $principal.IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)
}

if (-not (Test-Administrator)) {
    Write-Error-Custom "This script must be run as Administrator!"
    Write-Host ""
    Write-Host "Please:" -ForegroundColor Yellow
    Write-Host "1. Right-click PowerShell" -ForegroundColor White
    Write-Host "2. Select 'Run as Administrator'" -ForegroundColor White
    Write-Host "3. Run this script again" -ForegroundColor White
    exit 1
}

# Paths
$ProjectRoot = $PSScriptRoot
$DllSource = Join-Path $ProjectRoot "target\release\buttre_platform.dll"
$InstallDir = "$env:ProgramFiles\buttre"
$DllDest = Join-Path $InstallDir "buttre_platform.dll"

if ($Uninstall) {
    Write-Warning "Uninstalling buttre TSF..."
    Write-Host ""
    
    # Unregister DLL
    if (Test-Path $DllDest) {
        Write-Info "Unregistering COM server..."
        try {
            regsvr32 /u /s $DllDest
            Write-Success "COM server unregistered"
        } catch {
            Write-Warning "Failed to unregister (may not have been registered)"
        }
        
        Write-Info "Removing files..."
        Remove-Item $InstallDir -Recurse -Force -ErrorAction SilentlyContinue
        Write-Success "Files removed"
    } else {
        Write-Warning "Installation not found at: $InstallDir"
    }
    
    Write-Host ""
    Write-Success "Uninstall complete!"
    Write-Host ""
    Write-Warning "Manual step required:"
    Write-Host "1. Open Settings > Time & Language > Language" -ForegroundColor White
    Write-Host "2. Click Vietnamese > Options" -ForegroundColor White
    Write-Host "3. Find 'buttre' and click Remove" -ForegroundColor White
    Write-Host ""
    
} else {
    Write-Info "buttre TSF Auto-Installer"
    Write-Host ""
    
    # Build DLL
    if (-not $SkipBuild) {
        Write-Info "Building buttre TSF DLL (Release mode)..."
        Write-Host ""
        
        Push-Location $ProjectRoot
        try {
            $buildOutput = cargo build --package buttre-platform --lib --release 2>&1
            if ($LASTEXITCODE -ne 0) {
                Write-Error-Custom "Build failed!"
                Write-Host $buildOutput
                exit 1
            }
            Write-Success "Build complete"
        } finally {
            Pop-Location
        }
        Write-Host ""
    }
    
    # Check if DLL exists
    if (-not (Test-Path $DllSource)) {
        Write-Error-Custom "DLL not found at: $DllSource"
        Write-Host ""
        Write-Host "Please ensure the build succeeded" -ForegroundColor Yellow
        exit 1
    }
    
    $dllSize = (Get-Item $DllSource).Length / 1MB
    Write-Info "Found DLL: $([math]::Round($dllSize, 2)) MB"
    Write-Host ""
    
    # Create install directory
    Write-Info "Creating installation directory..."
    New-Item -ItemType Directory -Force -Path $InstallDir | Out-Null
    Write-Success "Directory created: $InstallDir"
    Write-Host ""
    
    # Copy DLL
    Write-Info "Installing DLL..."
    Copy-Item $DllSource $DllDest -Force
    Write-Success "DLL installed to: $DllDest"
    Write-Host ""
    
    # Register COM server
    Write-Info "Registering COM server..."
    $regOutput = regsvr32 /s $DllDest 2>&1
    
    if ($LASTEXITCODE -eq 0) {
        Write-Success "COM server registered successfully!"
        Write-Host ""
        Write-Host "=========================================" -ForegroundColor Cyan
        Write-Success "Installation Complete!"
        Write-Host "=========================================" -ForegroundColor Cyan
        Write-Host ""
        
        Write-Host "Next Steps to Enable buttre TSF:" -ForegroundColor Yellow
        Write-Host ""
        Write-Host "Step 1: Add Vietnamese Language (if not already added)" -ForegroundColor Cyan
        Write-Host "   - Open Settings > Time & Language > Language" -ForegroundColor White
        Write-Host "   - Click 'Add a language'" -ForegroundColor White
        Write-Host "   - Search for 'Vietnamese' and add it" -ForegroundColor White
        Write-Host ""
        
        Write-Host "Step 2: Add buttre Input Method" -ForegroundColor Cyan
        Write-Host "   - Click Vietnamese > Options" -ForegroundColor White
        Write-Host "   - Click 'Add a keyboard'" -ForegroundColor White
        Write-Host "   - Find and select 'buttre Vietnamese Input'" -ForegroundColor White
        Write-Host ""
        
        Write-Host "Step 3: Switch to buttre" -ForegroundColor Cyan
        Write-Host "   - Press Windows + Space to switch input methods" -ForegroundColor White
        Write-Host "   - Or click the language indicator in the taskbar" -ForegroundColor White
        Write-Host ""
        
        Write-Host "Step 4: Test It" -ForegroundColor Cyan
        Write-Host "   - Open Notepad or any text editor" -ForegroundColor White
        Write-Host "   - Try typing: viet, hoa, thuong" -ForegroundColor White
        Write-Host "   - Press 'f' for tone marks (Telex method)" -ForegroundColor White
        Write-Host ""
        
        Write-Host "=========================================" -ForegroundColor Cyan
        Write-Host ""
        Write-Info "Troubleshooting:"
        Write-Host "  - If buttre doesn't appear: Restart your computer" -ForegroundColor Gray
        Write-Host "  - Check logs at: $env:TEMP\buttre-tsf.log" -ForegroundColor Gray
        Write-Host "  - Uninstall: .\install-tsf-auto.ps1 -Uninstall" -ForegroundColor Gray
        Write-Host ""
        
    } else {
        Write-Error-Custom "COM registration failed!"
        Write-Host ""
        Write-Warning "This could be because:"
        Write-Host "  - Missing dependencies" -ForegroundColor Gray
        Write-Host "  - DLL is corrupted" -ForegroundColor Gray
        Write-Host "  - Windows is blocking the DLL" -ForegroundColor Gray
        Write-Host ""
        Write-Host "Check Event Viewer for details:" -ForegroundColor Yellow
        Write-Host "  Event Viewer > Windows Logs > Application" -ForegroundColor White
        exit 1
    }
}
