import market_data_source as mds

# Test reproducibility with same seed
g1 = mds.MarketDataGenerator(seed=42)
d1 = g1.generate_series(3)

g2 = mds.MarketDataGenerator(seed=42)
d2 = g2.generate_series(3)

print(f"Generator 1 first close: {d1[0]['close']}")
print(f"Generator 2 first close: {d2[0]['close']}")
print(f"Same seed produces same data: {d1[0]['close'] == d2[0]['close']}")

# Test different seed
g1.set_seed(99)
d3 = g1.generate_series(3)
print(f"\nAfter set_seed(99) first close: {d3[0]['close']}")
print(f"Different seed produces different data: {d1[0]['close'] != d3[0]['close']}")