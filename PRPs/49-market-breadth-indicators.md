# PRP-49: Market Breadth Indicators

## Context & Motivation

**Integration Goal**: Generate market breadth data (advance/decline, new highs/lows) for market health analysis.

**User Requirement**: Simulate market internals that reflect overall market conditions.

**Technical Challenge**: Coordinate individual stock movements with market-wide statistics.

## Requirements

### Breadth Indicators
1. **Advance/Decline Line**: Cumulative A/D
2. **New Highs/Lows**: 52-week highs and lows
3. **Up/Down Volume**: Volume breadth
4. **McClellan Oscillator**: Breadth momentum

### Market Internals
1. **Participation Rate**: Percentage of advancing stocks
2. **Breadth Thrust**: Strong momentum indicators
3. **Divergences**: Price vs breadth divergences
4. **Sector Breadth**: Sector-level statistics

## Implementation Blueprint

### Phase 1: Core Breadth
1. Create `src/universe/breadth.rs`
2. Track individual stock movements
3. Calculate A/D statistics
4. Implement cumulative line

### Phase 2: Advanced Indicators
1. Add new highs/lows tracking
2. Implement volume breadth
3. Create McClellan calculations
4. Add breadth thrust detection

### Phase 3: Analysis
1. Add divergence detection
2. Create breadth signals
3. Implement sector breadth
4. Add visualization data

## Success Criteria

### Validation Gates
```bash
# Test breadth indicators
cargo test market_breadth
cargo test advance_decline

# Validate calculations
cargo test mcclellan_oscillator
```

### Implementation Metrics
- [ ] A/D line calculation correct
- [ ] New highs/lows tracking working
- [ ] Breadth statistics accurate
- [ ] Divergences detected properly

## Dependencies & References

**Prerequisites**:
- Universe of stocks needed
- Historical price tracking

**Technical Indicators**:
- A/D Line = Σ(Advances - Declines)
- McClellan = EMA(19) - EMA(39) of A/D
- Breadth Thrust > 0.615 bullish

**Market Relationships**:
- Healthy rallies have broad participation
- Divergences warn of reversals
- Breadth leads price

## Implementation Tasks

### Phase 1: Basic Breadth (2-3 hours)
1. Implement A/D tracking
2. Calculate statistics
3. Create cumulative line
4. Test calculations

### Phase 2: Indicators (2-3 hours)
1. Add highs/lows
2. Implement McClellan
3. Add volume breadth
4. Validate formulas

### Phase 3: Analysis (2-3 hours)
1. Detect divergences
2. Generate signals
3. Add sector breadth
4. Document patterns

## Risk Mitigation
- Validate formulas against references
- Handle edge cases (no advances)
- Smooth indicators appropriately
- Provide historical lookback

## Success Score
**8/10** - Well-defined indicators with clear formulas and established patterns.