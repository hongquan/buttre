@echo off
REM Local CI checks - Run before pushing to catch issues early

echo.
echo [92m===========================================
echo    Running Local CI Checks
echo ===========================================[0m
echo.

REM 1. Format check
echo [93m[1/7] Checking formatting...[0m
cargo fmt --all -- --check
if errorlevel 1 (
    echo [91mFormatting check failed. Run: cargo fmt --all[0m
    exit /b 1
)
echo [92m  OK Formatting check passed[0m
echo.

REM 2. Clippy
echo [93m[2/7] Running clippy...[0m
cargo clippy --workspace --all-targets --all-features -- -D warnings
if errorlevel 1 (
    echo [91mClippy check failed. Fix warnings above.[0m
    exit /b 1
)
echo [92m  OK Clippy check passed[0m
echo.

REM 3. Build
echo [93m[3/7] Building workspace...[0m
cargo build --workspace --all-features
if errorlevel 1 (
    echo [91mBuild failed[0m
    exit /b 1
)
echo [92m  OK Build successful[0m
echo.

REM 4. Unit tests
echo [93m[4/7] Running unit tests...[0m
cargo test --workspace --all-features
if errorlevel 1 (
    echo [91mUnit tests failed[0m
    exit /b 1
)
echo [92m  OK Unit tests passed[0m
echo.

REM 5. Doc tests
echo [93m[5/7] Running doc tests...[0m
cargo test --workspace --doc
if errorlevel 1 (
    echo [91mDoc tests failed[0m
    exit /b 1
)
echo [92m  OK Doc tests passed[0m
echo.

REM 6. Security audit (optional)
echo [93m[6/7] Running security audit...[0m
where cargo-audit >nul 2>&1
if %errorlevel% equ 0 (
    cargo audit
    if errorlevel 1 (
        echo [93m  WARNING Security issues found ^(review above^)[0m
    ) else (
        echo [92m  OK No security issues found[0m
    )
) else (
    echo [93m  SKIP cargo-audit not installed. Run: cargo install cargo-audit[0m
)
echo.

REM 7. Dependency check (optional)
echo [93m[7/7] Checking dependencies...[0m
where cargo-deny >nul 2>&1
if %errorlevel% equ 0 (
    cargo deny check
    if errorlevel 1 (
        echo [93m  WARNING Dependency issues found ^(review above^)[0m
    ) else (
        echo [92m  OK Dependency check passed[0m
    )
) else (
    echo [93m  SKIP cargo-deny not installed. Run: cargo install cargo-deny[0m
)
echo.

REM Summary
echo [92m===========================================
echo    All mandatory CI checks passed!
echo ===========================================[0m
echo.
echo [93mNext steps:[0m
echo   1. Commit: git add . ^&^& git commit -m "your message"
echo   2. Push: git push
echo   3. View CI: https://github.com/YOUR_USERNAME/buttre/actions
echo.
