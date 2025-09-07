# PRP-53: Options Strategy Generator

## Context & Motivation

**Integration Goal**: Generate data for common options strategies (spreads, straddles, condors).

**User Requirement**: Support complex options positions with proper P&L and Greeks aggregation.

**Technical Challenge**: Calculate combined position metrics and maintain consistency.

## Requirements

### Strategy Types
1. **Vertical Spreads**: Bull/bear spreads
2. **Calendar Spreads**: Time spreads
3. **Volatility Plays**: Straddles, strangles
4. **Complex**: Condors, butterflies

### Position Management
1. **Multi-leg Support**: 2-4 leg strategies
2. **Greeks Aggregation**: Portfolio Greeks
3. **P&L Calculation**: Strategy payoffs
4. **Margin Requirements**: Risk-based margin

## Implementation Blueprint

### Phase 1: Strategy Definition
1. Create `src/options/strategies.rs`
2. Define strategy types enum
3. Implement leg management
4. Add position builder

### Phase 2: Analytics
1. Calculate combined Greeks
2. Implement P&L profiles
3. Add breakeven calculations
4. Create max profit/loss

### Phase 3: Generation
1. Generate strategy prices
2. Add execution simulation
3. Create strategy scanner
4. Implement optimization

## Success Criteria

### Validation Gates
```bash
# Test strategies
cargo test options_strategies
cargo test strategy_greeks

# Validate P&L
cargo test strategy_payoffs
```

### Implementation Metrics
- [ ] All common strategies supported
- [ ] Greeks aggregation correct
- [ ] P&L calculations accurate
- [ ] Margin calculations working

## Dependencies & References

**Prerequisites**:
- Complete PRP-51 (Options Pricing)
- Need PRP-52 (Vol Surface)

**Strategy Definitions**:
- Vertical: Same expiry, different strikes
- Calendar: Same strike, different expiries
- Iron Condor: Sell OTM strangle + buy further OTM

**Risk Metrics**:
- Max profit/loss
- Breakeven points
- Probability of profit
- Expected value

## Implementation Tasks

### Phase 1: Strategies (2-3 hours)
1. Define strategy types
2. Implement builders
3. Create validation
4. Test construction

### Phase 2: Analytics (2-3 hours)
1. Aggregate Greeks
2. Calculate P&L
3. Find breakevens
4. Test calculations

### Phase 3: Tools (2-3 hours)
1. Generate prices
2. Add scanner
3. Create optimizer
4. Document usage

## Risk Mitigation
- Validate strategy construction
- Check for arbitrage opportunities
- Handle early assignment risk
- Provide strategy templates

## Success Score
**7/10** - Well-defined strategies but requires careful Greek aggregation and P&L calculation.