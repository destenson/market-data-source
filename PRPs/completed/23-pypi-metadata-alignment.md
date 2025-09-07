# PRP-23: PyPI Metadata Alignment and Version Sync

## Context & Motivation

**Critical Issue**: Version mismatch between Cargo.toml (0.3.0) and pyproject.toml (0.2.0) will cause publication conflicts and user confusion.

**2025 PyPI Requirements**: Only `name` and `version` are mandatory, but comprehensive metadata improves discoverability and adoption. PyProject.toml placeholder URLs were recently updated but version sync is still needed.

**User Impact**: Version misalignment breaks Python wheel building and creates inconsistent user experience across Rust and Python interfaces.

## Requirements

### Critical Synchronization
1. **Version alignment**: Sync pyproject.toml version with Cargo.toml (0.3.0)
2. **Metadata consistency**: Ensure description, keywords, URLs match between both files
3. **License compatibility**: Verify MIT license works for both ecosystems

### PyPI Optimization  
1. **Python version support**: Specify supported Python versions (3.8+)
2. **Classifiers enhancement**: Add comprehensive PyPI classifiers
3. **Dependencies declaration**: Ensure maturin build system is properly configured

## Implementation Blueprint

### Version Synchronization Strategy
1. Update pyproject.toml version to match Cargo.toml (0.3.0)
2. Establish single source of truth for future version bumps
3. Document version coordination process for future releases

### Metadata Harmonization
1. Align descriptions between both package managers
2. Synchronize keywords targeting same communities (finance, trading, etc.)
3. Ensure URLs point to same repository and documentation

### PyPI Enhancement Areas
1. Add comprehensive Python version classifiers (3.8-3.12)
2. Include development status and intended audience classifiers
3. Optimize for PyPI search algorithm with relevant keywords

## Success Criteria

### Validation Gates
```bash
# Test Python wheel building
uv run maturin build --release

# Verify metadata consistency
uv run maturin build --check

# Test local installation
uv pip install target/wheels/market_data_source-*.whl

# Validate Python import
python -c "import market_data_source; print(market_data_source.__version__)"
```

### Version Verification
- Cargo.toml and pyproject.toml show identical version numbers
- Python wheel filename includes correct version
- Runtime version reporting matches package version

## Dependencies & References

**PyPI Standards**:
- PyPI classifiers: https://pypi.org/classifiers/
- Python packaging guide: https://packaging.python.org/en/latest/guides/writing-pyproject-toml/

**Build System**:
- Maturin documentation: https://maturin.rs/
- PyO3 version compatibility: https://pyo3.rs/

**Existing Configuration**:
- Reference current pyproject.toml structure
- Maintain existing maturin feature configurations
- Preserve Python source directory structure

## Implementation Tasks

1. Update pyproject.toml version field to 0.3.0
2. Review and enhance project description for Python audience
3. Add comprehensive PyPI classifiers including:
   - Development Status :: 4 - Beta
   - Programming Language :: Python :: 3.8 through 3.12
   - Topic :: Office/Business :: Financial
   - Topic :: Scientific/Engineering :: Information Analysis
4. Verify maturin build system configuration
5. Test wheel building with new metadata
6. Validate local installation and import
7. Cross-check all URLs and metadata consistency
8. Document version synchronization process

## Metadata Enhancement Plan

### Python-Specific Description
Emphasize Python developer benefits:
- "High-performance financial data generation with Python bindings"
- "Rust-powered market data simulation for Python data scientists"
- "Generate unlimited OHLC candles and trading scenarios"

### Classifier Additions
```toml
classifiers = [
    "Development Status :: 4 - Beta",
    "Intended Audience :: Developers",
    "Intended Audience :: Financial and Insurance Industry", 
    "Intended Audience :: Science/Research",
    "License :: OSI Approved :: MIT License",
    "Programming Language :: Python :: 3",
    "Programming Language :: Python :: 3.8",
    "Programming Language :: Python :: 3.9", 
    "Programming Language :: Python :: 3.10",
    "Programming Language :: Python :: 3.11",
    "Programming Language :: Python :: 3.12",
    "Programming Language :: Rust",
    "Topic :: Office/Business :: Financial",
    "Topic :: Office/Business :: Financial :: Investment",
    "Topic :: Scientific/Engineering :: Information Analysis",
    "Topic :: Software Development :: Libraries :: Python Modules",
]
```

## Estimated Effort
**3-4 hours** (synchronization, testing, validation)

## Risk Mitigation
- Test wheel building before committing version changes
- Verify Python import functionality with version changes
- Document rollback procedure if build issues occur
- Coordinate with PRP-22 for metadata consistency

## Success Score
**9/10** - Clear requirements with established validation tools and existing working build system.