#!/bin/bash
set -e

echo "=== Phase 3: Profile-Guided Optimization Build ==="
echo ""

# Configuration
PGO_DATA="/tmp/pgo-data"
BASELINE_BENCH="baseline_bench.txt"
PGO_BENCH="pgo_bench.txt"

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Step 1: Clean and prepare
echo -e "${YELLOW}Step 1: Cleaning previous builds...${NC}"
cargo clean
rm -rf "$PGO_DATA"
mkdir -p "$PGO_DATA"
echo -e "${GREEN}✓ Clean complete${NC}"
echo ""

# Step 2: Baseline benchmark (optional, for comparison)
echo -e "${YELLOW}Step 2: Running baseline benchmark (no PGO)...${NC}"
cargo build --release --example pgo_workload 2>&1 | grep -E "(Compiling|Finished)" || true
if [ -f "benches/end_to_end_vni.rs" ]; then
    echo "Running baseline benchmarks..."
    cargo bench --bench end_to_end_vni 2>&1 | tee "$BASELINE_BENCH" || echo "⚠ Benchmarks not available yet"
fi
echo -e "${GREEN}✓ Baseline complete${NC}"
echo ""

# Step 3: Build with instrumentation
echo -e "${YELLOW}Step 3: Building with instrumentation...${NC}"
cargo clean
RUSTFLAGS="-Cprofile-generate=$PGO_DATA" cargo build --release --example pgo_workload
echo -e "${GREEN}✓ Instrumented build complete${NC}"
echo ""

# Step 4: Run workload
echo -e "${YELLOW}Step 4: Running workload to collect profile data...${NC}"
./target/release/examples/pgo_workload.exe || ./target/release/examples/pgo_workload
echo -e "${GREEN}✓ Workload complete${NC}"
echo ""

# Step 5: Verify profile data
echo -e "${YELLOW}Step 5: Verifying profile data...${NC}"
if ls "$PGO_DATA"/*.profraw 1> /dev/null 2>&1; then
    echo -e "${GREEN}✓ Profile data generated successfully${NC}"
    ls -lh "$PGO_DATA"/*.profraw
    PROFRAW_COUNT=$(ls -1 "$PGO_DATA"/*.profraw | wc -l)
    echo "Generated $PROFRAW_COUNT profile data file(s)"
else
    echo -e "${RED}✗ No profile data found!${NC}"
    exit 1
fi
echo ""

# Step 6: Merge profile data (if llvm-profdata available)
echo -e "${YELLOW}Step 6: Processing profile data...${NC}"
if command -v llvm-profdata &> /dev/null; then
    echo "Merging profile data with llvm-profdata..."
    llvm-profdata merge -o "$PGO_DATA/merged.profdata" "$PGO_DATA"/*.profraw
    PROFILE_FLAG="-Cprofile-use=$PGO_DATA/merged.profdata"
    ls -lh "$PGO_DATA/merged.profdata"
    echo -e "${GREEN}✓ Profile data merged${NC}"
else
    echo -e "${YELLOW}⚠ llvm-profdata not found, using raw profile data${NC}"
    PROFILE_FLAG="-Cprofile-use=$PGO_DATA"
fi
echo ""

# Step 7: Clean and rebuild with PGO
echo -e "${YELLOW}Step 7: Rebuilding with profile-guided optimization...${NC}"
cargo clean
RUSTFLAGS="$PROFILE_FLAG" cargo build --release
echo -e "${GREEN}✓ PGO build complete${NC}"
echo ""

# Step 8: Run PGO benchmarks
echo -e "${YELLOW}Step 8: Running PGO benchmarks...${NC}"
if [ -f "benches/end_to_end_vni.rs" ]; then
    RUSTFLAGS="$PROFILE_FLAG" cargo bench --bench end_to_end_vni 2>&1 | tee "$PGO_BENCH" || echo "⚠ Benchmarks not available"
    
    # Compare results if both benchmarks exist
    if [ -f "$BASELINE_BENCH" ] && [ -f "$PGO_BENCH" ]; then
        echo ""
        echo "=== Performance Comparison ==="
        echo "Baseline results: $BASELINE_BENCH"
        echo "PGO results: $PGO_BENCH"
        echo ""
        echo "Use 'diff $BASELINE_BENCH $PGO_BENCH' to compare"
    fi
fi
echo ""

# Step 9: Summary
echo "=== PGO Build Complete ==="
echo ""
echo -e "${GREEN}✓ Binary location:${NC} target/release/"
echo -e "${GREEN}✓ Profile data:${NC} $PGO_DATA"
echo -e "${GREEN}✓ Baseline bench:${NC} $BASELINE_BENCH (if available)"
echo -e "${GREEN}✓ PGO bench:${NC} $PGO_BENCH (if available)"
echo ""
echo "To rebuild the library with PGO:"
echo "  RUSTFLAGS=\"$PROFILE_FLAG\" cargo build --release"
echo ""
echo "To run tests with PGO build:"
echo "  RUSTFLAGS=\"$PROFILE_FLAG\" cargo test --release"
echo ""
echo "To analyze profile data (if llvm-profdata available):"
echo "  llvm-profdata show --all-functions $PGO_DATA/merged.profdata | head -50"
echo ""
