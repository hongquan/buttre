#!/bin/bash
# Local CI checks - Run before pushing to catch issues early
set -e

GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

echo -e "${GREEN}🔍 Running local CI checks...${NC}\n"

# 1. Format check
echo -e "${YELLOW}📝 Checking formatting...${NC}"
if cargo fmt --all -- --check; then
    echo -e "${GREEN}✓ Formatting check passed${NC}\n"
else
    echo -e "${RED}✗ Formatting check failed. Run: cargo fmt --all${NC}\n"
    exit 1
fi

# 2. Clippy
echo -e "${YELLOW}🔎 Running clippy...${NC}"
if cargo clippy --workspace --all-targets --all-features -- -D warnings; then
    echo -e "${GREEN}✓ Clippy check passed${NC}\n"
else
    echo -e "${RED}✗ Clippy check failed. Fix warnings above.${NC}\n"
    exit 1
fi

# 3. Build
echo -e "${YELLOW}🏗️  Building workspace...${NC}"
if cargo build --workspace --all-features; then
    echo -e "${GREEN}✓ Build successful${NC}\n"
else
    echo -e "${RED}✗ Build failed${NC}\n"
    exit 1
fi

# 4. Unit tests
echo -e "${YELLOW}🧪 Running unit tests...${NC}"
if cargo test --workspace --all-features; then
    echo -e "${GREEN}✓ Unit tests passed${NC}\n"
else
    echo -e "${RED}✗ Unit tests failed${NC}\n"
    exit 1
fi

# 5. Doc tests
echo -e "${YELLOW}📚 Running doc tests...${NC}"
if cargo test --workspace --doc; then
    echo -e "${GREEN}✓ Doc tests passed${NC}\n"
else
    echo -e "${RED}✗ Doc tests failed${NC}\n"
    exit 1
fi

# 6. Security audit (optional, warn only)
echo -e "${YELLOW}🔒 Running security audit...${NC}"
if command -v cargo-audit &> /dev/null; then
    if cargo audit; then
        echo -e "${GREEN}✓ No security issues found${NC}\n"
    else
        echo -e "${YELLOW}⚠️  Security issues found (review above)${NC}\n"
    fi
else
    echo -e "${YELLOW}⚠️  cargo-audit not installed. Run: cargo install cargo-audit${NC}\n"
fi

# 7. Dependency check (optional, warn only)
echo -e "${YELLOW}📦 Checking dependencies...${NC}"
if command -v cargo-deny &> /dev/null; then
    if cargo deny check; then
        echo -e "${GREEN}✓ Dependency check passed${NC}\n"
    else
        echo -e "${YELLOW}⚠️  Dependency issues found (review above)${NC}\n"
    fi
else
    echo -e "${YELLOW}⚠️  cargo-deny not installed. Run: cargo install cargo-deny${NC}\n"
fi

# Summary
echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${GREEN}✅ All mandatory CI checks passed!${NC}"
echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""
echo -e "${YELLOW}💡 Tips:${NC}"
echo "  • Commit your changes: git add . && git commit -m 'your message'"
echo "  • Push to trigger CI: git push"
echo "  • View CI results: https://github.com/YOUR_USERNAME/buttre/actions"
echo ""
