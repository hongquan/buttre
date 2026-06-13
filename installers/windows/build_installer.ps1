#!/usr/bin/env pwsh
# Build the buttre MSI installer via cargo-wix.
# Usage: ./build_installer.ps1 [-Version 0.6.3-alpha]
param(
    [string]$Version = ""
)

$ErrorActionPreference = "Stop"
$repoRoot = Resolve-Path "$PSScriptRoot\..\.."

Push-Location $repoRoot
try {
    Write-Host "==> Building buttre-platform release..."
    cargo build -p buttre-platform --release

    $targetDir = Join-Path $repoRoot "target\release"
    $nomDb     = Join-Path $targetDir "buttre_nom.db"

    # cargo-wix only forwards preprocessor defines to candle via -C/--compiler-arg;
    # args after `--` are NOT passed through.
    $wixArgs = @()
    if (Test-Path $nomDb) {
        Write-Host "==> Nom DB found, including in MSI"
        $wixArgs += @("-C", "-dIncludeNomDb=1")
    } else {
        Write-Host "==> Nom DB not found, MSI will ship without it"
    }

    if ($Version -eq "") {
        # cargo pkgid returns something like path+file:///...#0.6.3-alpha
        $Version = (cargo pkgid -p buttre-platform) -replace '.*#', ''
    }

    # --install-version takes SemVer 3-part (strips pre-release suffix).
    # cargo-wix automatically defines $(var.Version) = this value for product.wxs.
    # Do NOT also pass -C "-dVersion=..." — candle rejects duplicate variable declarations.
    $semVer = $Version -replace '-.*$', ''  # e.g. "0.7.0"

    # cargo-wix resolves `include` paths relative to cwd, so run from the crate directory
    # where include = "../../installers/windows/product.wxs" resolves correctly.
    $crateDir  = Join-Path $repoRoot "crates\buttre-platform"
    $outputAbs = Join-Path $repoRoot "target\wix\buttre-$Version-x86_64.msi"
    New-Item -ItemType Directory -Force (Join-Path $repoRoot "target\wix") | Out-Null

    Push-Location $crateDir
    Write-Host "==> Building MSI v$Version (install-version: $semVer)..."
    cargo wix `
        --package buttre-platform `
        --nocapture `
        --output $outputAbs `
        --install-version $semVer `
        @wixArgs
    Pop-Location

    $msiPath = "target\wix\buttre-$Version-x86_64.msi"
    Write-Host ""
    Write-Host "==> MSI: $msiPath"
    Get-Item $msiPath | Select-Object Name, @{N='Size';E={"{0:N0} KB" -f ($_.Length / 1KB)}}
}
finally {
    Pop-Location
}
