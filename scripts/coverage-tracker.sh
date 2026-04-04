#!/bin/bash

# Coverage Tracking and Reporting Script
# Monitors test coverage over time and generates reports

set -e

# Configuration
COVERAGE_DIR="coverage"
HISTORY_FILE="$COVERAGE_DIR/coverage_history.csv"
THRESHOLD_LINE=80
THRESHOLD_BRANCH=70

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

# Create coverage directory
mkdir -p "$COVERAGE_DIR"

# Initialize history file if it doesn't exist
if [ ! -f "$HISTORY_FILE" ]; then
    echo "date,line_coverage,branch_coverage,total_lines,covered_lines" > "$HISTORY_FILE"
fi

echo -e "${BLUE}═══════════════════════════════════════${NC}"
echo -e "${BLUE}    Neo-RS Coverage Tracking System    ${NC}"
echo -e "${BLUE}═══════════════════════════════════════${NC}"
echo ""

# Function to extract coverage percentage from tarpaulin output
extract_coverage() {
    local output="$1"
    echo "$output" | grep -oP '\d+\.\d+%' | head -1 | tr -d '%'
}

# Run coverage analysis
echo "📊 Running coverage analysis..."
COVERAGE_OUTPUT=$(cargo tarpaulin --workspace --print-summary 2>&1 || echo "error")

if [[ "$COVERAGE_OUTPUT" == *"error"* ]]; then
    echo -e "${RED}❌ Coverage analysis failed${NC}"
    echo "Trying with reduced scope..."
    COVERAGE_OUTPUT=$(cargo tarpaulin --lib --print-summary 2>&1 || echo "error")
fi

# Extract metrics
LINE_COVERAGE=$(extract_coverage "$COVERAGE_OUTPUT")
if [ -z "$LINE_COVERAGE" ]; then
    echo -e "${YELLOW}⚠️ Could not extract coverage metrics${NC}"
    LINE_COVERAGE="0.0"
fi

# Generate detailed HTML report
echo "📄 Generating detailed coverage report..."
cargo tarpaulin --workspace --out Html --output-dir "$COVERAGE_DIR" 2>/dev/null || {
    echo "⚠️ HTML report generation failed"
}

# Record coverage history
DATE=$(date +%Y-%m-%d)
echo "$DATE,$LINE_COVERAGE,0,0,0" >> "$HISTORY_FILE"

# Generate coverage badge
generate_badge() {
    local coverage=$1
    local color="red"
    
    if (( $(echo "$coverage >= $THRESHOLD_LINE" | bc -l) )); then
        color="green"
    elif (( $(echo "$coverage >= 60" | bc -l) )); then
        color="yellow"
    fi
    
    cat > "$COVERAGE_DIR/coverage-badge.svg" << EOF
<svg xmlns="http://www.w3.org/2000/svg" width="114" height="20">
  <linearGradient id="b" x2="0" y2="100%">
    <stop offset="0" stop-color="#bbb" stop-opacity=".1"/>
    <stop offset="1" stop-opacity=".1"/>
  </linearGradient>
  <mask id="a">
    <rect width="114" height="20" rx="3" fill="#fff"/>
  </mask>
  <g mask="url(#a)">
    <path fill="#555" d="M0 0h63v20H0z"/>
    <path fill="$color" d="M63 0h51v20H63z"/>
    <path fill="url(#b)" d="M0 0h114v20H0z"/>
  </g>
  <g fill="#fff" text-anchor="middle" font-family="DejaVu Sans,Verdana,Geneva,sans-serif" font-size="11">
    <text x="31.5" y="15" fill="#010101" fill-opacity=".3">coverage</text>
    <text x="31.5" y="14">coverage</text>
    <text x="87.5" y="15" fill="#010101" fill-opacity=".3">${coverage}%</text>
    <text x="87.5" y="14">${coverage}%</text>
  </g>
</svg>
EOF
}

generate_badge "$LINE_COVERAGE"

# Generate trend report
echo "📈 Generating coverage trend report..."
cat > "$COVERAGE_DIR/coverage_trend.md" << EOF
# Coverage Trend Report
Generated: $(date)

## Current Coverage
- **Line Coverage**: ${LINE_COVERAGE}%
- **Threshold**: ${THRESHOLD_LINE}%
- **Status**: $(if (( $(echo "$LINE_COVERAGE >= $THRESHOLD_LINE" | bc -l) )); then echo "✅ PASSING"; else echo "❌ FAILING"; fi)

## Historical Trend
\`\`\`
$(tail -10 "$HISTORY_FILE" | column -t -s,)
\`\`\`

## Coverage by Component
$(if [ -f "$COVERAGE_DIR/tarpaulin-report.html" ]; then
    echo "Detailed report available: $COVERAGE_DIR/tarpaulin-report.html"
else
    echo "Detailed report not available"
fi)

## Recommendations
$(if (( $(echo "$LINE_COVERAGE < $THRESHOLD_LINE" | bc -l) )); then
    echo "- ⚠️ Coverage is below threshold ($THRESHOLD_LINE%)"
    echo "- Focus on testing uncovered code paths"
    echo "- Add tests for error handling scenarios"
else
    echo "- ✅ Coverage meets threshold requirements"
    echo "- Consider increasing threshold to $((THRESHOLD_LINE + 5))%"
fi)
EOF

# Display results
echo ""
echo -e "${BLUE}═══════════════════════════════════════${NC}"
echo -e "${BLUE}           Coverage Results            ${NC}"
echo -e "${BLUE}═══════════════════════════════════════${NC}"
echo ""

if (( $(echo "$LINE_COVERAGE >= $THRESHOLD_LINE" | bc -l) )); then
    echo -e "${GREEN}✅ Coverage: ${LINE_COVERAGE}% (Threshold: ${THRESHOLD_LINE}%)${NC}"
else
    echo -e "${RED}❌ Coverage: ${LINE_COVERAGE}% (Threshold: ${THRESHOLD_LINE}%)${NC}"
fi

echo ""
echo "📊 Reports generated:"
echo "  • HTML Report: $COVERAGE_DIR/tarpaulin-report.html"
echo "  • Trend Report: $COVERAGE_DIR/coverage_trend.md"
echo "  • Coverage Badge: $COVERAGE_DIR/coverage-badge.svg"
echo "  • History: $HISTORY_FILE"
echo ""

# Check for uncovered files
echo "🔍 Analyzing uncovered areas..."
if [ -f "$COVERAGE_DIR/tarpaulin-report.html" ]; then
    echo "  Review the HTML report for detailed uncovered lines"
else
    # Simple check for files with no tests
    for crate in crates/*/; do
        if [ -d "$crate" ]; then
            crate_name=$(basename "$crate")
            if [ ! -d "$crate/tests" ] && ! grep -q "#\[test\]" "$crate/src/"*.rs 2>/dev/null; then
                echo "  ⚠️ $crate_name appears to have no tests"
            fi
        fi
    done
fi

echo ""
echo -e "${GREEN}Coverage tracking complete!${NC}"

# Exit with appropriate code
if (( $(echo "$LINE_COVERAGE < $THRESHOLD_LINE" | bc -l) )); then
    exit 1
else
    exit 0
fi