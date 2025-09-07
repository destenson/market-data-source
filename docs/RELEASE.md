# Release Process Documentation

This document describes the release process for market-data-source, including version management, validation, and publication to both crates.io and PyPI.

## Table of Contents

- [Overview](#overview)
- [Version Management](#version-management)
- [Release Types](#release-types)
- [Release Workflow](#release-workflow)
- [Manual Release Process](#manual-release-process)
- [Automated Release Process](#automated-release-process)
- [Troubleshooting](#troubleshooting)
- [Environment Configuration](#environment-configuration)

## Overview

The market-data-source project follows semantic versioning (MAJOR.MINOR.PATCH) and maintains synchronized versions across both Rust (crates.io) and Python (PyPI) packages.

### Key Components

- **Rust Package**: Published to crates.io as `market-data-source`
- **Python Package**: Published to PyPI as `market-data-source`
- **Release Automation**: GitHub Actions workflows for automated testing and publishing
- **Version Sync**: Scripts to ensure version consistency across all configuration files

## Version Management

### Version Format

We follow [Semantic Versioning 2.0.0](https://semver.org/):

- **MAJOR**: Breaking API changes
- **MINOR**: New features, backwards compatible
- **PATCH**: Bug fixes, backwards compatible
- **Pre-release**: Optional (e.g., `0.3.1-alpha.1`)
- **Build metadata**: Optional (e.g., `0.3.1+build.123`)

### Version Sources

The project maintains version information in two files:

1. `Cargo.toml` - Rust package version (authoritative source)
2. `pyproject.toml` - Python package version (synchronized from Cargo.toml)

### Version Synchronization

Use the version sync script to manage versions:

```bash
# Check version consistency
python scripts/sync-version.py --check

# Sync pyproject.toml from Cargo.toml
python scripts/sync-version.py

# Set a specific version in both files
python scripts/sync-version.py --set-version 0.3.1
```

## Release Types

### Patch Release (0.3.0 → 0.3.1)

For bug fixes and minor improvements:

```bash
python scripts/prepare-release.py --bump patch
```

### Minor Release (0.3.1 → 0.4.0)

For new features and non-breaking changes:

```bash
python scripts/prepare-release.py --bump minor
```

### Major Release (0.4.0 → 1.0.0)

For breaking changes:

```bash
python scripts/prepare-release.py --bump major
```

### Pre-release

For testing and preview releases:

```bash
python scripts/prepare-release.py --version 0.3.1-alpha.1
```

## Release Workflow

### Prerequisites

1. **Clean Working Directory**: Ensure all changes are committed
2. **Updated CHANGELOG**: Document all changes in CHANGELOG.md
3. **Passing Tests**: All tests must pass
4. **Version Consistency**: Versions must match across files

### Quick Release Process

For a standard release:

```bash
# 1. Prepare the release (updates version, runs tests)
python scripts/prepare-release.py --bump patch

# 2. Review and update CHANGELOG.md with actual changes
# Edit CHANGELOG.md manually

# 3. Commit the changes
git add -A
git commit -m "chore: prepare release v0.3.1"

# 4. Push the tag to trigger automated release
git push origin v0.3.1

# 5. Monitor the release workflow
# Go to GitHub Actions to watch the progress
```

## Manual Release Process

### Step 1: Version Preparation

```bash
# Update version in both files
python scripts/sync-version.py --set-version 0.3.1

# Verify version consistency
python scripts/sync-version.py --check
```

### Step 2: Quality Validation

```bash
# Run all tests
cargo test --all-features

# Check formatting
cargo fmt --all -- --check

# Run clippy
cargo clippy --all-features --all-targets -- -D warnings

# Build documentation
cargo doc --no-deps --all-features

# Test Python package build
cd market-data-source-python
maturin build --release
cd ..
```

### Step 3: Update Documentation

1. Update CHANGELOG.md with release notes
2. Update README.md if needed
3. Update version in examples if applicable

### Step 4: Create Git Tag

```bash
# Commit all changes
git add -A
git commit -m "chore: prepare release v0.3.1"

# Create annotated tag
git tag -a v0.3.1 -m "Release version 0.3.1"
```

### Step 5: Push and Publish

```bash
# Push commits and tag
git push origin main
git push origin v0.3.1
```

The automated workflow will handle publishing to both registries.

## Automated Release Process

### GitHub Actions Workflow

The release is automated through GitHub Actions when a version tag is pushed:

1. **Validation Phase**
   - Version consistency check
   - Extract changelog entries
   - Validate semantic version format

2. **Quality Gates**
   - Run tests on all platforms (Linux, Windows, macOS)
   - Check code formatting
   - Run clippy lints
   - Build documentation

3. **Build Phase**
   - Build Rust binaries for all targets
   - Build Python wheels for all Python versions
   - Package release artifacts

4. **Publication Phase**
   - Publish to crates.io (using OIDC trusted publishing)
   - Publish to PyPI (using OIDC trusted publishing)
   - Create GitHub release with artifacts

5. **Post-Release**
   - Create PR for next development version
   - Generate release summary
   - Update documentation

### Triggering Automated Release

```bash
# Standard release
git push origin v0.3.1

# Or manually trigger with dry-run
# Go to GitHub Actions > Release Automation > Run workflow
# Set version: 0.3.1
# Set dry-run: true (for testing)
```

### Monitoring Release Progress

1. Go to GitHub Actions tab
2. Watch the "Automated Release Pipeline" workflow
3. Check each phase for successful completion
4. Review the release summary

## Troubleshooting

### Common Issues

#### Version Mismatch

```bash
# Fix version mismatch
python scripts/sync-version.py
git add Cargo.toml pyproject.toml
git commit -m "fix: sync versions"
```

#### Failed Tests

```bash
# Run tests locally to debug
cargo test --all-features --verbose

# Check specific test
cargo test test_name -- --nocapture
```

#### Build Failures on Windows

```bash
# Use single-threaded build
cargo build -j 1
```

#### Python Wheel Build Issues

```bash
# Ensure maturin is installed
pip install maturin

# Build with verbose output
cd market-data-source-python
maturin build --release --verbose
```

### Rollback Procedure

If a release fails:

1. **Delete the tag locally**:
   ```bash
   git tag -d v0.3.1
   ```

2. **Delete the tag remotely**:
   ```bash
   git push origin :refs/tags/v0.3.1
   ```

3. **Fix issues and retry**

### Manual Publication (Emergency)

If automation fails, publish manually:

#### Publish to crates.io

```bash
cargo publish --all-features
```

#### Publish to PyPI

```bash
cd market-data-source-python
maturin build --release
twine upload ../dist/*.whl
```

## Environment Configuration

### GitHub Repository Settings

Required repository secrets and variables:

1. **Environments**: Configure `release` environment with:
   - Required reviewers (optional)
   - Environment protection rules
   - Deployment branches: Protected branches only

2. **OIDC Configuration**: Already configured for:
   - crates.io trusted publishing
   - PyPI trusted publishing

### Local Development Setup

#### Pre-commit Hooks

Install pre-commit hooks for version consistency:

```bash
# Install pre-commit
pip install pre-commit

# Install hooks
pre-commit install

# Run manually
pre-commit run --all-files
```

#### Required Tools

- Rust toolchain (stable)
- Python 3.8+
- maturin (`pip install maturin`)
- twine (`pip install twine`)
- pre-commit (`pip install pre-commit`)

## Release Checklist

Before releasing, ensure:

- [ ] All tests pass (`cargo test --all-features`)
- [ ] No clippy warnings (`cargo clippy --all-features -- -D warnings`)
- [ ] Code is formatted (`cargo fmt --all -- --check`)
- [ ] Versions are synchronized (`python scripts/sync-version.py --check`)
- [ ] CHANGELOG.md is updated with release notes
- [ ] Documentation is up to date
- [ ] Examples work with new version
- [ ] Git working directory is clean
- [ ] Pre-release testing completed (if major release)

## Release Schedule

- **Patch releases**: As needed for bug fixes
- **Minor releases**: Monthly or when features are ready
- **Major releases**: Planned with breaking changes
- **Pre-releases**: Before major releases for testing

## Support

For release-related issues:

1. Check the [troubleshooting section](#troubleshooting)
2. Review GitHub Actions logs
3. Open an issue with the `release` label
4. Contact maintainers for urgent issues

## Additional Resources

- [Semantic Versioning](https://semver.org/)
- [GitHub Actions Documentation](https://docs.github.com/en/actions)
- [crates.io Publishing Guide](https://doc.rust-lang.org/cargo/reference/publishing.html)
- [PyPI Publishing Guide](https://packaging.python.org/tutorials/packaging-projects/)
- [Trusted Publishing Setup](https://docs.pypi.org/trusted-publishers/)