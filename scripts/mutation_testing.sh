#!/bin/bash
# Mutation Testing Framework for Neo-RS
# 
# This script implements comprehensive mutation testing to validate
# the quality and effectiveness of the test suite.

set -e

echo "🧬 Neo-RS Mutation Testing Framework"
echo "===================================="

# Configuration
MUTATION_CONFIG_FILE="mutation_config.toml"
MUTATION_RESULTS_DIR="target/mutation-results"
BASELINE_COVERAGE_FILE="target/coverage-baseline.json"
MUTATION_REPORT_FILE="$MUTATION_RESULTS_DIR/mutation-report.html"

# Ensure required tools are installed
check_dependencies() {
    echo "🔍 Checking dependencies..."
    
    if ! command -v cargo-mutants &> /dev/null; then
        echo "Installing cargo-mutants..."
        cargo install --locked cargo-mutants
    fi
    
    if ! command -v cargo-tarpaulin &> /dev/null; then
        echo "Installing cargo-tarpaulin..."
        cargo install cargo-tarpaulin
    fi
    
    echo "✅ Dependencies ready"
}

# Create mutation testing configuration
create_mutation_config() {
    echo "📝 Creating mutation configuration..."
    
    cat > $MUTATION_CONFIG_FILE << 'EOF'
# Mutation Testing Configuration for Neo-RS

# Target crates for mutation testing
[targets]
core = { path = "crates/core", priority = "high" }
vm = { path = "crates/vm", priority = "high" }
cryptography = { path = "crates/cryptography", priority = "critical" }
consensus = { path = "crates/consensus", priority = "high" }
network = { path = "crates/network", priority = "medium" }
ledger = { path = "crates/ledger", priority = "high" }

# Mutation operators to apply
[operators]
# Arithmetic mutations
arithmetic = [
    "replace + with -",
    "replace - with +", 
    "replace * with /",
    "replace / with *",
    "replace % with *",
]

# Relational mutations  
relational = [
    "replace == with !=",
    "replace != with ==",
    "replace < with >=",
    "replace > with <=",
    "replace <= with >",
    "replace >= with <",
]

# Logical mutations
logical = [
    "replace && with ||",
    "replace || with &&",
    "replace ! with identity",
]

# Conditional mutations
conditional = [
    "replace if with if !",
    "replace while with while !",
    "replace true with false",
    "replace false with true",
]

# Boundary mutations
boundary = [
    "replace 0 with 1",
    "replace 1 with 0", 
    "replace n with n+1",
    "replace n with n-1",
]

# Method mutations
method = [
    "replace return with panic",
    "remove function calls",
    "replace Some with None",
    "replace Ok with Err",
]

# Configuration
[config]
timeout_multiplier = 3.0
minimum_test_coverage = 0.80
target_mutation_score = 0.85
exclude_patterns = [
    "tests/**",
    "**/tests.rs", 
    "**/bench.rs",
    "target/**",
    "examples/**"
]

# Critical modules requiring higher mutation scores
[critical_modules]
"crates/cryptography/src/ecdsa.rs" = 0.95
"crates/cryptography/src/hash.rs" = 0.95
"crates/vm/src/execution_engine.rs" = 0.90
"crates/consensus/src/dbft.rs" = 0.90
"crates/core/src/transaction.rs" = 0.90
EOF

    echo "✅ Mutation configuration created"
}

# Generate baseline coverage
generate_baseline_coverage() {
    echo "📊 Generating baseline test coverage..."
    
    mkdir -p "$(dirname "$BASELINE_COVERAGE_FILE")"
    
    # Run coverage analysis
    cargo tarpaulin \
        --out Json \
        --output-dir target \
        --exclude-files target/* \
        --exclude-files tests/* \
        --exclude-files */tests.rs \
        --exclude-files */bench.rs \
        --timeout 300 \
        --verbose
    
    # Move to baseline location
    mv target/tarpaulin-report.json "$BASELINE_COVERAGE_FILE"
    
    # Extract coverage percentage
    local coverage=$(cat "$BASELINE_COVERAGE_FILE" | jq -r '.files | map(.coverage) | add / length')
    echo "📈 Baseline coverage: ${coverage}%"
    
    if (( $(echo "$coverage < 80" | bc -l) )); then
        echo "⚠️  Warning: Coverage below 80% threshold"
        echo "   Consider adding more tests before mutation testing"
    fi
    
    echo "✅ Baseline coverage generated"
}

