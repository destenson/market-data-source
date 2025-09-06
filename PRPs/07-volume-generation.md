# PRP: Volume Generation with Price Correlation

## Objective
Implement realistic volume generation that correlates with price movements and follows typical market patterns.

## Context
Volume is crucial for realistic market data. It typically correlates with price volatility - higher volume during large price moves. Volume also follows intraday patterns (U-shape for stocks) and responds to events.

## Success Criteria
- Generate volume that correlates with price changes
- Follow configured average volume
- Implement intraday volume patterns
- Support volume spikes for events
- Maintain realistic volume distributions

## Implementation Tasks
1. Create src/algorithms/volume.rs
2. Define VolumeGenerator struct
3. Implement base volume calculation
4. Add price-volume correlation
5. Create intraday volume profile
6. Add volume spike generation
7. Implement volume smoothing
8. Write correlation tests

## Volume Patterns
- Base volume from configured average
- Correlation with absolute price change
- U-shaped intraday pattern (high at open/close)
- Random spikes for "news events"
- Log-normal distribution for realism

## Correlation Model
- Volume increases with |price_change|
- Correlation coefficient (configurable 0-1)
- Volume multiplier for large moves
- Minimum volume floor

## Validation Gates
```bash
# Build and test
cargo test volume

# Test correlation
cargo test test_price_volume_correlation

# Test distributions
cargo test test_volume_distribution

# Verify patterns
cargo test test_intraday_pattern
```

## References
- Volume patterns: https://www.investopedia.com/terms/v/volume.asp
- Intraday patterns: https://www.sciencedirect.com/science/article/pii/S0304405X00000571
- Log-normal distribution: https://docs.rs/rand_distr/latest/rand_distr/struct.LogNormal.html

## Dependencies
- PRP 05 (uses price changes from random walk)
- rand_distr for distributions

## Notes
- Volume should never be negative
- Consider different patterns for different markets
- Crypto typically has different patterns than stocks
- Volume in shares/contracts, not dollar volume

## Confidence Score: 7/10
Requires understanding of market microstructure and correlation modeling.