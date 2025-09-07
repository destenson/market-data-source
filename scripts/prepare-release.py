#!/usr/bin/env python3
"""
Release preparation script for market-data-source.
Automates the release process including version bumping, validation, and tagging.
"""

import argparse
import subprocess
import sys
import re
from pathlib import Path
from datetime import datetime
import json


class ReleasePreparation:
    def __init__(self, root_dir: Path):
        self.root_dir = root_dir
        self.cargo_toml = root_dir / "Cargo.toml"
        self.pyproject_toml = root_dir / "pyproject.toml"
        self.changelog = root_dir / "CHANGELOG.md"
        self.errors = []
        
    def run_command(self, cmd: list, check=True, capture_output=True) -> subprocess.CompletedProcess:
        """Run a shell command and return the result."""
        print(f"Running: {' '.join(cmd)}")
        result = subprocess.run(
            cmd,
            check=check,
            capture_output=capture_output,
            text=True,
            cwd=self.root_dir
        )
        return result
    
    def get_current_version(self) -> str:
        """Get the current version from Cargo.toml."""
        with open(self.cargo_toml, 'r') as f:
            content = f.read()
        match = re.search(r'^version\s*=\s*"([^"]+)"', content, re.MULTILINE)
        if not match:
            raise ValueError("Could not find version in Cargo.toml")
        return match.group(1)
    
    def bump_version(self, version: str, bump_type: str) -> str:
        """Bump the version based on the bump type."""
        parts = version.split('.')
        if len(parts) != 3:
            raise ValueError(f"Invalid version format: {version}")
        
        major, minor, patch = map(int, parts)
        
        if bump_type == 'major':
            return f"{major + 1}.0.0"
        elif bump_type == 'minor':
            return f"{major}.{minor + 1}.0"
        elif bump_type == 'patch':
            return f"{major}.{minor}.{patch + 1}"
        else:
            raise ValueError(f"Invalid bump type: {bump_type}")
    
    def validate_code_quality(self) -> bool:
        """Run all code quality checks."""
        print("\nRunning code quality checks...")
        
        checks = [
            ("Cargo format check", ["cargo", "fmt", "--all", "--", "--check"]),
            ("Cargo clippy", ["cargo", "clippy", "--all-features", "--all-targets", "--", "-D", "warnings"]),
            ("Cargo test", ["cargo", "test", "--all-features"]),
            ("Cargo doc", ["cargo", "doc", "--no-deps", "--all-features"]),
        ]
        
        all_passed = True
        for name, cmd in checks:
            print(f"\n  > {name}...")
            try:
                self.run_command(cmd)
                print(f"  [OK] {name} passed")
            except subprocess.CalledProcessError as e:
                print(f"  [ERROR] {name} failed")
                self.errors.append(f"{name} failed: {e}")
                all_passed = False
        
        return all_passed
    
    def validate_python_package(self) -> bool:
        """Validate Python package can be built."""
        print("\nValidating Python package...")
        
        try:
            # Check if maturin is available
            self.run_command(["python", "-m", "pip", "show", "maturin"], check=False)
            
            # Try to build the Python package
            print("  > Building Python wheel...")
            result = self.run_command(
                ["maturin", "build", "--release", "-o", "dist"],
                cwd=self.root_dir / "market-data-source-python",
                check=False
            )
            
            if result.returncode == 0:
                print("  [OK] Python package build successful")
                return True
            else:
                print("  [WARNING] Python package build failed (non-critical)")
                return True  # Non-critical for now
                
        except Exception as e:
            print(f"  [WARNING] Could not validate Python package: {e}")
            return True  # Non-critical
    
    def validate_version_consistency(self) -> bool:
        """Check that versions are consistent across all files."""
        print("\nChecking version consistency...")
        
        # Run the sync-version script in check mode
        sync_script = self.root_dir / "scripts" / "sync-version.py"
        if sync_script.exists():
            try:
                self.run_command(["python", str(sync_script), "--check"])
                print("  [OK] Versions are consistent")
                return True
            except subprocess.CalledProcessError:
                print("  [ERROR] Version mismatch detected")
                self.errors.append("Version mismatch between Cargo.toml and pyproject.toml")
                return False
        else:
            print("  [WARNING] Version sync script not found, skipping check")
            return True
    
    def check_git_status(self) -> bool:
        """Check if git working directory is clean."""
        print("\nChecking git status...")
        
        result = self.run_command(["git", "status", "--porcelain"], check=False)
        if result.stdout.strip():
            print("  [WARNING] Uncommitted changes detected:")
            print(result.stdout)
            return False
        
        print("  [OK] Working directory is clean")
        return True
    
    def update_changelog(self, new_version: str, dry_run: bool = False) -> bool:
        """Update CHANGELOG.md with the new version."""
        print(f"\nUpdating CHANGELOG.md for version {new_version}...")
        
        if not self.changelog.exists():
            print("  [WARNING] CHANGELOG.md not found")
            return True
        
        with open(self.changelog, 'r') as f:
            content = f.read()
        
        # Check if version already exists
        if f"## [{new_version}]" in content:
            print(f"  [INFO] Version {new_version} already in CHANGELOG.md")
            return True
        
        # Add new version section
        today = datetime.now().strftime("%Y-%m-%d")
        new_section = f"\n## [{new_version}] - {today}\n\n### Added\n\n### Changed\n\n### Fixed\n\n"
        
        # Insert after the header
        lines = content.split('\n')
        insert_index = 0
        for i, line in enumerate(lines):
            if line.startswith('## '):
                insert_index = i
                break
        
        if not dry_run:
            lines.insert(insert_index, new_section)
            with open(self.changelog, 'w') as f:
                f.write('\n'.join(lines))
            print(f"  [OK] Added section for version {new_version}")
        else:
            print(f"  [DRY-RUN] Would add section for version {new_version}")
        
        return True
    
    def create_git_tag(self, version: str, dry_run: bool = False) -> bool:
        """Create a git tag for the release."""
        tag = f"v{version}"
        print(f"\nCreating git tag {tag}...")
        
        # Check if tag already exists
        result = self.run_command(["git", "tag", "-l", tag], check=False)
        if result.stdout.strip():
            print(f"  [WARNING] Tag {tag} already exists")
            return False
        
        if not dry_run:
            # Create annotated tag
            message = f"Release version {version}"
            self.run_command(["git", "tag", "-a", tag, "-m", message])
            print(f"  [OK] Created tag {tag}")
        else:
            print(f"  [DRY-RUN] Would create tag {tag}")
        
        return True
    
    def prepare_release(self, version: str = None, bump_type: str = None, dry_run: bool = False):
        """Main release preparation workflow."""
        print("Starting release preparation...\n")
        
        # Determine new version
        current_version = self.get_current_version()
        print(f"Current version: {current_version}")
        
        if version:
            new_version = version
        elif bump_type:
            new_version = self.bump_version(current_version, bump_type)
        else:
            new_version = current_version
        
        print(f"Preparing release for version: {new_version}\n")
        
        # Run validation checks
        if not self.validate_code_quality():
            print("\n[ERROR] Code quality checks failed. Please fix issues before releasing.")
            return False
        
        if not self.validate_python_package():
            print("\n[WARNING] Python package validation had issues (continuing)")
        
        # Update version if needed
        if new_version != current_version:
            print(f"\nUpdating version from {current_version} to {new_version}...")
            
            if not dry_run:
                # Use sync-version script to update versions
                sync_script = self.root_dir / "scripts" / "sync-version.py"
                if sync_script.exists():
                    self.run_command(["python", str(sync_script), "--set-version", new_version])
                    print(f"  [OK] Updated version to {new_version}")
                else:
                    print("  [ERROR] Version sync script not found")
                    return False
            else:
                print(f"  [DRY-RUN] Would update version to {new_version}")
        
        if not self.validate_version_consistency():
            print("\n[ERROR] Version consistency check failed")
            return False
        
        # Update changelog
        self.update_changelog(new_version, dry_run)
        
        # Check git status
        if not dry_run and not self.check_git_status():
            print("\n[WARNING] Uncommitted changes detected. Please commit changes before creating tag.")
            print("Suggested commit message:")
            print(f'  git add -A && git commit -m "chore: prepare release v{new_version}"')
            return False
        
        # Create git tag
        self.create_git_tag(new_version, dry_run)
        
        # Summary
        print("\n" + "="*50)
        print("[OK] Release preparation complete!")
        print(f"Version: {new_version}")
        
        if dry_run:
            print("\n[DRY-RUN] This was a dry run. No changes were made.")
        else:
            print("\nNext steps:")
            print(f"  1. Review and update CHANGELOG.md with actual changes")
            print(f"  2. Commit any remaining changes")
            print(f"  3. Push the tag to trigger release workflow:")
            print(f"     git push origin v{new_version}")
            print(f"  4. Monitor the GitHub Actions workflow")
        
        return True


def main():
    parser = argparse.ArgumentParser(
        description='Prepare a new release for market-data-source'
    )
    parser.add_argument(
        '--version',
        type=str,
        help='Set a specific version for the release'
    )
    parser.add_argument(
        '--bump',
        choices=['major', 'minor', 'patch'],
        help='Bump the version (major, minor, or patch)'
    )
    parser.add_argument(
        '--dry-run',
        action='store_true',
        help='Run in dry-run mode without making changes'
    )
    parser.add_argument(
        '--skip-tests',
        action='store_true',
        help='Skip running tests (not recommended)'
    )
    parser.add_argument(
        '--root',
        type=Path,
        default=Path(__file__).parent.parent,
        help='Root directory of the project'
    )
    
    args = parser.parse_args()
    
    if args.version and args.bump:
        print("Error: Cannot specify both --version and --bump")
        sys.exit(1)
    
    prep = ReleasePreparation(args.root)
    
    try:
        success = prep.prepare_release(
            version=args.version,
            bump_type=args.bump,
            dry_run=args.dry_run
        )
        
        if not success:
            sys.exit(1)
            
    except Exception as e:
        print(f"\n[ERROR] Error during release preparation: {e}")
        sys.exit(1)


if __name__ == '__main__':
    main()