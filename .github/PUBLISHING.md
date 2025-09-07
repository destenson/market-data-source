# Trusted Publishing Configuration

This document outlines the trusted publishing setup for automated releases to crates.io and PyPI using OpenID Connect (OIDC).

## Overview

Market Data Source uses GitHub's OIDC provider to authenticate with package registries, eliminating the need for long-lived API tokens stored as secrets.

## Configuration Steps

### 1. Environment Protection Rules

Create a `release` environment in GitHub repository settings with the following protection rules:

1. Go to **Settings → Environments**
2. Create new environment: `release`
3. Configure protection rules:
   - **Required reviewers**: 1 (optional, for extra security)
   - **Deployment branches**: Only allow `main` or `master` branch
   - **Wait timer**: 0 minutes (or set delay if desired)

### 2. Crates.io Trusted Publishing

After the initial manual publication to establish ownership:

1. Visit https://crates.io/settings/tokens
2. Navigate to your `market-data-source` crate settings
3. Add GitHub repository as trusted publisher:
   - Repository: `destenson/market-data-source`
   - Workflow: `.github/workflows/publish.yml`
   - Environment: `release` (optional but recommended)

### 3. PyPI Trusted Publishing

Configure trusted publishing on PyPI:

1. Visit https://pypi.org/manage/project/market-data-source/settings/publishing/
2. Add GitHub repository:
   - Owner: `destenson`
   - Repository: `market-data-source`
   - Workflow: `publish.yml`
   - Environment: `release` (optional)

### 4. Test PyPI Configuration (Optional)

For testing before production:

1. Visit https://test.pypi.org/manage/project/market-data-source/settings/publishing/
2. Configure same settings as production PyPI
3. Test with `dry-run: true` in workflow dispatch

## Workflow Usage

### Manual Dry-Run Testing

Test the publishing workflow without actually publishing:

```bash
# Via GitHub UI:
# Actions → Publish Release → Run workflow → dry-run: true
```

### Automated Release Publishing

Create and push a version tag to trigger automatic publishing:

```bash
# Update version in Cargo.toml and pyproject.toml
# Commit changes
git add .
git commit -m "Release version 0.3.1"

# Create and push tag
git tag v0.3.1
git push origin v0.3.1
```

The workflow will:
1. Validate version consistency across files
2. Run full test suite
3. Build release artifacts
4. Publish to crates.io using OIDC
5. Build Python wheels for all platforms
6. Publish to PyPI using OIDC
7. Create GitHub release with changelog

## Security Best Practices

### Branch Protection

Configure branch protection for `main`/`master`:

1. **Settings → Branches → Add rule**
2. Branch name pattern: `main` or `master`
3. Enable:
   - Require pull request reviews
   - Dismiss stale pull request approvals
   - Require status checks to pass
   - Require branches to be up to date
   - Include administrators

### Required Status Checks

Add these workflows as required status checks:
- `test / Test Suite`
- `quality / Code Quality`
- `python-test / Python Tests`

### Environment Secrets

**No secrets required!** OIDC eliminates the need for:
- `CARGO_REGISTRY_TOKEN`
- `PYPI_API_TOKEN`
- `TWINE_PASSWORD`

## Monitoring and Alerts

### Publication Monitoring

1. Watch workflow runs: https://github.com/destenson/market-data-source/actions
2. Monitor package pages:
   - https://crates.io/crates/market-data-source
   - https://pypi.org/project/market-data-source/

### Failure Recovery

If publication fails:

1. Check workflow logs for specific errors
2. Verify OIDC configuration on registry websites
3. Ensure version hasn't been previously published
4. For partial failures, manually publish missing components

### Emergency Manual Publishing

If OIDC fails, fall back to manual publishing:

```bash
# Crates.io (requires cargo login)
cargo publish

# PyPI (requires twine and API token)
maturin build --release
twine upload target/wheels/*.whl
```

## Troubleshooting

### Common Issues

1. **"Version already exists"**: Version was previously published
   - Solution: Bump version number and retry

2. **"Unauthorized"**: OIDC configuration mismatch
   - Solution: Verify repository and workflow names match exactly

3. **"Environment not found"**: Environment protection not configured
   - Solution: Create `release` environment in repository settings

4. **"Token exchange failed"**: OIDC provider issue
   - Solution: Check GitHub Actions permissions include `id-token: write`

### Validation Commands

Test OIDC configuration without publishing:

```yaml
# In workflow, add debug step:
- name: Test OIDC Token
  run: |
    echo "ACTIONS_ID_TOKEN_REQUEST_URL: $ACTIONS_ID_TOKEN_REQUEST_URL"
    echo "ACTIONS_ID_TOKEN_REQUEST_TOKEN exists: ${{ env.ACTIONS_ID_TOKEN_REQUEST_TOKEN != '' }}"
```

## References

- [Crates.io Trusted Publishing](https://blog.rust-lang.org/2025/01/09/crates-io-trusted-publishing.html)
- [PyPI Trusted Publishers](https://docs.pypi.org/trusted-publishers/)
- [GitHub OIDC Documentation](https://docs.github.com/en/actions/deployment/security-hardening-your-deployments/about-security-hardening-with-openid-connect)
- [rust-lang/crates-io-auth-action](https://github.com/rust-lang/crates-io-auth-action)
- [pypa/gh-action-pypi-publish](https://github.com/pypa/gh-action-pypi-publish)