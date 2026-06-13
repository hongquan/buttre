# buttre Linting and Security Check Script
# Usage: .\scripts\lint.ps1 [command]
# Commands: clippy, audit, all, fix

param(
    [Parameter(Position=0)]
    [ValidateSet("clippy", "audit", "all", "fix", "check")]
    [string]$Command = "all"
)

$ErrorActionPreference = "Stop"

function Write-Header {
    param([string]$Text)
    Write-Host ""
    Write-Host "============================================" -ForegroundColor Cyan
    Write-Host " $Text" -ForegroundColor Cyan
    Write-Host "============================================" -ForegroundColor Cyan
}

function Run-Clippy {
    Write-Header "Running Clippy"
    cargo clippy --workspace --all-targets --all-features -- -D warnings
    if ($LASTEXITCODE -eq 0) {
        Write-Host "Clippy passed!" -ForegroundColor Green
    } else {
        Write-Host "Clippy found issues!" -ForegroundColor Red
        exit 1
    }
}

function Run-ClippyFix {
    Write-Header "Running Clippy with Auto-Fix"
    cargo clippy --workspace --all-targets --all-features --fix --allow-dirty --allow-staged
    if ($LASTEXITCODE -eq 0) {
        Write-Host "Clippy auto-fix complete!" -ForegroundColor Green
    } else {
        Write-Host "Clippy fix encountered issues!" -ForegroundColor Red
        exit 1
    }
}

function Run-Audit {
    Write-Header "Running Security Audit"
    
    # Check if cargo-audit is installed
    $auditInstalled = cargo audit --version 2>$null
    if (-not $auditInstalled) {
        Write-Host "Installing cargo-audit..." -ForegroundColor Yellow
        cargo install cargo-audit
    }
    
    cargo audit
    if ($LASTEXITCODE -eq 0) {
        Write-Host "Security audit passed!" -ForegroundColor Green
    } else {
        Write-Host "Security vulnerabilities found!" -ForegroundColor Red
        exit 1
    }
}

function Run-Check {
    Write-Header "Running Cargo Check"
    cargo check --workspace --all-targets --all-features
    if ($LASTEXITCODE -eq 0) {
        Write-Host "Cargo check passed!" -ForegroundColor Green
    } else {
        Write-Host "Cargo check failed!" -ForegroundColor Red
        exit 1
    }
}

switch ($Command) {
    "clippy" { Run-Clippy }
    "audit" { Run-Audit }
    "fix" { Run-ClippyFix }
    "check" { Run-Check }
    "all" {
        Run-Check
        Run-Clippy
        Run-Audit
        Write-Host ""
        Write-Host "All checks passed!" -ForegroundColor Green
    }
}