# Run mutation testing by category
run_mutation_category() {
    local category=$1
    local crate_path=$2
    local priority=$3
    
    echo "🧬 Running $priority priority mutations on $category..."
    
    local output_dir="$MUTATION_RESULTS_DIR/$category"
    mkdir -p "$output_dir"
    
    # Determine number of mutants based on priority
    local mutant_limit
    case $priority in
        "critical") mutant_limit=1000 ;;
        "high") mutant_limit=500 ;;
        "medium") mutant_limit=250 ;;
        "low") mutant_limit=100 ;;
        *) mutant_limit=100 ;;
    esac
    
    # Run cargo-mutants with configuration
    cargo mutants \
        --package "$(basename "$crate_path")" \
        --output "$output_dir" \
        --timeout-multiplier 3.0 \
        --test-tool cargo \
        --jobs "$(nproc)" \
        --shuffle \
        --baseline auto \
        --limit "$mutant_limit" || true
    
    # Parse results
    if [ -f "$output_dir/mutants.out" ]; then
        local total_mutants=$(grep -c "MUTANT" "$output_dir/mutants.out" || echo "0")
        local killed_mutants=$(grep -c "KILLED" "$output_dir/mutants.out" || echo "0") 
        local survived_mutants=$(grep -c "SURVIVED" "$output_dir/mutants.out" || echo "0")
        local timeout_mutants=$(grep -c "TIMEOUT" "$output_dir/mutants.out" || echo "0")
        
        if [ "$total_mutants" -gt 0 ]; then
            local mutation_score=$(echo "scale=2; $killed_mutants * 100 / $total_mutants" | bc)
            echo "📊 $category Results:"
            echo "   Total mutants: $total_mutants"
            echo "   Killed: $killed_mutants"
            echo "   Survived: $survived_mutants" 
            echo "   Timeout: $timeout_mutants"
            echo "   Mutation Score: ${mutation_score}%"
            
            # Store results for report generation
            echo "$category,$total_mutants,$killed_mutants,$survived_mutants,$timeout_mutants,$mutation_score" \
                >> "$MUTATION_RESULTS_DIR/summary.csv"
        else
            echo "⚠️  No mutants generated for $category"
        fi
    else
        echo "❌ No output file found for $category"
    fi
    
    echo "✅ Completed $category mutation testing"
}

# Run comprehensive mutation testing
run_mutation_testing() {
    echo "🚀 Starting comprehensive mutation testing..."
    
    mkdir -p "$MUTATION_RESULTS_DIR"
    echo "category,total,killed,survived,timeout,score" > "$MUTATION_RESULTS_DIR/summary.csv"
    
    # Critical cryptography mutations
    run_mutation_category "cryptography" "crates/cryptography" "critical"
    
    # High priority core components
    run_mutation_category "core" "crates/core" "high"
    run_mutation_category "vm" "crates/vm" "high"
    run_mutation_category "consensus" "crates/consensus" "high"
    run_mutation_category "ledger" "crates/ledger" "high"
    
    # Medium priority components
    run_mutation_category "network" "crates/network" "medium"
    
    echo "✅ All mutation testing completed"
}

# Analyze mutation test results
analyze_results() {
    echo "📈 Analyzing mutation test results..."
    
    if [ ! -f "$MUTATION_RESULTS_DIR/summary.csv" ]; then
        echo "❌ No summary results found"
        return 1
    fi
    
    local total_mutants=0
    local total_killed=0
    local total_survived=0
    local total_timeout=0
    
    # Calculate overall statistics
    while IFS=',' read -r category total killed survived timeout score; do
        if [[ "$category" != "category" ]]; then
            total_mutants=$((total_mutants + total))
            total_killed=$((total_killed + killed))
            total_survived=$((total_survived + survived))
            total_timeout=$((total_timeout + timeout))
        fi
    done < "$MUTATION_RESULTS_DIR/summary.csv"
    
    local overall_score=0
    if [ "$total_mutants" -gt 0 ]; then
        overall_score=$(echo "scale=2; $total_killed * 100 / $total_mutants" | bc)
    fi
    
    echo ""
    echo "🎯 Overall Mutation Testing Results"
    echo "=================================="
    echo "Total Mutants Generated: $total_mutants"
    echo "Mutants Killed: $total_killed"
    echo "Mutants Survived: $total_survived"
    echo "Mutants Timeout: $total_timeout"
    echo "Overall Mutation Score: ${overall_score}%"
    echo ""
    
    # Quality assessment
    if (( $(echo "$overall_score >= 85" | bc -l) )); then
        echo "🏆 EXCELLENT: Test suite quality is exceptional"
    elif (( $(echo "$overall_score >= 75" | bc -l) )); then
        echo "✅ GOOD: Test suite quality is solid"
    elif (( $(echo "$overall_score >= 60" | bc -l) )); then
        echo "⚠️  MODERATE: Test suite needs improvement"
    else
        echo "🚨 POOR: Test suite requires immediate attention"
    fi
    
    # Generate detailed analysis
    echo ""
    echo "📊 Per-Category Analysis:"
    echo "========================"
    while IFS=',' read -r category total killed survived timeout score; do
        if [[ "$category" != "category" ]]; then
            printf "%-15s | Score: %6s%% | Mutants: %4s | Killed: %4s | Survived: %3s\n" \
                "$category" "$score" "$total" "$killed" "$survived"
        fi
    done < "$MUTATION_RESULTS_DIR/summary.csv"
}

