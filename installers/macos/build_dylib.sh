#!/usr/bin/env bash
set -euo pipefail

VERSION="${1:-}"
if [ -z "$VERSION" ]; then
    echo "Usage: $0 <version>"
    echo "Example: $0 0.6.3-alpha"
    exit 1
fi

REPO_ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$REPO_ROOT"

echo "==> Adding cross-compile targets..."
rustup target add aarch64-apple-darwin
rustup target add x86_64-apple-darwin

echo "==> Building arm64 dylib..."
cargo build -p buttre-platform --release --target aarch64-apple-darwin

echo "==> Building x86_64 dylib..."
cargo build -p buttre-platform --release --target x86_64-apple-darwin

STAGING="target/macos/buttre-${VERSION}-macos-dylib"
rm -rf "$STAGING"
mkdir -p "$STAGING/keyboards"

echo "==> Creating universal dylib via lipo..."
lipo -create \
    "target/aarch64-apple-darwin/release/libbuttre_platform.dylib" \
    "target/x86_64-apple-darwin/release/libbuttre_platform.dylib" \
    -output "$STAGING/libbuttre_platform.dylib"

echo "==> Verifying universal binary..."
lipo -info "$STAGING/libbuttre_platform.dylib"

echo "==> Copying keyboards..."
cp keyboards/*.toml "$STAGING/keyboards/"

echo "==> Copying README..."
cp installers/macos/ARTIFACT_README.md "$STAGING/README.md"

echo "==> Zipping..."
( cd target/macos && zip -r "buttre-${VERSION}-macos-dylib.zip" "buttre-${VERSION}-macos-dylib" )

echo ""
echo "Artifact: target/macos/buttre-${VERSION}-macos-dylib.zip"
ls -lh "target/macos/buttre-${VERSION}-macos-dylib.zip"
