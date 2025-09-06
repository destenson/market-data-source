//! Algorithms for generating market data.

pub mod random_walk;

pub use random_walk::{RandomWalkGenerator, generate_ohlc_from_prices};