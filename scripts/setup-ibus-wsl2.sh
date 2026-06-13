#!/bin/bash
# IBus Development Environment Setup Script
# Run this in WSL2 Ubuntu

set -e  # Exit on error

echo "================================================"
echo "  buttre IBus Development Environment Setup"
echo "================================================"
echo ""

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Step 1: Update system
echo -e "${YELLOW}[1/6] Updating system packages...${NC}"
sudo apt update
sudo apt upgrade -y

# Step 2: Install IBus and development tools
echo -e "${YELLOW}[2/6] Installing IBus and development tools...${NC}"
sudo apt install -y \
    ibus \
    ibus-dev \
    libibus-1.0-dev \
    pkg-config \
    build-essential \
    git \
    libglib2.0-dev \
    libdbus-1-dev \
    dbus-x11

# Step 3: Check if Rust is installed
echo -e "${YELLOW}[3/6] Checking Rust installation...${NC}"
if ! command -v cargo &> /dev/null; then
    echo "Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source $HOME/.cargo/env
else
    echo -e "${GREEN}✓ Rust already installed${NC}"
fi

# Step 4: Verify installations
echo -e "${YELLOW}[4/6] Verifying installations...${NC}"
echo "Rust version: $(rustc --version)"
echo "Cargo version: $(cargo --version)"
echo "IBus version: $(ibus version)"
echo "pkg-config version: $(pkg-config --version)"

# Step 5: Setup D-Bus session
echo -e "${YELLOW}[5/6] Setting up D-Bus session...${NC}"
if [ -z "$DBUS_SESSION_BUS_ADDRESS" ]; then
    echo "Starting D-Bus session..."
    eval $(dbus-launch --sh-syntax)
    echo "export DBUS_SESSION_BUS_ADDRESS='$DBUS_SESSION_BUS_ADDRESS'" >> ~/.bashrc
fi

# Step 6: Start IBus daemon
echo -e "${YELLOW}[6/6] Starting IBus daemon...${NC}"
# Kill any existing IBus processes
killall ibus-daemon 2>/dev/null || true
# Start IBus daemon
ibus-daemon --xim --verbose --replace &
sleep 2

# Verify IBus is running
if pgrep -x "ibus-daemon" > /dev/null; then
    echo -e "${GREEN}✓ IBus daemon is running${NC}"
else
    echo -e "${YELLOW}⚠ IBus daemon may not be running. Try: ibus-daemon --xim --verbose &${NC}"
fi

echo ""
echo -e "${GREEN}================================================${NC}"
echo -e "${GREEN}  Setup Complete!${NC}"
echo -e "${GREEN}================================================${NC}"
echo ""
echo "Next steps:"
echo "1. Navigate to the buttre project (your repo checkout), e.g.:"
echo "   cd /mnt/d/buttre"
echo ""
echo "2. Build the platform crate:"
echo "   cargo build --package buttre-platform"
echo ""
echo "3. Run tests:"
echo "   cargo test --package buttre-platform"
echo ""
echo "4. Check IBus daemon:"
echo "   ps aux | grep ibus"
echo ""
echo "5. Monitor D-Bus messages:"
echo "   dbus-monitor --session"
echo ""
