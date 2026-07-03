<#
.SYNOPSIS
    Purity-invariant deny-rules (event-sourcing-completion Phase 8).

.DESCRIPTION
    Enforces two of AGENTS.md's "Event-sourcing purity (INVARIANT)" rules by
    ripgrep-style pattern matching over the workspace's own source tree — a
    companion to the frozen-bool-count test in
    crates/buttre-engine/tests/purity_audit.rs (which catches a NEW bool
    FIELD on TypingContext specifically; this script catches misuse of the
    one remaining one-way field that survived the Phase 8 audit,
    `temp_english_mode`, plus any future sibling one-way `_mode: bool` field
    anywhere in the three source crates).

    Rule 1 - `temp_english_mode` may only be ASSIGNED at its blessed
    derivation/reset sites: `context.rs` (struct init + `Self::clear()`),
    `compose_stage.rs` (the evidence-based un-latch clear, the run-on-cap
    latch, and the normal-path result copy), and `stage2_gatekeeper.rs` (the
    separator/reset passthrough). A NEW file assigning it is exactly the
    class of bug the purity invariant exists to prevent: a platform-layer or
    other-stage special case bypassing the pipeline's own re-derivation
    logic (history: "pre-gate heuristic guards" per AGENTS.md).

    Rule 2 - no new `<name>_mode: bool` field beyond the frozen baseline of
    5: `native_script_mode` (declared 3x - `pipeline/config.rs`,
    `keyboard/config.rs`, and copied into `stage2_gatekeeper.rs`'s own
    struct) and `strict_mode` (`stage3_validation.rs`) are legitimate STATIC
    CONFIGURATION flags - set once from config at construction, never
    mutated at runtime - not one-way decisions over typing history.
    `temp_english_mode` (`context.rs`) is the one derived-every-keystroke
    exception documented in purity_audit.rs. A new one-way RUNTIME `_mode`
    bool is a purity red flag: justify it in purity_audit.rs's
    field-justification table and bump BOTH that test's count and this
    script's baseline in the same commit.

    Implemented with PowerShell's own Select-String (available on every
    supported dev/CI machine - see .github/workflows/ci.yml's windows-latest
    "Quick Checks" job) rather than depending on an external ripgrep binary
    install; the pattern rules themselves are the deny-rule contract this
    exists to enforce; this deliberately follows the same deny-rule design
    as a checked-in ripgrep script would.

.EXAMPLE
    pwsh ./scripts/check-purity.ps1
#>

$ErrorActionPreference = "Stop"
$RepoRoot = Split-Path -Parent $PSScriptRoot
$SrcRoots = @("crates/buttre-engine/src", "crates/buttre-core/src", "crates/buttre-platform/src") |
    ForEach-Object { Join-Path $RepoRoot $_ }

$failed = $false

function Get-RustFiles {
    param([string]$Root)
    Get-ChildItem -Path $Root -Recurse -Filter "*.rs" -File |
        Where-Object { $_.Name -notmatch "(?i)test" }
}

# ── Rule 1: temp_english_mode assignment allowlist ─────────────────────────
$AllowedAssignmentFiles = @(
    "context.rs",          # struct init + Self::clear()
    "compose_stage.rs",    # evidence-based un-latch, run-on-cap latch, normal-path copy
    "stage2_gatekeeper.rs" # separator/reset passthrough
)

$assignmentPattern = '\.temp_english_mode\s*='
$violations = @()
foreach ($root in $SrcRoots) {
    if (-not (Test-Path $root)) { continue }
    foreach ($file in Get-RustFiles -Root $root) {
        $found = Select-String -Path $file.FullName -Pattern $assignmentPattern
        if ($found -and ($AllowedAssignmentFiles -notcontains $file.Name)) {
            $violations += $found | ForEach-Object { "$($file.FullName):$($_.LineNumber): $($_.Line.Trim())" }
        }
    }
}
if ($violations.Count -gt 0) {
    Write-Host "PURITY VIOLATION (Rule 1): 'temp_english_mode' assigned outside its blessed derivation sites:" -ForegroundColor Red
    $violations | ForEach-Object { Write-Host "  $_" -ForegroundColor Red }
    Write-Host "  Allowed files: $($AllowedAssignmentFiles -join ', ')" -ForegroundColor Yellow
    $failed = $true
} else {
    Write-Host "[ok] Rule 1: temp_english_mode assignments confined to blessed sites" -ForegroundColor Green
}

# ── Rule 2: frozen `_mode: bool` baseline ───────────────────────────────────
$ExpectedModeBoolCount = 5
$modePattern = '\w+_mode\s*:\s*bool'
$modeHits = @()
foreach ($root in $SrcRoots) {
    if (-not (Test-Path $root)) { continue }
    foreach ($file in Get-RustFiles -Root $root) {
        $found = Select-String -Path $file.FullName -Pattern $modePattern
        if ($found) {
            $modeHits += $found | ForEach-Object { "$($file.FullName):$($_.LineNumber): $($_.Line.Trim())" }
        }
    }
}
if ($modeHits.Count -ne $ExpectedModeBoolCount) {
    Write-Host "PURITY VIOLATION (Rule 2): found $($modeHits.Count) '_mode: bool' field(s), expected ${ExpectedModeBoolCount}:" -ForegroundColor Red
    $modeHits | ForEach-Object { Write-Host "  $_" -ForegroundColor Red }
    Write-Host "  If this is a legitimate new STATIC CONFIG field, bump the baseline HERE." -ForegroundColor Yellow
    Write-Host "  If it is a new RUNTIME one-way flag, justify it in purity_audit.rs instead." -ForegroundColor Yellow
    $failed = $true
} else {
    Write-Host "[ok] Rule 2: '_mode: bool' field count matches the frozen baseline ($ExpectedModeBoolCount)" -ForegroundColor Green
}

if ($failed) {
    Write-Host ""
    Write-Host "Purity check FAILED - see AGENTS.md's 'Event-sourcing purity (INVARIANT)' rule." -ForegroundColor Red
    exit 1
}

Write-Host ""
Write-Host "Purity check passed." -ForegroundColor Green
exit 0
