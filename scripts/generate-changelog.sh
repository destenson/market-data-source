#!/bin/bash
# Unix/Linux script to generate CHANGELOG.md

echo "Generating CHANGELOG.md from git history..."

# Get the directory where this script is located
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

# Run the Python script
python3 "$SCRIPT_DIR/generate-changelog.py" "$@"

if [ $? -ne 0 ]; then
    echo ""
    echo "Error: Failed to generate changelog."
    echo "Make sure Python 3 is installed and you're in a git repository."
    exit 1
fi

echo ""
echo "Changelog generation complete!"