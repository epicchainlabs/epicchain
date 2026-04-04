#!/bin/bash

# Test Orchestration System for Neo-RS
# Comprehensive test management and automation platform

set -e

# Configuration
TEST_DIR="test_results"
TIMESTAMP=$(date +%Y%m%d_%H%M%S)
REPORT_DIR="$TEST_DIR/$TIMESTAMP"
PARALLEL_JOBS=4

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m'

# Create directories
mkdir -p "$REPORT_DIR"/{unit,integration,benchmark,coverage,mutation,fuzzing}

echo -e "${CYAN}╔══════════════════════════════════════════════════╗${NC}"
echo -e "${CYAN}║       Neo-RS Test Orchestration Platform         ║${NC}"
echo -e "${CYAN}╚══════════════════════════════════════════════════╝${NC}"
echo ""

# Log function
log() {
    echo "[$(date +'%Y-%m-%d %H:%M:%S')] $1" | tee -a "$REPORT_DIR/orchestrator.log"
}

# Test execution function with timing
run_test_suite() {
    local suite_name=$1
    local command=$2
    local output_file=$3
    
    echo -e "${BLUE}▶ Running $suite_name...${NC}"
    local start_time=$(date +%s)
    
    if eval "$command" > "$output_file" 2>&1; then
        local end_time=$(date +%s)
        local duration=$((end_time - start_time))
        echo -e "${GREEN}✅ $suite_name completed in ${duration}s${NC}"
        log "SUCCESS: $suite_name (${duration}s)"
        echo "$suite_name,SUCCESS,$duration" >> "$REPORT_DIR/summary.csv"
        return 0
    else
        local end_time=$(date +%s)
        local duration=$((end_time - start_time))
        echo -e "${RED}❌ $suite_name failed after ${duration}s${NC}"
        log "FAILED: $suite_name (${duration}s)"
        echo "$suite_name,FAILED,$duration" >> "$REPORT_DIR/summary.csv"
        return 1
    fi
}

# Phase 1: Pre-flight checks
echo -e "${YELLOW}Phase 1: Pre-flight Checks${NC}"
echo "═══════════════════════════════"

# Check Rust toolchain
log "Checking Rust toolchain..."
rustc --version | tee -a "$REPORT_DIR/environment.txt"
cargo --version | tee -a "$REPORT_DIR/environment.txt"

# Check available tools
for tool in cargo-tarpaulin cargo-mutants cargo-fuzz; do
    if command -v $tool &> /dev/null; then
        echo -e "${GREEN}✅ $tool installed${NC}"
        log "$tool: installed"
    else
        echo -e "${YELLOW}⚠️ $tool not installed${NC}"
        log "$tool: not installed"
    fi
done

echo ""

# Phase 2: Static Analysis
echo -e "${YELLOW}Phase 2: Static Analysis${NC}"
echo "═══════════════════════════"

run_test_suite "Format Check" \
    "cargo fmt -- --check" \
    "$REPORT_DIR/format.txt" || true

run_test_suite "Clippy Analysis" \
    "cargo clippy --workspace --all-features -- -W clippy::all" \
    "$REPORT_DIR/clippy.txt" || true

echo ""

# Phase 3: Unit Tests
echo -e "${YELLOW}Phase 3: Unit Tests${NC}"
echo "════════════════════"

run_test_suite "Unit Tests" \
    "cargo test --lib --workspace -- --test-threads=$PARALLEL_JOBS" \
    "$REPORT_DIR/unit/results.txt"

# Extract test metrics
if [ -f "$REPORT_DIR/unit/results.txt" ]; then
    UNIT_PASSED=$(grep -c "test .* ok" "$REPORT_DIR/unit/results.txt" 2>/dev/null || echo "0")
    UNIT_FAILED=$(grep -c "test .* FAILED" "$REPORT_DIR/unit/results.txt" 2>/dev/null || echo "0")
    echo "Unit Tests: $UNIT_PASSED passed, $UNIT_FAILED failed"
fi

echo ""

# Phase 4: Integration Tests
echo -e "${YELLOW}Phase 4: Integration Tests${NC}"
echo "═══════════════════════════"

run_test_suite "Integration Tests" \
    "cargo test --test '*' --workspace" \
    "$REPORT_DIR/integration/results.txt"

echo ""

# Phase 5: Documentation Tests
echo -e "${YELLOW}Phase 5: Documentation Tests${NC}"
echo "════════════════════════════"

run_test_suite "Doc Tests" \
    "cargo test --doc --workspace" \
    "$REPORT_DIR/doc_tests.txt"

echo ""

# Phase 6: Coverage Analysis
echo -e "${YELLOW}Phase 6: Coverage Analysis${NC}"
echo "═══════════════════════════"

