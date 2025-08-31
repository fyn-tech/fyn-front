#!/bin/bash

# fix_rust_client.sh - Enhanced fix for OpenAPI Generator Rust client models::models:: bug
# Issue: https://github.com/OpenAPITools/openapi-generator/issues/20141

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
FYN_API_DIR="../fyn-front/fyn_api"

echo "Fixing OpenAPI Generator Rust client bug (models::models:: -> models::)"
echo "   Issue: https://github.com/OpenAPITools/openapi-generator/issues/20141"
echo "   Generator version: $(openapi-generator-cli version 2>/dev/null || echo 'unknown')"

# Check if the directory exists
if [ ! -d "$FYN_API_DIR" ]; then
    echo "Error: fyn_api directory not found at $FYN_API_DIR"
    exit 1
fi

# Count files that need fixing
files_to_fix=$(find "$FYN_API_DIR/src" -name "*.rs" -type f -exec grep -l "models::models::" {} \; 2>/dev/null | wc -l)

if [ "$files_to_fix" -eq 0 ]; then
    echo "No files need fixing - models::models:: bug not present"
    exit 0
fi

echo "Found $files_to_fix file(s) that need fixing"

# Apply the fix
find "$FYN_API_DIR/src" -name "*.rs" -type f -exec grep -l "models::models::" {} \; 2>/dev/null | while read file; do
    echo "   Fixing: $(basename "$file")"
    sed -i 's/models::models::/models::/g' "$file"
done

# Verify the fix worked
remaining_issues=$(find "$FYN_API_DIR/src" -name "*.rs" -type f -exec grep -l "models::models::" {} \; 2>/dev/null | wc -l)

if [ "$remaining_issues" -eq 0 ]; then
    echo "All fixes applied successfully!"
    echo "You can now run: cargo build"
else
    echo "Warning: $remaining_issues file(s) still have issues"
    find "$FYN_API_DIR/src" -name "*.rs" -type f -exec grep -l "models::models::" {} \; 2>/dev/null
fi