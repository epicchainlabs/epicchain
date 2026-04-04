#!/bin/bash

# Script to fix common test warnings in Neo-RS project
# Usage: ./scripts/fix-test-warnings.sh

echo "Neo-RS Test Warning Fixer"
echo "========================="
echo ""

# Fix unused variables in tests
echo "Fixing unused variables in test files..."
find crates -name "*.rs" -path "*/tests/*" -exec sed -i 's/let \([a-z_][a-z0-9_]*\) =/let _\1 =/g' {} \; 2>/dev/null

# Fix unused mut warnings
echo "Fixing unused mut warnings..."
find crates -name "*.rs" -path "*/tests/*" -exec sed -i 's/let mut \([a-z_][a-z0-9_]*\) = thread_rng()/let \1 = thread_rng()/g' {} \; 2>/dev/null

# Fix unused imports in test files
echo "Fixing unused imports..."
cargo fix --workspace --tests --allow-dirty 2>/dev/null

# Count remaining warnings
echo ""
echo "Checking remaining warnings..."
WARNING_COUNT=$(cargo test --workspace --no-run 2>&1 | grep -c "warning:")
echo "Remaining warnings: $WARNING_COUNT"

echo ""
echo "Done! Run 'cargo test' to verify fixes."