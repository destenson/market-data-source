# PRP-22: Crates.io Publication Metadata Setup

## Context & Motivation

Crates.io requires specific mandatory metadata fields for publication: `description` and `license` are absolute requirements. Current Cargo.toml is missing critical metadata that blocks publication.

**Based on 2025 crates.io requirements**: Package names are allocated first-come-first-served, and `description` + `license` are mandatory for publication acceptance.

**Current Status**: Cargo.toml has `repository` and `authors` but missing required fields.

## Requirements

### Mandatory for Publication
- `description`: Clear, single-line description for crates.io search and display
- `license`: SPDX identifier (project has MIT LICENSE file)

### Recommended for Discoverability  
- `keywords`: Up to 5 keywords for search optimization
- `categories`: Official crates.io categories for classification
- `readme`: Link to README.md for package page
- `homepage`: Project homepage URL
- `documentation`: Generated docs URL

## Implementation Blueprint

### Cargo.toml Additions
Based on existing PUBLISHING_STRATEGY.md recommendations and 2025 requirements:

1. Add required `description` field using existing README.md content
2. Add `license = "MIT"` referencing existing LICENSE file
3. Add `readme = "README.md"` for package page display
4. Include optimized `keywords` for financial/trading community discovery
5. Select appropriate `categories` from crates.io official list
6. Add `homepage` and `documentation` URLs

### SEO Optimization Strategy
Target keywords from PUBLISHING_STRATEGY.md:
- "market-data" (primary)
- "trading"
- "finance" 
- "synthetic-data"
- "ohlc"

## Success Criteria

### Validation Gates
```bash
# Verify metadata validity
cargo publish --dry-run

# Check package contents
cargo package --list

# Validate no warnings about missing metadata
cargo check --all-features
```

### Publication Requirements Checklist
- [ ] Package name "market-data-source" available on crates.io
- [ ] Description under 300 characters, non-technical language
- [ ] License matches existing LICENSE file
- [ ] Keywords target financial data generation community
- [ ] Categories align with official crates.io taxonomy
- [ ] README displays properly in package preview

## Dependencies & References

**Documentation Requirements**:
- Crates.io publishing guide: https://doc.rust-lang.org/cargo/reference/publishing.html
- SPDX license identifiers: https://spdx.org/licenses/
- Crates.io categories: https://crates.io/categories

**Existing Resources**:
- Reference existing docs/PUBLISHING_STRATEGY.md template
- Use existing LICENSE file (MIT)
- Leverage existing README.md content

**Integration Points**:
- Must align with pyproject.toml metadata (PRP-23)
- Coordinates with GitHub repository settings

## Implementation Tasks

1. Research crates.io categories and select 2-3 most appropriate
2. Craft description under 300 characters highlighting key value propositions
3. Compile 5 strategic keywords for SEO optimization
4. Add all metadata fields to Cargo.toml [package] section
5. Test with `cargo publish --dry-run` to validate metadata
6. Verify package preview display with `cargo package`
7. Cross-reference with existing docs/PUBLISHING_STRATEGY.md
8. Coordinate with README.md content to ensure consistency

## Metadata Values Planning

### Target Description
"High-performance synthetic market data generator with financial precision. Generate unlimited OHLC candles, tick data, and realistic trading scenarios for backtesting and research."

### Keywords Strategy
- "market-data"
- "synthetic-data"  
- "trading"
- "finance"
- "backtesting"

### Categories Research
- "finance" (primary)
- "simulation" (secondary)
- "api-bindings" (for PyO3 integration)

## Estimated Effort
**4-6 hours** (research, implementation, validation)

## Risk Mitigation
- Verify package name availability early via crates.io search
- Test description clarity with non-technical users
- Validate all URLs are accessible and permanent
- Ensure license identifier matches exactly with LICENSE file content

## Success Score
**9/10** - Straightforward metadata addition with clear requirements and validation tools.