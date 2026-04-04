#!/bin/bash

# Unwrap Migration Script for Neo-RS
# Helps automate the migration from unwrap() to safe error handling

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Check if module is provided
if [ $# -eq 0 ]; then
    echo -e "${RED}Error: No module specified${NC}"
    echo "Usage: $0 <module_name> [--dry-run]"
    echo "Example: $0 network"
    exit 1
fi

MODULE=$1
DRY_RUN=false

if [ "$2" == "--dry-run" ]; then
    DRY_RUN=true
    echo -e "${YELLOW}Running in dry-run mode (no files will be modified)${NC}"
fi

MODULE_PATH="crates/$MODULE"

# Check if module exists
if [ ! -d "$MODULE_PATH" ]; then
    echo -e "${RED}Error: Module '$MODULE' not found at $MODULE_PATH${NC}"
    exit 1
fi

echo -e "${GREEN}=== Unwrap Migration Tool for $MODULE ===${NC}"
echo

# Count total unwraps
TOTAL_UNWRAPS=$(rg "\.unwrap\(\)" "$MODULE_PATH" -c | awk -F: '{sum += $2} END {print sum}')
echo -e "Found ${YELLOW}$TOTAL_UNWRAPS${NC} unwrap() calls in module $MODULE"
echo

# Find files with unwrap
FILES_WITH_UNWRAP=$(rg -l "\.unwrap\(\)" "$MODULE_PATH" 2>/dev/null || true)

if [ -z "$FILES_WITH_UNWRAP" ]; then
    echo -e "${GREEN}No unwrap() calls found in $MODULE!${NC}"
    exit 0
fi

# Count affected files
FILE_COUNT=$(echo "$FILES_WITH_UNWRAP" | wc -l)
echo -e "Affected files: ${YELLOW}$FILE_COUNT${NC}"
echo

# Process each file
for file in $FILES_WITH_UNWRAP; do
    # Count unwraps in this file
    UNWRAP_COUNT=$(rg "\.unwrap\(\)" "$file" -c | cut -d: -f2)
    echo -e "Processing ${YELLOW}$file${NC} ($UNWRAP_COUNT unwraps)..."
    
    if [ "$DRY_RUN" == "false" ]; then
        # Create backup
        cp "$file" "$file.backup"
        
        # Apply basic transformations
        # 1. Simple unwrap() -> ?
        sed -i 's/\.unwrap()/\.ok_or_else(|| todo!("Add proper error context"))?/g' "$file"
        
        # 2. expect("message") -> safe_expect("message")?
        sed -i 's/\.expect(\([^)]*\))/\.safe_expect(\1)?/g' "$file"
        
        # Add import if not present
        if ! grep -q "use crate::safe_result" "$file"; then
            # Add import after the first use statement or at the top
            if grep -q "^use " "$file"; then
                sed -i '/^use /a use crate::safe_result::{SafeOption, SafeResult};' "$file"
            else
                sed -i '1a use crate::safe_result::{SafeOption, SafeResult};' "$file"
            fi
        fi
        
        echo -e "  ${GREEN}✓${NC} Basic migration applied (manual review required)"
    else
        echo -e "  ${YELLOW}[DRY-RUN]${NC} Would migrate $UNWRAP_COUNT unwraps"
    fi
done

echo
echo -e "${GREEN}=== Migration Summary ===${NC}"
echo -e "Module: $MODULE"
echo -e "Total unwraps: $TOTAL_UNWRAPS"
echo -e "Files affected: $FILE_COUNT"

if [ "$DRY_RUN" == "false" ]; then
    echo
    echo -e "${YELLOW}⚠️  IMPORTANT:${NC}"
    echo "1. Backup files created with .backup extension"
    echo "2. Review all TODO markers and add proper error context"
    echo "3. Ensure proper error types are imported"
    echo "4. Run 'cargo build -p neo-$MODULE' to check compilation"
    echo "5. Run 'cargo test -p neo-$MODULE' to verify tests"
else
    echo
    echo "Run without --dry-run to apply changes"
fi

# Generate report
REPORT_FILE="$MODULE_PATH/unwrap_migration_report.md"
if [ "$DRY_RUN" == "false" ]; then
    cat > "$REPORT_FILE" << EOF
# Unwrap Migration Report for $MODULE

## Statistics
- Total unwraps migrated: $TOTAL_UNWRAPS
- Files modified: $FILE_COUNT
- Date: $(date)

## Files Modified
EOF

    for file in $FILES_WITH_UNWRAP; do
        UNWRAP_COUNT=$(rg "\.unwrap\(\)" "$file.backup" -c | cut -d: -f2 || echo "0")
        echo "- $file ($UNWRAP_COUNT unwraps)" >> "$REPORT_FILE"
    done

    cat >> "$REPORT_FILE" << EOF

## Next Steps
1. Review all TODO markers in the code
2. Add appropriate error context messages
3. Update error types as needed
4. Run tests to ensure functionality
5. Remove backup files after verification

## Manual Review Checklist
- [ ] All TODO markers replaced with context
- [ ] Error types properly imported
- [ ] Tests updated for new error paths
- [ ] Documentation updated
- [ ] Performance impact assessed
EOF

    echo
    echo -e "${GREEN}Report generated:${NC} $REPORT_FILE"
fi

echo
echo -e "${GREEN}Migration tool completed!${NC}"