if command -v cargo-tarpaulin &> /dev/null; then
    run_test_suite "Coverage Analysis" \
        "cargo tarpaulin --workspace --timeout 300 --out Json --output-dir $REPORT_DIR/coverage" \
        "$REPORT_DIR/coverage/tarpaulin.json" || true
    
    # Extract coverage percentage
    if [ -f "$REPORT_DIR/coverage/tarpaulin.json" ]; then
        COVERAGE=$(jq '.coverage' "$REPORT_DIR/coverage/tarpaulin.json" 2>/dev/null || echo "0")
        echo -e "Coverage: ${COVERAGE}%"
    fi
else
    echo -e "${YELLOW}⚠️ Skipping coverage (cargo-tarpaulin not installed)${NC}"
fi

echo ""

# Phase 7: Benchmarks
echo -e "${YELLOW}Phase 7: Performance Benchmarks${NC}"
echo "════════════════════════════════"

if [ -d "benches" ]; then
    run_test_suite "Benchmarks" \
        "cargo bench --workspace" \
        "$REPORT_DIR/benchmark/results.txt" || true
else
    echo -e "${YELLOW}⚠️ No benchmark suite found${NC}"
fi

echo ""

# Phase 8: Security Testing
echo -e "${YELLOW}Phase 8: Security Testing${NC}"
echo "═════════════════════════"

run_test_suite "Dependency Audit" \
    "cargo audit" \
    "$REPORT_DIR/security_audit.txt" || true

echo ""

# Phase 9: Generate Reports
echo -e "${YELLOW}Phase 9: Report Generation${NC}"
echo "═══════════════════════════"

# Generate HTML report
cat > "$REPORT_DIR/index.html" << EOF
<!DOCTYPE html>
<html>
<head>
    <title>Neo-RS Test Report - $TIMESTAMP</title>
    <style>
        body { font-family: Arial, sans-serif; margin: 20px; }
        h1 { color: #333; }
        .success { color: green; }
        .failure { color: red; }
        .warning { color: orange; }
        table { border-collapse: collapse; width: 100%; }
        th, td { border: 1px solid #ddd; padding: 8px; text-align: left; }
        th { background-color: #f2f2f2; }
        .metric { font-size: 24px; font-weight: bold; }
    </style>
</head>
<body>
    <h1>Neo-RS Test Orchestration Report</h1>
    <p>Generated: $(date)</p>
    
    <h2>Summary</h2>
    <table>
        <tr><th>Test Suite</th><th>Status</th><th>Duration</th></tr>
        $(while IFS=, read -r suite status duration; do
            echo "<tr><td>$suite</td><td class='${status,,}'>$status</td><td>${duration}s</td></tr>"
        done < "$REPORT_DIR/summary.csv")
    </table>
    
    <h2>Metrics</h2>
    <p>Unit Tests Passed: <span class="metric success">$UNIT_PASSED</span></p>
    <p>Unit Tests Failed: <span class="metric failure">$UNIT_FAILED</span></p>
    <p>Code Coverage: <span class="metric">$COVERAGE%</span></p>
    
    <h2>Reports</h2>
    <ul>
        <li><a href="unit/results.txt">Unit Test Results</a></li>
        <li><a href="integration/results.txt">Integration Test Results</a></li>
        <li><a href="coverage/tarpaulin.json">Coverage Report</a></li>
        <li><a href="clippy.txt">Clippy Analysis</a></li>
        <li><a href="security_audit.txt">Security Audit</a></li>
    </ul>
</body>
</html>
EOF

# Generate JSON summary
cat > "$REPORT_DIR/summary.json" << EOF
{
    "timestamp": "$TIMESTAMP",
    "environment": {
        "rust_version": "$(rustc --version | cut -d' ' -f2)",
        "cargo_version": "$(cargo --version | cut -d' ' -f2)"
    },
    "results": {
        "unit_tests": {
            "passed": $UNIT_PASSED,
            "failed": $UNIT_FAILED
        },
        "coverage": $COVERAGE,
        "duration_seconds": $(($(date +%s) - start_time))
    }
}
EOF

echo -e "${GREEN}✅ Reports generated in: $REPORT_DIR${NC}"
echo ""

# Phase 10: Summary
echo -e "${CYAN}╔══════════════════════════════════════════════════╗${NC}"
echo -e "${CYAN}║              Orchestration Complete               ║${NC}"
echo -e "${CYAN}╚══════════════════════════════════════════════════╝${NC}"
echo ""

# Display summary
echo "📊 Test Summary:"
echo "═══════════════"
cat "$REPORT_DIR/summary.csv" | column -t -s,
echo ""

# Calculate overall status
FAILURES=$(grep -c "FAILED" "$REPORT_DIR/summary.csv" 2>/dev/null || echo "0")
if [ "$FAILURES" -eq 0 ]; then
    echo -e "${GREEN}✅ All test suites passed successfully!${NC}"
    exit 0
else
    echo -e "${RED}❌ $FAILURES test suite(s) failed${NC}"
    echo "Review detailed reports in: $REPORT_DIR"
    exit 1
fi