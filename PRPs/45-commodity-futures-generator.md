# PRP-45: Commodity Futures Data Generator

## Context & Motivation

**Integration Goal**: Generate realistic commodity futures data with term structure and seasonality.

**User Requirement**: Support energy, metals, and agricultural futures with proper forward curves.

**Technical Challenge**: Model contango/backwardation, seasonality, and storage costs.

## Requirements

### Futures Features
1. **Forward Curves**: Contango and backwardation
2. **Seasonality**: Periodic patterns (e.g., natural gas)
3. **Roll Yield**: Returns from curve dynamics
4. **Storage Costs**: Cost of carry modeling

### Commodity Types
1. **Energy**: Oil, gas with inventory effects
2. **Metals**: Gold, silver with storage costs
3. **Agriculture**: Grains with harvest cycles
4. **Softs**: Coffee, sugar with weather impacts

## Implementation Blueprint

### Phase 1: Futures Curves
1. Create `src/commodities/mod.rs`
2. Define futures curve structures
3. Implement contango/backwardation
4. Add curve evolution model

### Phase 2: Seasonality
1. Add seasonal patterns
2. Implement commodity calendars
3. Create weather/event impacts
4. Add harvest cycle modeling

### Phase 3: Specific Commodities
1. Implement energy futures
2. Add precious metals
3. Create agricultural products
4. Add soft commodities

## Success Criteria

### Validation Gates
```bash
# Test commodity generation
cargo test commodity_futures
cargo test seasonality_patterns

# Validate curves
cargo test futures_curves
```

### Implementation Metrics
- [ ] Realistic forward curves
- [ ] Proper seasonal patterns
- [ ] Storage cost integration
- [ ] Roll yield calculation correct

## Dependencies & References

**Research Sources**:
- Commodity futures literature
- Seasonality studies
- Storage theory papers
- Forward curve models

**Key Concepts**:
- Convenience yield
- Storage costs and carry
- Seasonal supply/demand
- Weather risk factors

**Implementation Patterns**:
- Extend bond curve concepts
- Add commodity-specific features
- Use calendar for seasonality

## Implementation Tasks

### Phase 1: Core Futures (3-4 hours)
1. Design curve structure
2. Implement basic curves
3. Add evolution model
4. Test dynamics

### Phase 2: Seasonality (2-3 hours)
1. Add seasonal factors
2. Implement calendars
3. Create patterns
4. Validate seasonality

### Phase 3: Commodities (2-3 hours)
1. Add energy futures
2. Implement metals
3. Create agriculture
4. Document specifics

## Risk Mitigation
- Research commodity-specific patterns
- Validate against historical curves
- Handle extreme weather events
- Provide sensible defaults per commodity

## Success Score
**6/10** - Domain-specific knowledge required for realistic commodity modeling.