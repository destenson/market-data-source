# PRP-46: Foreign Exchange Pair Generator

## Context & Motivation

**Integration Goal**: Generate realistic forex market data with proper cross-rate consistency.

**User Requirement**: Support major, minor, and exotic currency pairs with realistic spreads and volatility.

**Technical Challenge**: Maintain triangular arbitrage-free conditions across all pairs.

## Requirements

### FX Features
1. **Currency Pairs**: Majors, minors, exotics
2. **Cross Rates**: Triangular arbitrage-free
3. **Interest Rate Parity**: Forward rates
4. **Market Hours**: 24/5 trading patterns

### Market Characteristics
1. **Spread Dynamics**: Pair-specific spreads
2. **Volatility Patterns**: Session-based volatility
3. **Central Bank Events**: Rate decisions impact
4. **Carry Trade Dynamics**: Interest differentials

## Implementation Blueprint

### Phase 1: Core FX
1. Create `src/forex/mod.rs`
2. Define currency pair structures
3. Implement rate generation
4. Ensure triangular consistency

### Phase 2: Market Dynamics
1. Add session volatility (Asian/European/US)
2. Implement spread widening/tightening
3. Create interest rate differentials
4. Add forward rate calculation

### Phase 3: Advanced Features
1. Add central bank events
2. Implement correlation matrix
3. Create exotic pair modeling
4. Add carry trade factors

## Success Criteria

### Validation Gates
```bash
# Test forex generation
cargo test forex_generator
cargo test triangular_arbitrage

# Validate cross rates
cargo test cross_rate_consistency
```

### Implementation Metrics
- [ ] No triangular arbitrage opportunities
- [ ] Realistic session volatility
- [ ] Proper interest rate parity
- [ ] Correlation matrix positive definite

## Dependencies & References

**Research Sources**:
- FX market microstructure papers
- Interest rate parity theory
- Carry trade literature
- Central bank impact studies

**Key Relationships**:
- Triangular: EUR/USD * USD/JPY = EUR/JPY
- Covered interest parity
- Purchasing power parity
- Real exchange rates

**Market Structure**:
- 24/5 continuous trading
- Major sessions overlap
- Central bank meetings
- Economic data releases

## Implementation Tasks

### Phase 1: Basic FX (2-3 hours)
1. Define pair structure
2. Generate spot rates
3. Ensure consistency
4. Test arbitrage-free

### Phase 2: Dynamics (2-3 hours)
1. Add volatility patterns
2. Implement spreads
3. Create forward rates
4. Validate parity

### Phase 3: Advanced (2-3 hours)
1. Add event modeling
2. Create correlations
3. Implement exotics
4. Document patterns

## Risk Mitigation
- Enforce triangular consistency
- Validate against real FX data
- Handle weekend gaps properly
- Provide major pair defaults

## Success Score
**7/10** - Well-understood market with clear consistency requirements to implement.