#!/bin/bash
# buttre IBus - Installation Script
# Run with sudo

set -e

echo "🐧 Installing buttre IBus Engine..."

# Configuration
PREFIX="${PREFIX:-/usr}"
BINDIR="$PREFIX/bin"
COMPONENTDIR="$PREFIX/share/ibus/component"

# Check if running as root
if [ "$EUID" -ne 0 ]; then
    echo "❌ Please run as root (sudo ./install.sh)"
    exit 1
fi

# Build release binary
echo "📦 Building release binary..."
cargo build --release -p buttre-platform

# Create directories
echo "📁 Creating directories..."
mkdir -p "$BINDIR"
mkdir -p "$COMPONENTDIR"

# Install binary (component XML expects /usr/bin/buttre)
echo "📥 Installing binary..."
install -m 755 target/release/buttre "$BINDIR/"

# Install component XML
echo "📄 Installing component..."
install -m 644 installers/linux/buttre.xml "$COMPONENTDIR/buttre.xml"

# Restart IBus
echo "🔄 Restarting IBus..."
if command -v ibus-daemon &> /dev/null; then
    killall ibus-daemon 2>/dev/null || true
    sleep 1
    ibus-daemon -drx &
fi

echo "✅ Installation complete!"
echo ""
echo "📝 Next steps:"
echo "1. Open IBus Preferences: ibus-setup"
echo "2. Go to 'Input Method' tab"
echo "3. Click 'Add' button"
echo "4. Select 'Vietnamese' → 'buttre Vietnamese (Telex)'"
echo "5. Test in any application (gedit, Firefox, etc.)"
echo ""
echo "🔑 Switch input method: Super+Space (or configured hotkey)"
