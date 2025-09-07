# Release Next Version

This command prepares and publishes a new release of Market Data Source to both crates.io and PyPI using the automated release workflow.

**Note**: All version numbers shown (e.g., 0.3.0, 0.3.1) are examples. Replace with your actual version numbers.

## Prerequisites

Before starting, ensure you have:
- Git repository with no uncommitted changes
- Access to push tags to the GitHub repository
- The release automation workflows are configured

## Release Process

### Step 1: Verify Current State

First, check that everything is ready for release:

```bash
# Check git status - should be clean
git status

# Run all tests to ensure everything passes
cargo test --all-features

# Run clippy to check for any warnings
cargo clippy --all-features

# Check that Python bindings work
maturin develop --release
python -c "import market_data_source as mds; print(mds.__version__)"
```

### Step 2: Determine Version Number

Decide on the new version based on semantic versioning:
- **MAJOR**: Breaking changes (e.g., 0.3.0 → 1.0.0)
- **MINOR**: New features, backwards compatible (e.g., 0.3.0 → 0.4.0)
- **PATCH**: Bug fixes only (e.g., 0.3.0 → 0.3.1)

Check current version:
```bash
grep "^version" Cargo.toml
```

### Step 3: Update Version Numbers

Update the version in all necessary files:

```bash
# Option 1: Set specific version (replace X.Y.Z with your version)
python scripts/sync-version.py --set-version X.Y.Z

# Option 2: Use automatic version bumping
python scripts/prepare-release.py --bump patch  # For bug fixes
python scripts/prepare-release.py --bump minor  # For new features
python scripts/prepare-release.py --bump major  # For breaking changes

# Verify versions are synchronized
python scripts/sync-version.py --check
```

### Step 4: Update CHANGELOG

Update the CHANGELOG.md file with release notes:

```bash
# Generate changelog entries (review and edit as needed)
python scripts/generate-changelog.py

# Manually edit CHANGELOG.md to:
# - Add release date
# - Summarize key features
# - List breaking changes (if any)
# - Credit contributors
```

### Step 5: Commit Release Changes

```bash
# Stage all changes
git add -A

# Commit with release message (replace X.Y.Z with your version)
git commit -m "chore: prepare release vX.Y.Z"

# Push to main branch
git push origin main
```

### Step 6: Create and Push Release Tag

This triggers the automated release workflow:

```bash
# Create annotated tag (replace X.Y.Z with your version)
git tag -a vX.Y.Z -m "Release version X.Y.Z"

# Push the tag - THIS TRIGGERS THE RELEASE
git push origin vX.Y.Z
```

### Step 7: Monitor Release Workflow

1. Go to GitHub Actions tab in your repository
2. Watch the "Release" workflow triggered by the tag
3. The workflow will:
   - Run all tests on multiple platforms
   - Build Rust package and publish to crates.io
   - Build Python wheels for all platforms
   - Publish to PyPI
   - Create GitHub release with artifacts

### Step 8: Verify Publication

After the workflow completes successfully:

```bash
# Verify on crates.io
curl -s https://crates.io/api/v1/crates/market-data-source | grep version

# Verify on PyPI (wait a few minutes for propagation)
pip index versions market-data-source

# Test installation from crates.io
cargo install market-data-source

# Test installation from PyPI
pip install market-data-source
```

### Step 9: Post-Release Tasks

```bash
# Update version to next development version (replace X.Y.Z with next version)
python scripts/sync-version.py --set-version X.Y.Z-dev

# Commit the version bump
git add -A
git commit -m "chore: bump version to X.Y.Z-dev"
git push origin main
```

## Quick Release Command Sequence

Example for a patch release:

```bash
# Ensure clean state
git status
cargo test --all-features
cargo clippy --all-features

# Prepare release (choose one based on changes)
python scripts/prepare-release.py --bump patch  # Bug fixes
python scripts/prepare-release.py --bump minor  # New features
python scripts/prepare-release.py --bump major  # Breaking changes

# Edit CHANGELOG.md to add release notes
# Then commit and tag (replace X.Y.Z with actual version)
git add -A
git commit -m "chore: prepare release vX.Y.Z"
git tag -a vX.Y.Z -m "Release version X.Y.Z"

# Trigger release
git push origin main
git push origin vX.Y.Z

# After release, bump to dev version
python scripts/sync-version.py --set-version X.Y.Z-dev
git add -A
git commit -m "chore: bump version to X.Y.Z-dev"
git push origin main
```

## Troubleshooting

### If Tests Fail
- Fix the failing tests before proceeding
- Ensure all tests pass locally before pushing

### If Version Sync Fails
- Manually edit Cargo.toml and pyproject.toml
- Ensure versions match exactly

### If Release Workflow Fails
- Check GitHub Actions logs for specific errors
- Common issues:
  - Version already exists on crates.io/PyPI
  - Tests failing on specific platforms
  - Network timeouts (retry by deleting and recreating tag)

### If Publication Partially Succeeds
- If crates.io succeeds but PyPI fails (or vice versa):
  - Check the workflow logs for the specific failure
  - You may need to manually publish the failed component
  - For PyPI: `maturin publish`
  - For crates.io: `cargo publish`

## Manual Publication (Emergency Only)

If automated release fails completely:

```bash
# For crates.io
cargo publish

# For PyPI
maturin build --release
maturin publish
```

## Important Notes

1. **Version Format**: Use semantic versioning (MAJOR.MINOR.PATCH)
2. **Tag Format**: Always use `v` prefix (e.g., `v0.3.0`, `v1.0.0`)
3. **Branch**: Always release from the `main` branch
4. **Testing**: Never skip the test step
5. **Changelog**: Always update CHANGELOG.md before release

## Release Checklist

- [ ] All tests passing
- [ ] No clippy warnings
- [ ] Version numbers synchronized
- [ ] CHANGELOG.md updated
- [ ] Changes committed to main branch
- [ ] Tag created and pushed
- [ ] GitHub Actions workflow completed
- [ ] Package visible on crates.io
- [ ] Package visible on PyPI
- [ ] Version bumped to next dev version

## Support

If you encounter issues with the release process:
1. Check the GitHub Actions logs
2. Review [docs/RELEASE.md](docs/RELEASE.md) for detailed information
3. Open an issue if the problem persists