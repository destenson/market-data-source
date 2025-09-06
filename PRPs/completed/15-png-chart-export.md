# PRP: Implement PNG Chart Export

## Objective
Add PNG chart generation to visualize OHLC data as candlestick charts and line charts, enabling quick visual inspection of generated market data without external tools.

## Context
Visualization is crucial for validating generated data. Using plotters crate, we can create publication-quality charts in pure Rust without external dependencies. Charts will include candlestick patterns, volume bars, and price overlays.

## Success Criteria
- Generate candlestick charts from OHLC data
- Generate line charts from tick data
- Include volume bars below price chart
- Add moving averages overlay
- Configurable chart dimensions and style
- Save charts as PNG files
- Tests verify image generation

## Implementation Tasks
1. Add plotters dependency to Cargo.toml
2. Create src/export/chart.rs module
3. Implement ChartBuilder struct
4. Create candlestick drawing function
5. Add volume bar subplot
6. Implement moving average overlay
7. Create line chart for tick data
8. Add chart styling options (colors, grid)
9. Write tests verifying PNG output

## Dependencies
- plotters = "0.3" with bitmap backend
- image = "0.24" for additional image processing (optional)

## References
- Plotters documentation: https://docs.rs/plotters/latest/plotters/
- Candlestick example: https://github.com/plotters-rs/plotters/blob/master/plotters/examples/stock.rs
- BitMapBackend: https://docs.rs/plotters/latest/plotters/prelude/struct.BitMapBackend.html
- Chart styling: https://docs.rs/plotters/latest/plotters/style/index.html

## Chart Components
- Main panel: Candlesticks with price axis
- Volume panel: Bar chart below main
- Overlays: SMA/EMA lines
- Legend: Symbol, timeframe, stats
- Grid: Major and minor gridlines

## Validation Gates
```bash
# Build with plotting support
cargo build --lib

# Run chart generation tests
cargo test chart
cargo test png_export

# Tests should verify PNG files are created
# Check file size > 0 and valid PNG header
```

## Notes
- Use BitMapBackend for PNG output
- Default size: 1920x1080 (configurable)
- Green/red candles for bullish/bearish
- Volume bars with transparency
- Include basic technical indicators (SMA)
- Consider memory usage for large datasets
- Make chart generation optional feature flag

## Confidence Score: 8/10
Plotters is well-documented, but chart layout and styling requires careful implementation.