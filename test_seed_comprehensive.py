import market_data_source as mds

print("Testing seed reproducibility and uniqueness")
print("=" * 50)

# Test 1: Same seed produces identical data
print("\n1. Testing same seed (42) produces identical data:")
gen1 = mds.MarketDataGenerator(seed=42, initial_price=100.0, volatility=0.02)
gen2 = mds.MarketDataGenerator(seed=42, initial_price=100.0, volatility=0.02)

data1 = gen1.generate_series(10)
data2 = gen2.generate_series(10)

identical = True
for i in range(10):
    if data1[i]['close'] != data2[i]['close']:
        identical = False
        break

print(f"  First 3 closes from gen1: {[round(d['close'], 2) for d in data1[:3]]}")
print(f"  First 3 closes from gen2: {[round(d['close'], 2) for d in data2[:3]]}")
print(f"  Result: {'PASS - Data is identical' if identical else 'FAIL - Data differs'}")

# Test 2: Different seeds produce different data
print("\n2. Testing different seeds produce different data:")
seeds = [42, 99, 123, 456, 789]
generators = []
first_values = []

for seed in seeds:
    gen = mds.MarketDataGenerator(seed=seed, initial_price=100.0, volatility=0.02)
    data = gen.generate_series(1)
    first_close = data[0]['close']
    generators.append(gen)
    first_values.append(first_close)
    print(f"  Seed {seed:3d}: First close = {first_close:.4f}")

# Check all values are unique
unique_values = len(set(first_values))
print(f"\n  Unique values: {unique_values} out of {len(seeds)}")
print(f"  Result: {'PASS - All seeds produce different data' if unique_values == len(seeds) else 'FAIL - Some seeds produced identical data'}")

# Test 3: set_seed changes the sequence
print("\n3. Testing set_seed() changes the sequence:")
gen = mds.MarketDataGenerator(seed=42)
data1 = gen.generate_series(5)
first_sequence = [round(d['close'], 2) for d in data1]
print(f"  Initial sequence (seed=42): {first_sequence}")

gen.set_seed(999)
data2 = gen.generate_series(5)
second_sequence = [round(d['close'], 2) for d in data2]
print(f"  After set_seed(999):        {second_sequence}")

sequences_differ = first_sequence != second_sequence
print(f"  Result: {'PASS - Sequences differ' if sequences_differ else 'FAIL - Sequences are identical'}")

# Test 4: Resetting to same seed reproduces data
print("\n4. Testing resetting to same seed reproduces data:")
gen.set_seed(42)
data3 = gen.generate_series(5)
reset_sequence = [round(d['close'], 2) for d in data3]
print(f"  After set_seed(42) again:   {reset_sequence}")

sequences_match = first_sequence == reset_sequence
print(f"  Result: {'PASS - Original sequence reproduced' if sequences_match else 'FAIL - Sequences differ'}")

print("\n" + "=" * 50)
print("OVERALL: All tests", "PASSED" if identical and unique_values == len(seeds) and sequences_differ and sequences_match else "FAILED")