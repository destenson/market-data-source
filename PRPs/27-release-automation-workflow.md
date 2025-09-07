# PRP-27: Automated Release Workflow and Version Management

## Context & Motivation

**Integration Goal**: Combine all previous PRPs (21-26) into a cohesive, automated release workflow that ensures quality and consistency across Rust and Python package ecosystems.

**User Requirement**: Streamlined release process triggered by git tags that automatically validates, builds, and publishes to both crates.io and PyPI with proper coordination.

**Version Coordination Challenge**: Must maintain version synchronization between Cargo.toml and pyproject.toml while supporting different ecosystem conventions.

## Requirements

### Automated Release Triggers
1. **Git tag triggers**: Release workflow activates on `v*` tags (e.g., v0.3.1)
2. **Pre-release validation**: Comprehensive testing before any publication
3. **Coordinated publishing**: Ensure both packages publish or fail together
4. **Post-release tasks**: Update documentation, create GitHub release

### Version Management Strategy
1. **Single source of truth**: Establish authoritative version location
2. **Automated synchronization**: Update both package files from single source
3. **Semantic versioning**: Enforce proper version bumping practices
4. **Pre-release verification**: Validate version consistency before publication

### Quality Assurance Integration
1. **All PRP validation**: Ensure PRPs 21-26 requirements are met
2. **Package building verification**: Test both Rust and Python packages build
3. **Release notes generation**: Automated changelog and GitHub release creation
4. **Rollback procedures**: Clear process for handling failed releases

## Implementation Blueprint

### Release Preparation Phase
1. **Pre-flight checks**: Validate all quality gates from PRP-21
2. **Version synchronization**: Update both Cargo.toml and pyproject.toml versions
3. **Documentation updates**: Generate CHANGELOG entries and update README badges
4. **Build verification**: Test package building for both ecosystems

### Publication Phase
1. **Crates.io publication**: Use trusted publishing from PRP-26
2. **PyPI publication**: Coordinate Python package publishing
3. **GitHub release creation**: Automated release notes and asset attachment
4. **Documentation deployment**: Update docs.rs and any additional documentation

### Post-Release Phase  
1. **Version bumping**: Prepare for next development cycle
2. **Monitoring setup**: Track download metrics and community feedback
3. **Issue template updates**: Update bug report templates with new version
4. **Community notifications**: Announce release through appropriate channels

## Success Criteria

### Validation Gates
```bash
# Complete release validation pipeline
cargo test --all-features
cargo clippy --all-targets --all-features -- -D warnings
cargo publish --dry-run
uv run maturin build --release
uv run pytest tests/ -v

# Version consistency check
grep "version = " Cargo.toml | grep "0\.3\.1"
grep "version = " pyproject.toml | grep "0\.3\.1"

# Release readiness verification
git tag --list | grep "v0\.3\.1"
cargo package --list | grep -v "/target/"
```

### Release Success Metrics
- [ ] Both packages published successfully to their respective registries
- [ ] GitHub release created with proper changelog and assets
- [ ] Documentation updated with new version badges and examples
- [ ] Version consistency maintained across all configuration files
- [ ] Post-release monitoring dashboards show successful deployment

## Dependencies & References

**Prerequisites (All Must Be Complete)**:
- PRP-21: Code quality fixes applied
- PRP-22: Crates.io metadata configured
- PRP-23: PyPI metadata synchronized
- PRP-24: Documentation and changelog ready
- PRP-25: CI/CD foundation established
- PRP-26: Trusted publishing configured

**GitHub Actions Integration**:
- Workflow triggers on semantic version tags
- Environment-based deployment approvals
- Secure token management via OIDC

**Version Management Tools**:
- cargo-bump for automated version updates
- Python packaging tools for wheel distribution
- GitHub CLI for release automation

## Implementation Tasks

### Phase 1: Version Management Automation
1. Create version synchronization script to update both Cargo.toml and pyproject.toml
2. Add pre-commit hooks to verify version consistency
3. Document version bumping procedures for maintainers
4. Create helper scripts for preparing releases

### Phase 2: Release Workflow Creation
1. Create `.github/workflows/release.yml` combining all validation steps
2. Configure release environment with required approvals
3. Add comprehensive testing matrix for release candidates
4. Implement rollback procedures for failed releases

### Phase 3: Post-Release Automation
1. Add GitHub release creation with automated changelog
2. Configure documentation updates for new versions
3. Set up download monitoring and analytics
4. Create community notification templates

### Phase 4: Testing and Validation
1. Test release workflow with pre-release tags (e.g., v0.3.1-alpha.1)
2. Verify publishing works correctly with test registries
3. Validate rollback procedures with failed release simulation
4. Document troubleshooting procedures for common issues

## Release Workflow Template Structure
```yaml
name: Release
on:
  push:
    tags: ['v*']
    
jobs:
  validate:
    runs-on: ubuntu-latest
    steps:
      - name: Pre-flight Quality Checks
        run: # All PRP-21 validation steps
      
  publish-rust:
    needs: validate
    environment: release
    permissions:
      id-token: write
    steps:
      - name: Publish to crates.io
        # Use PRP-26 trusted publishing
        
  publish-python:
    needs: validate
    environment: release  
    permissions:
      id-token: write
    steps:
      - name: Publish to PyPI
        # Use PRP-26 trusted publishing
        
  create-release:
    needs: [publish-rust, publish-python]
    steps:
      - name: Generate Release Notes
        # Auto-generate from CHANGELOG.md
      - name: Create GitHub Release
        # Attach built packages and documentation
```

## Version Synchronization Strategy

### Single Source Implementation
1. Use Cargo.toml as authoritative version source
2. Create sync script that updates pyproject.toml from Cargo.toml
3. Add CI check to verify version consistency
4. Document version bumping workflow for contributors

### Semantic Versioning Enforcement
1. Validate tag format matches semver patterns
2. Ensure CHANGELOG.md has corresponding entries
3. Verify breaking changes are properly communicated
4. Add pre-release and build metadata support

## Estimated Effort
**2-3 days** (workflow creation, testing, documentation, validation)

## Risk Mitigation
- Test extensively with pre-release tags before production use
- Maintain manual release capability as backup
- Implement comprehensive logging and monitoring
- Create detailed troubleshooting documentation
- Plan for coordinated rollback if one ecosystem fails

## Success Score
**6/10** - Complex integration requiring coordination across multiple systems and ecosystems, but builds on solid foundation from previous PRPs.