# Generate HTML report
generate_html_report() {
    echo "📄 Generating HTML mutation report..."
    
    cat > "$MUTATION_REPORT_FILE" << 'EOF'
<!DOCTYPE html>
<html>
<head>
    <title>Neo-RS Mutation Testing Report</title>
    <style>
        body { font-family: Arial, sans-serif; margin: 40px; }
        .header { background: #f0f0f0; padding: 20px; border-radius: 5px; }
        .summary { margin: 20px 0; }
        .category { margin: 10px 0; padding: 15px; border: 1px solid #ddd; border-radius: 5px; }
        .excellent { background-color: #d4edda; border-color: #28a745; }
        .good { background-color: #d1ecf1; border-color: #17a2b8; }
        .moderate { background-color: #fff3cd; border-color: #ffc107; }
        .poor { background-color: #f8d7da; border-color: #dc3545; }
        table { border-collapse: collapse; width: 100%; margin: 20px 0; }
        th, td { border: 1px solid #ddd; padding: 8px; text-align: right; }
        th { background-color: #f2f2f2; }
        .score-excellent { color: #28a745; font-weight: bold; }
        .score-good { color: #17a2b8; font-weight: bold; }
        .score-moderate { color: #ffc107; font-weight: bold; }
        .score-poor { color: #dc3545; font-weight: bold; }
    </style>
</head>
<body>
    <div class="header">
        <h1>🧬 Neo-RS Mutation Testing Report</h1>
        <p>Generated on: $(date)</p>
    </div>
EOF
    
    # Add overall summary to HTML
    local overall_score=$(tail -n +2 "$MUTATION_RESULTS_DIR/summary.csv" | awk -F',' 'BEGIN{total=0; killed=0} {total+=$2; killed+=$3} END{if(total>0) printf "%.1f", killed*100/total; else print "0"}')
    
    cat >> "$MUTATION_REPORT_FILE" << EOF
    
    <div class="summary">
        <h2>📊 Overall Results</h2>
        <p><strong>Mutation Score:</strong> ${overall_score}%</p>
    </div>
    
    <table>
        <tr>
            <th>Category</th>
            <th>Total Mutants</th>
            <th>Killed</th>
            <th>Survived</th>
            <th>Timeout</th>
            <th>Score</th>
        </tr>
EOF
    
    # Add per-category results
    while IFS=',' read -r category total killed survived timeout score; do
        if [[ "$category" != "category" ]]; then
            local score_class="score-poor"
            if (( $(echo "$score >= 85" | bc -l) )); then
                score_class="score-excellent"
            elif (( $(echo "$score >= 75" | bc -l) )); then
                score_class="score-good"
            elif (( $(echo "$score >= 60" | bc -l) )); then
                score_class="score-moderate"
            fi
            
            cat >> "$MUTATION_REPORT_FILE" << EOF
        <tr>
            <td style="text-align: left;">$category</td>
            <td>$total</td>
            <td>$killed</td>
            <td>$survived</td>
            <td>$timeout</td>
            <td class="$score_class">${score}%</td>
        </tr>
EOF
        fi
    done < "$MUTATION_RESULTS_DIR/summary.csv"
    
    cat >> "$MUTATION_REPORT_FILE" << 'EOF'
    </table>
    
    <div class="summary">
        <h2>📋 Recommendations</h2>
        <ul>
            <li>Focus on categories with scores below 75%</li>
            <li>Review survived mutants to identify missing test cases</li>
            <li>Add edge case testing for boundary conditions</li>
            <li>Improve error condition testing</li>
            <li>Consider adding property-based tests</li>
        </ul>
    </div>
    
</body>
</html>
EOF
    
    echo "✅ HTML report generated: $MUTATION_REPORT_FILE"
}

# Main execution function
main() {
    local command=${1:-"all"}
    
    case $command in
        "deps")
            check_dependencies
            ;;
        "config")
            create_mutation_config
            ;;
        "coverage")
            generate_baseline_coverage
            ;;
        "mutate")
            run_mutation_testing
            ;;
        "analyze")
            analyze_results
            ;;
        "report")
            generate_html_report
            ;;
        "all")
            check_dependencies
            create_mutation_config
            generate_baseline_coverage
            run_mutation_testing
            analyze_results
            generate_html_report
            ;;
        *)
            echo "Usage: $0 [deps|config|coverage|mutate|analyze|report|all]"
            echo ""
            echo "Commands:"
            echo "  deps     - Install required dependencies"
            echo "  config   - Create mutation testing configuration"
            echo "  coverage - Generate baseline coverage report"
            echo "  mutate   - Run mutation testing"
            echo "  analyze  - Analyze mutation results"
            echo "  report   - Generate HTML report"
            echo "  all      - Run complete mutation testing pipeline (default)"
            exit 1
            ;;
    esac
}

# Execute main function with all arguments
main "$@"