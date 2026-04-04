#!/bin/bash

# Enhanced test runner with metrics and reporting
# Usage: ./scripts/test-runner.sh [options]

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Default options
VERBOSE=false
COVERAGE=false
BENCH=false
QUICK=false

# Parse arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --verbose)
            VERBOSE=true
            shift
            ;;
        --coverage)
            COVERAGE=true
            shift
            ;;
        --bench)
            BENCH=true
            shift
            ;;
        --quick)
            QUICK=true
            shift
            ;;
        *)
            echo "Unknown option: $1"
            echo "Usage: $0 [--verbose] [--coverage] [--bench] [--quick]"
            exit 1
            ;;
    esac
done

echo -e "${GREEN}╔══════════════════════════════════════╗${NC}"
echo -e "${GREEN}║     Neo-RS Test Suite Runner         ║${NC}"
echo -e "${GREEN}╚══════════════════════════════════════╝${NC}"
echo ""

# Function to run tests with timing
run_tests() {
    local category=$1
    local command=$2
    
    echo -e "${YELLOW}Running $category tests...${NC}"
    START_TIME=$(date +%s)
    
    if $VERBOSE; then
        eval "$command"
    else
        eval "$command" > /tmp/test_output.txt 2>&1
        if [ $? -eq 0 ]; then
            echo -e "${GREEN}✅ $category tests passed${NC}"
        else
            echo -e "${RED}❌ $category tests failed${NC}"
            cat /tmp/test_output.txt
            exit 1
        fi
    fi
    
    END_TIME=$(date +%s)
    DURATION=$((END_TIME - START_TIME))
    echo "⏱️  Duration: ${DURATION}s"
    echo ""
}

# Quick tests (unit tests only)
if $QUICK; then
    echo "Running quick test suite..."
    run_tests "Unit" "cargo test --lib --workspace"
    exit 0
fi

# Full test suite
echo "📊 Test Statistics:"
echo "==================="
TEST_COUNT=$(grep -r "#\[test\]" --include="*.rs" crates/ | wc -l)
IGNORED_COUNT=$(grep -r "#\[ignore\]" --include="*.rs" crates/ | wc -l)
TEST_FILES=$(find crates -name "*.rs" -path "*/tests/*" | wc -l)

echo "Total test functions: $TEST_COUNT"
echo "Ignored tests: $IGNORED_COUNT"
echo "Test files: $TEST_FILES"
echo ""

# Run different test categories
echo "🧪 Running Test Suite:"
echo "====================="

# 1. Unit tests
run_tests "Unit" "cargo test --lib --workspace"

# 2. Integration tests
run_tests "Integration" "cargo test --test '*' --workspace"

# 3. Doc tests
run_tests "Documentation" "cargo test --doc --workspace"

# 4. Benchmarks (if requested)
if $BENCH; then
    echo -e "${YELLOW}Running benchmarks...${NC}"
    cargo bench --workspace
fi

# 5. Coverage (if requested)
if $COVERAGE; then
    echo -e "${YELLOW}Generating coverage report...${NC}"
    if command -v cargo-tarpaulin &> /dev/null; then
        cargo tarpaulin --workspace --out Html
        echo "Coverage report generated: tarpaulin-report.html"
    else
        echo "cargo-tarpaulin not installed. Install with: cargo install cargo-tarpaulin"
    fi
fi

# Summary
echo ""
echo -e "${GREEN}╔══════════════════════════════════════╗${NC}"
echo -e "${GREEN}║        Test Suite Complete!          ║${NC}"
echo -e "${GREEN}╚══════════════════════════════════════╝${NC}"
echo ""
echo "Summary:"
echo "--------"
echo "✅ All tests passed successfully"
echo "📈 Test coverage: Run with --coverage for detailed report"
echo "⚡ Performance: Run with --bench for benchmarks"