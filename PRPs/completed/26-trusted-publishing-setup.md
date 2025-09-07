# PRP-26: Trusted Publishing Configuration for Secure Automated Releases

## Context & Motivation

**2025 Security Standard**: Both crates.io and PyPI now support "Trusted Publishing" via OpenID Connect (OIDC), eliminating the need for managing API tokens as GitHub Actions secrets.

**Security Benefit**: Short-lived tokens (30 minutes) issued via OIDC are significantly more secure than permanent API tokens stored as secrets.

**Implementation Requirement**: Must complete PRP-25 (CI/CD Foundation) first, as trusted publishing integrates with GitHub Actions workflows.

## Requirements

### Crates.io Trusted Publishing (New in 2025)
1. **Initial manual publish**: First release must be published manually to establish crate ownership
2. **Repository linking**: Configure GitHub repository trust on crates.io via web interface  
3. **OIDC workflow integration**: Use rust-lang/crates-io-auth-action for token exchange

### PyPI Trusted Publishing (Established)
1. **Repository configuration**: Link GitHub repository on PyPI project settings
2. **Workflow integration**: Configure publishing workflow with OIDC permissions
3. **Environment protection**: Optional security enhancement for production releases

### Security Configuration
1. **Minimal permissions**: Grant only id-token: write to workflows
2. **Environment restrictions**: Limit publishing to specific environments
3. **Branch protection**: Ensure only protected branches can trigger publishes

## Implementation Blueprint

### Phase 1: Initial Setup Requirements
1. Complete manual publication of version 0.3.0 to establish package ownership
2. Access crates.io and PyPI web interfaces to configure trusted repositories
3. Document the linking process for future reference

### Phase 2: Workflow Configuration  
1. Create publish workflows with OIDC token exchange
2. Configure environment permissions and restrictions
3. Test token exchange without actual publishing (dry-run mode)

### Phase 3: Security Hardening
1. Implement environment protection rules
2. Configure branch restrictions for publishing
3. Add monitoring and logging for publish attempts

## Success Criteria

### Validation Gates
```bash
# Test crates.io trusted publishing (dry-run)
# Run in GitHub Actions environment:
- uses: rust-lang/crates-io-auth-action@v1
  id: auth
- run: cargo publish --dry-run
  env:
    CARGO_REGISTRY_TOKEN: ${{ steps.auth.outputs.token }}

# Test PyPI trusted publishing (dry-run)  
# Run with PyPI workflow:
- uses: pypa/gh-action-pypi-publish@release/v1
  with:
    repository-url: https://test.pypi.org/legacy/
```

### Security Verification
- [ ] No API tokens stored as GitHub secrets
- [ ] Publishing workflows only trigger on protected branches
- [ ] OIDC token exchange succeeds in CI environment
- [ ] Environment protection rules active for production publishing
- [ ] Audit logs available for all publishing attempts

## Dependencies & References

**2025 Documentation**:
- Crates.io trusted publishing: https://crates.io/docs/trusted-publishing
- RFC #3691: https://rust-lang.github.io/rfcs/3691-trusted-publishing-cratesio.html
- PyPI trusted publishing: https://docs.pypi.org/trusted-publishers/

**GitHub Actions**:
- rust-lang/crates-io-auth-action@v1
- pypa/gh-action-pypi-publish@release/v1

**Prerequisites**:
- PRP-25: CI/CD Foundation must be completed
- Manual publication must be completed first for both registries
- Repository ownership verified on both platforms

## Implementation Tasks

### Pre-Configuration Steps
1. Publish version 0.3.0 manually to crates.io to establish ownership
2. Publish version 0.3.0 manually to PyPI to establish ownership
3. Verify package pages are accessible and properly configured

### Crates.io Trusted Publishing Setup
1. Access crates.io package settings page
2. Navigate to "Trusted Publishers" section  
3. Add GitHub repository: destenson/market-data-source
4. Configure workflow and environment restrictions
5. Document the configuration for future reference

### PyPI Trusted Publishing Setup
1. Access PyPI project settings for market-data-source
2. Navigate to "Publishing" section
3. Configure GitHub repository and workflow permissions
4. Set up environment restrictions if desired
5. Test configuration with test.pypi.org first

### Workflow Integration
1. Create `.github/workflows/publish.yml` with OIDC permissions
2. Add crates.io publishing job with rust-lang/crates-io-auth-action
3. Add PyPI publishing job with pypa/gh-action-pypi-publish
4. Configure jobs to run only on release tags
5. Test workflows with dry-run mode before live publishing

### Security Hardening
1. Create "release" environment with protection rules
2. Require review for release environment deployment
3. Restrict publishing workflows to main/master branch only
4. Add monitoring for failed authentication attempts
5. Document emergency procedures for revoked access

## Workflow Template Structure
```yaml
name: Publish Release
on:
  push:
    tags: ['v*']
jobs:
  publish-crates:
    runs-on: ubuntu-latest
    environment: release
    permissions:
      id-token: write
    steps:
      - uses: actions/checkout@v4
      - uses: rust-lang/crates-io-auth-action@v1
        id: auth
      - run: cargo publish
        env:
          CARGO_REGISTRY_TOKEN: ${{ steps.auth.outputs.token }}
  
  publish-pypi:
    runs-on: ubuntu-latest  
    environment: release
    permissions:
      id-token: write
    steps:
      - uses: actions/checkout@v4
      - name: Build wheel
        run: uv run maturin build --release
      - uses: pypa/gh-action-pypi-publish@release/v1
        with:
          packages-dir: target/wheels/
```

## Estimated Effort
**4-6 hours** (setup, configuration, testing, documentation)

## Risk Mitigation
- Test with test.pypi.org before production PyPI
- Use `cargo publish --dry-run` before real publishing
- Maintain manual publishing capability as backup
- Document rollback procedures for configuration issues
- Start with less restrictive environment rules, tighten gradually

## Success Score
**7/10** - New technology (2025) with good documentation but requires manual setup steps and external service configuration.