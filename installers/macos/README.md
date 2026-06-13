# macOS Installer Build Guide

## Prerequisites

- macOS machine (10.13+)
- Xcode Command Line Tools installed
- Rust toolchain installed

## Build Process

### 1. Build the installer package

```bash
cd installers/macos
chmod +x build_pkg.sh
./build_pkg.sh
```

This will create:
- `build/buttre-0.6.0.pkg` - Component package
- `build/buttre-Installer-0.6.0.pkg` - Distribution package (recommended)

### 2. Test installation locally

```bash
sudo installer -pkg build/buttre-Installer-0.6.0.pkg -target /
```

### 3. Verify installation

```bash
ls -la "/Library/Input Methods/buttre.app"
```

### 4. Activate Input Method

1. Log out and log back in (or restart)
2. Open **System Preferences → Keyboard → Input Sources**
3. Click **+** button
4. Select **Vietnamese → buttre**

## Code Signing (Optional but Recommended)

For distribution outside the App Store, you need to sign and notarize:

### Sign the package

```bash
productsign --sign "Developer ID Installer: Your Name" \
  build/buttre-Installer-0.6.0.pkg \
  build/buttre-Installer-0.6.0-Signed.pkg
```

### Notarize with Apple

```bash
# Upload for notarization
xcrun notarytool submit build/buttre-Installer-0.6.0-Signed.pkg \
  --apple-id "your@email.com" \
  --team-id "TEAM_ID" \
  --password "app-specific-password" \
  --wait

# Staple the notarization ticket
xcrun stapler staple build/buttre-Installer-0.6.0-Signed.pkg
```

## Directory Structure

```
/Library/Input Methods/buttre.app/
├── Contents/
│   ├── Info.plist
│   ├── MacOS/
│   │   └── buttre (binary)
│   └── Resources/
│       ├── buttre_nom.db
│       └── custom/
│           ├── README.md
│           └── taynguyen.toml
```

## Troubleshooting

### Input Method not showing up

1. Check if app is installed:
   ```bash
   ls -la "/Library/Input Methods/buttre.app"
   ```

2. Re-register with system:
   ```bash
   /System/Library/Frameworks/Carbon.framework/Versions/A/Support/lsregister \
     -f "/Library/Input Methods/buttre.app"
   ```

3. Restart your Mac

### Permission issues

```bash
sudo chmod -R 755 "/Library/Input Methods/buttre.app"
sudo chown -R root:wheel "/Library/Input Methods/buttre.app"
```

## Uninstallation

```bash
sudo rm -rf "/Library/Input Methods/buttre.app"
# Then remove from Input Sources in System Preferences
```

## Distribution

For public distribution, you **must**:
1. Sign with Developer ID Installer certificate
2. Notarize with Apple
3. Staple the notarization ticket

Without these steps, users will see security warnings on macOS 10.15+.
