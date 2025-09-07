#!/usr/bin/env python3
"""
Version synchronization script for market-data-source.
Ensures version consistency between Cargo.toml and pyproject.toml.
Uses Cargo.toml as the single source of truth.
"""

import re
import sys
from pathlib import Path
import argparse
import tomllib
import subprocess


def get_cargo_version(cargo_path: Path) -> str:
    """Extract version from Cargo.toml."""
    with open(cargo_path, 'r') as f:
        content = f.read()
    
    # Match version = "x.y.z" pattern
    match = re.search(r'^version\s*=\s*"([^"]+)"', content, re.MULTILINE)
    if not match:
        raise ValueError(f"Could not find version in {cargo_path}")
    
    return match.group(1)


def update_pyproject_version(pyproject_path: Path, new_version: str) -> bool:
    """Update version in pyproject.toml."""
    with open(pyproject_path, 'r') as f:
        content = f.read()
    
    # Match version = "x.y.z" pattern
    old_content = content
    content = re.sub(
        r'^version\s*=\s*"[^"]+"',
        f'version = "{new_version}"',
        content,
        count=1,
        flags=re.MULTILINE
    )
    
    if old_content == content:
        return False  # No changes made
    
    with open(pyproject_path, 'w') as f:
        f.write(content)
    
    return True


def update_workspace_cargo_version(cargo_path: Path, new_version: str) -> bool:
    """Update version in workspace Cargo.toml."""
    with open(cargo_path, 'r') as f:
        content = f.read()
    
    # Find [workspace.package] section and update version within it
    old_content = content
    
    # Split content into lines for processing
    lines = content.split('\n')
    in_workspace_package = False
    updated = False
    
    for i, line in enumerate(lines):
        if line.strip() == '[workspace.package]':
            in_workspace_package = True
        elif line.strip().startswith('[') and in_workspace_package:
            in_workspace_package = False
        elif in_workspace_package and line.strip().startswith('version'):
            lines[i] = f'version = "{new_version}"'
            updated = True
            break
    
    if not updated:
        # Fallback to simple regex replacement
        content = re.sub(
            r'^version\s*=\s*"[^"]+"',
            f'version = "{new_version}"',
            content,
            count=1,
            flags=re.MULTILINE
        )
    else:
        content = '\n'.join(lines)
    
    if old_content == content:
        return False
    
    with open(cargo_path, 'w') as f:
        f.write(content)
    
    return True


def verify_versions(root_dir: Path) -> dict:
    """Verify version consistency across all files."""
    cargo_path = root_dir / "Cargo.toml"
    pyproject_path = root_dir / "pyproject.toml"
    
    cargo_version = get_cargo_version(cargo_path)
    
    # Check pyproject.toml
    with open(pyproject_path, 'r') as f:
        content = f.read()
    match = re.search(r'^version\s*=\s*"([^"]+)"', content, re.MULTILINE)
    pyproject_version = match.group(1) if match else None
    
    return {
        'cargo': cargo_version,
        'pyproject': pyproject_version,
        'consistent': cargo_version == pyproject_version
    }


def main():
    parser = argparse.ArgumentParser(
        description='Synchronize version across Cargo.toml and pyproject.toml'
    )
    parser.add_argument(
        '--check',
        action='store_true',
        help='Check version consistency without making changes'
    )
    parser.add_argument(
        '--set-version',
        type=str,
        help='Set a specific version in both files'
    )
    parser.add_argument(
        '--root',
        type=Path,
        default=Path(__file__).parent.parent,
        help='Root directory of the project'
    )
    
    args = parser.parse_args()
    
    cargo_path = args.root / "Cargo.toml"
    pyproject_path = args.root / "pyproject.toml"
    
    if not cargo_path.exists():
        print(f"Error: {cargo_path} not found", file=sys.stderr)
        sys.exit(1)
    
    if not pyproject_path.exists():
        print(f"Error: {pyproject_path} not found", file=sys.stderr)
        sys.exit(1)
    
    if args.check:
        # Check mode - verify consistency
        versions = verify_versions(args.root)
        print(f"Cargo.toml version: {versions['cargo']}")
        print(f"pyproject.toml version: {versions['pyproject']}")
        
        if versions['consistent']:
            print("[OK] Versions are consistent")
            sys.exit(0)
        else:
            print("[ERROR] Version mismatch detected!", file=sys.stderr)
            sys.exit(1)
    
    elif args.set_version:
        # Set a specific version in both files
        new_version = args.set_version
        
        # Validate version format (basic semver)
        if not re.match(r'^\d+\.\d+\.\d+(-[\w\.]+)?(\+[\w\.]+)?$', new_version):
            print(f"Error: Invalid version format: {new_version}", file=sys.stderr)
            print("Expected format: MAJOR.MINOR.PATCH[-prerelease][+build]", file=sys.stderr)
            sys.exit(1)
        
        print(f"Setting version to {new_version} in all files...")
        
        # Update Cargo.toml
        if update_workspace_cargo_version(cargo_path, new_version):
            print(f"[OK] Updated {cargo_path}")
        else:
            print(f"  No changes needed in {cargo_path}")
        
        # Update pyproject.toml
        if update_pyproject_version(pyproject_path, new_version):
            print(f"[OK] Updated {pyproject_path}")
        else:
            print(f"  No changes needed in {pyproject_path}")
        
        # Verify the changes
        versions = verify_versions(args.root)
        if versions['consistent']:
            print(f"[OK] Successfully set version to {new_version}")
        else:
            print("[ERROR] Failed to update versions consistently", file=sys.stderr)
            sys.exit(1)
    
    else:
        # Default mode - sync from Cargo.toml to pyproject.toml
        cargo_version = get_cargo_version(cargo_path)
        print(f"Cargo.toml version: {cargo_version}")
        
        if update_pyproject_version(pyproject_path, cargo_version):
            print(f"[OK] Updated pyproject.toml to version {cargo_version}")
        else:
            print("  pyproject.toml already has the correct version")
        
        # Verify the sync
        versions = verify_versions(args.root)
        if versions['consistent']:
            print("[OK] Versions are now synchronized")
        else:
            print("[ERROR] Failed to synchronize versions", file=sys.stderr)
            sys.exit(1)


if __name__ == '__main__':
    main()