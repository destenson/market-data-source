# PRP: Random Walk Price Generation Algorithm

## Objective
Implement the random walk with drift algorithm for generating realistic price movements based on configured parameters.

## Context
Random walk with drift is a fundamental model for asset prices. It combines a trend component (drift) with random fluctuations (volatility). This will be the core algorithm inside the generator for creating price movements.

## Success Criteria
- Generates prices following random walk with drift
- Respects configured volatility (standard deviation)
- Follows trend direction and strength
- Stays within price bounds if configured
- Produces realistic OHLC relationships

## Implementation Tasks
1. Create src/algorithms/mod.rs module structure
2. Create src/algorithms/random_walk.rs
3. Implement calculate_next_price() function
4. Add drift calculation based on trend
5. Add volatility using normal distribution
6. Implement OHLC generation from price path
7. Add price bound enforcement
8. Write statistical tests

## Algorithm Components
- Price change = (drift * dt) + (volatility * sqrt(dt) * random_normal)
- Drift = trend_strength * current_price
- Random normal from standard normal distribution
- OHLC from multiple price points within period

## Mathematical Model
- Next price = Current price * (1 + drift + volatility * N(0,1))
- Where N(0,1) is standard normal distribution
- Time scaling: volatility scales with sqrt(time)
- Ensure prices stay positive (use log-normal if needed)

## Validation Gates
```bash
# Build and test
cargo test algorithms

# Statistical tests
cargo test test_volatility_distribution
cargo test test_trend_drift

# Verify bounds
cargo test test_price_bounds
```

## References
- Random walk theory: https://en.wikipedia.org/wiki/Random_walk_hypothesis
- Geometric Brownian Motion: https://en.wikipedia.org/wiki/Geometric_Brownian_motion
- Normal distribution in rand: https://docs.rs/rand_distr/latest/rand_distr/struct.Normal.html

## Dependencies
- PRP 04 (Generator struct to use this)
- rand_distr crate for distributions
- Add `rand_distr = "0.4"` to Cargo.toml

## Notes
- Use log-normal to ensure positive prices
- Consider using Geometric Brownian Motion for more realism
- Time intervals affect volatility scaling
- Test with known parameters to verify statistical properties

## Confidence Score: 7/10
Mathematical algorithm requiring careful implementation and testing.