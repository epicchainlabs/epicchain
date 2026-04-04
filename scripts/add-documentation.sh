#!/bin/bash

# Script to add missing documentation to public items
# Usage: ./scripts/add-documentation.sh [file]

add_docs_to_file() {
    local file=$1
    echo "Processing: $file"
    
    # Add docs for pub fn new
    sed -i '/pub fn new(/i\    /// Creates a new instance.' "$file"
    
    # Add docs for pub fn snapshot
    sed -i '/pub fn snapshot(/i\    /// Returns a snapshot of the current state.' "$file"
    
    # Add docs for pub fn reset
    sed -i '/pub fn reset(/i\    /// Resets the internal state.' "$file"
    
    # Add docs for pub fn record_
    sed -i '/pub fn record_/i\    /// Records an event for metrics tracking.' "$file"
    
    # Add docs for pub fn update_
    sed -i '/pub fn update_/i\    /// Updates the internal state with new data.' "$file"
}

if [ -z "$1" ]; then
    echo "Adding documentation to all source files..."
    
    # Core module files with missing docs
    FILES=(
        "crates/core/src/system_monitoring.rs"
        "crates/core/src/error_handling.rs"
        "crates/core/src/safe_operations.rs"
        "crates/core/src/monitoring/alerting.rs"
        "crates/core/src/monitoring/health.rs"
    )
    
    for file in "${FILES[@]}"; do
        if [ -f "$file" ]; then
            add_docs_to_file "$file"
        fi
    done
else
    add_docs_to_file "$1"
fi

echo "Documentation added. Run 'cargo doc' to verify."