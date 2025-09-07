#!/bin/bash
# Version synchronization script for market-data-source
# Ensures version consistency between Cargo.toml and pyproject.toml

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Get the script directory and project root
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

# File paths
CARGO_TOML="$PROJECT_ROOT/Cargo.toml"
PYPROJECT_TOML="$PROJECT_ROOT/pyproject.toml"

# Function to extract version from Cargo.toml
get_cargo_version() {
    grep '^version' "$CARGO_TOML" | head -1 | cut -d'"' -f2
}

# Function to extract version from pyproject.toml
get_pyproject_version() {
    grep '^version' "$PYPROJECT_TOML" | head -1 | cut -d'"' -f2
}

# Function to update version in pyproject.toml
update_pyproject_version() {
    local new_version="$1"
    if [[ "$OSTYPE" == "darwin"* ]]; then
        # macOS
        sed -i '' "s/^version = \".*\"/version = \"$new_version\"/" "$PYPROJECT_TOML"
    else
        # Linux
        sed -i "s/^version = \".*\"/version = \"$new_version\"/" "$PYPROJECT_TOML"
    fi
}

# Function to update version in Cargo.toml
update_cargo_version() {
    local new_version="$1"
    if [[ "$OSTYPE" == "darwin"* ]]; then
        # macOS
        sed -i '' "s/^version = \".*\"/version = \"$new_version\"/" "$CARGO_TOML"
    else
        # Linux
        sed -i "s/^version = \".*\"/version = \"$new_version\"/" "$CARGO_TOML"
    fi
}

# Parse command line arguments
CHECK_MODE=false
SET_VERSION=""

while [[ $# -gt 0 ]]; do
    case $1 in
        --check)
            CHECK_MODE=true
            shift
            ;;
        --set-version)
            SET_VERSION="$2"
            shift 2
            ;;
        --help|-h)
            echo "Usage: $0 [OPTIONS]"
            echo ""
            echo "Options:"
            echo "  --check           Check version consistency without making changes"
            echo "  --set-version VER Set a specific version in both files"
            echo "  --help, -h        Show this help message"
            echo ""
            echo "Default behavior: Sync version from Cargo.toml to pyproject.toml"
            exit 0
            ;;
        *)
            echo -e "${RED}Unknown option: $1${NC}"
            echo "Use --help for usage information"
            exit 1
            ;;
    esac
done

# Check if files exist
if [ ! -f "$CARGO_TOML" ]; then
    echo -e "${RED}Error: $CARGO_TOML not found${NC}"
    exit 1
fi

if [ ! -f "$PYPROJECT_TOML" ]; then
    echo -e "${RED}Error: $PYPROJECT_TOML not found${NC}"
    exit 1
fi

# Execute based on mode
if [ "$CHECK_MODE" = true ]; then
    # Check mode - verify consistency
    cargo_version=$(get_cargo_version)
    pyproject_version=$(get_pyproject_version)
    
    echo "Cargo.toml version: $cargo_version"
    echo "pyproject.toml version: $pyproject_version"
    
    if [ "$cargo_version" = "$pyproject_version" ]; then
        echo -e "${GREEN}✓ Versions are consistent${NC}"
        exit 0
    else
        echo -e "${RED}✗ Version mismatch detected!${NC}"
        exit 1
    fi
    
elif [ -n "$SET_VERSION" ]; then
    # Set a specific version in both files
    # Validate version format (basic semver)
    if ! echo "$SET_VERSION" | grep -qE '^[0-9]+\.[0-9]+\.[0-9]+(-[a-zA-Z0-9\.]+)?(\+[a-zA-Z0-9\.]+)?$'; then
        echo -e "${RED}Error: Invalid version format: $SET_VERSION${NC}"
        echo "Expected format: MAJOR.MINOR.PATCH[-prerelease][+build]"
        exit 1
    fi
    
    echo "Setting version to $SET_VERSION in all files..."
    
    # Update both files
    update_cargo_version "$SET_VERSION"
    echo -e "${GREEN}✓ Updated $CARGO_TOML${NC}"
    
    update_pyproject_version "$SET_VERSION"
    echo -e "${GREEN}✓ Updated $PYPROJECT_TOML${NC}"
    
    # Verify the changes
    cargo_version=$(get_cargo_version)
    pyproject_version=$(get_pyproject_version)
    
    if [ "$cargo_version" = "$SET_VERSION" ] && [ "$pyproject_version" = "$SET_VERSION" ]; then
        echo -e "${GREEN}✓ Successfully set version to $SET_VERSION${NC}"
    else
        echo -e "${RED}✗ Failed to update versions consistently${NC}"
        exit 1
    fi
    
else
    # Default mode - sync from Cargo.toml to pyproject.toml
    cargo_version=$(get_cargo_version)
    pyproject_version=$(get_pyproject_version)
    
    echo "Cargo.toml version: $cargo_version"
    echo "Current pyproject.toml version: $pyproject_version"
    
    if [ "$cargo_version" = "$pyproject_version" ]; then
        echo -e "${GREEN}  Versions are already synchronized${NC}"
    else
        update_pyproject_version "$cargo_version"
        echo -e "${GREEN}✓ Updated pyproject.toml to version $cargo_version${NC}"
        
        # Verify the sync
        new_pyproject_version=$(get_pyproject_version)
        if [ "$cargo_version" = "$new_pyproject_version" ]; then
            echo -e "${GREEN}✓ Versions are now synchronized${NC}"
        else
            echo -e "${RED}✗ Failed to synchronize versions${NC}"
            exit 1
        fi
    fi
fi