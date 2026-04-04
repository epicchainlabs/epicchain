#!/bin/bash

# Mutation Testing Setup and Execution
# Uses cargo-mutants to verify test effectiveness

set -e

# Configuration
MUTANTS_DIR="mutants_output"
TIMEOUT_MULTIPLIER=3
MAX_THREADS=4

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
NC='\033[0m'

echo -e "${PURPLE}╔══════════════════════════════════════════╗${NC}"
echo -e "${PURPLE}║     Neo-RS Mutation Testing Suite        ║${NC}"
echo -e "${PURPLE}╚══════════════════════════════════════════╝${NC}"
echo ""

# Check if cargo-mutants is installed
if ! command -v cargo-mutants &> /dev/null; then
    echo -e "${YELLOW}📦 Installing cargo-mutants...${NC}"
    cargo install cargo-mutants
fi

# Create output directory
mkdir -p "$MUTANTS_DIR"

# Function to run mutation testing on a specific crate
run_mutation_test() {
    local crate=$1
    echo -e "${BLUE}🧬 Running mutation tests for: $crate${NC}"
    
    cargo mutants \
        --package "$crate" \
        --timeout-multiplier "$TIMEOUT_MULTIPLIER" \
        --jobs "$MAX_THREADS" \
        --output "$MUTANTS_DIR/$crate" \
        --caught-timeout-min-seconds 20 \
        2>&1 | tee "$MUTANTS_DIR/$crate.log"
    
    # Extract results
    local killed=$(grep -c "caught" "$MUTANTS_DIR/$crate.log" 2>/dev/null || echo "0")
    local survived=$(grep -c "survived" "$MUTANTS_DIR/$crate.log" 2>/dev/null || echo "0")
    local total=$((killed + survived))
    
    if [ "$total" -gt 0 ]; then
        local score=$((killed * 100 / total))
        
        if [ "$score" -ge 80 ]; then
            echo -e "${GREEN}✅ Mutation score: ${score}% (${killed}/${total} killed)${NC}"
        elif [ "$score" -ge 60 ]; then
            echo -e "${YELLOW}⚠️ Mutation score: ${score}% (${killed}/${total} killed)${NC}"
        else
            echo -e "${RED}❌ Mutation score: ${score}% (${killed}/${total} killed)${NC}"
        fi
    fi
    echo ""
}

# Select testing mode
echo "Select mutation testing mode:"
echo "1) Quick - Test core modules only"
echo "2) Standard - Test all main crates"
echo "3) Comprehensive - Test entire workspace"
echo "4) Specific - Test specific crate"
read -p "Choice (1-4): " choice

case $choice in
    1)
        echo -e "${YELLOW}Running quick mutation tests...${NC}"
        run_mutation_test "epicchain-core"
        ;;
    2)
        echo -e "${YELLOW}Running standard mutation tests...${NC}"
        for crate in epicchain-core epicchain-vm neo-network neo-consensus; do
            if [ -d "crates/${crate#neo-}" ]; then
                run_mutation_test "${crate#neo-}"
            fi
        done
        ;;
    3)
        echo -e "${YELLOW}Running comprehensive mutation tests...${NC}"
        cargo mutants \
            --workspace \
            --timeout-multiplier "$TIMEOUT_MULTIPLIER" \
            --jobs "$MAX_THREADS" \
            --output "$MUTANTS_DIR/workspace" \
            2>&1 | tee "$MUTANTS_DIR/workspace.log"
        ;;
    4)
        read -p "Enter crate name: " crate_name
        run_mutation_test "$crate_name"
        ;;
    *)
        echo "Invalid choice"
        exit 1
        ;;
esac

# Generate mutation report
echo -e "${BLUE}📊 Generating mutation testing report...${NC}"

cat > "$MUTANTS_DIR/mutation_report.md" << EOF
# Mutation Testing Report
Generated: $(date)

## Summary
Mutation testing verifies that tests can detect code changes (mutations).
A high mutation score indicates effective tests.

## Results by Crate
$(for log in $MUTANTS_DIR/*.log; do
    if [ -f "$log" ]; then
        crate=$(basename "$log" .log)
        killed=$(grep -c "caught" "$log" 2>/dev/null || echo "0")
        survived=$(grep -c "survived" "$log" 2>/dev/null || echo "0")
        timeout=$(grep -c "timeout" "$log" 2>/dev/null || echo "0")
        total=$((killed + survived))
        if [ "$total" -gt 0 ]; then
            score=$((killed * 100 / total))
            echo "- **$crate**: ${score}% (${killed}/${total} killed, ${survived} survived, ${timeout} timeout)"
        fi
    fi
done)

## Mutation Categories
- **Killed**: Test suite detected the mutation ✅
- **Survived**: Test suite didn't detect the mutation ❌
- **Timeout**: Mutation caused infinite loop ⏱️

## Common Surviving Mutations
$(grep "survived" $MUTANTS_DIR/*.log 2>/dev/null | head -10 || echo "No surviving mutations found")

## Recommendations
1. Add tests for code paths with surviving mutations
2. Improve assertion coverage in existing tests
3. Add boundary condition tests
4. Consider property-based testing for complex logic

## Next Steps
- Review surviving mutations in: $MUTANTS_DIR/
- Add tests to kill surviving mutants
- Re-run mutation tests after improvements
EOF

echo -e "${GREEN}✅ Mutation testing complete!${NC}"
echo ""
echo "📊 Reports generated:"
echo "  • Summary: $MUTANTS_DIR/mutation_report.md"
echo "  • Detailed logs: $MUTANTS_DIR/*.log"
echo "  • HTML reports: $MUTANTS_DIR/*/mutants.out/outcomes.html"
echo ""
echo "💡 Tips:"
echo "  • Review surviving mutants to identify test gaps"
echo "  • Focus on high-value code paths first"
echo "  • Aim for >80% mutation score for critical code"