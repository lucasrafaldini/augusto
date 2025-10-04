#!/bin/bash
set -e
# Script to fix line endings in all markdown files

echo "Normalizing line endings in markdown files..."

# Detect platform and set correct sed -i syntax
if [[ "$(uname)" == "Darwin" ]]; then
    SED_INPLACE="sed -i ''"
else
    SED_INPLACE="sed -i"
fi

# Find all markdown files and convert CRLF to LF
find . -name "*.md" -type f -exec $SED_INPLACE 's/\r$//' {} \;

echo "Done! All markdown files now have LF line endings."