# Smart Defaults for Symbol Configuration

The Market Data Source API server now features **intelligent default configuration** for the create symbol endpoint. This means you can create symbols with minimal or even no configuration, and the system will apply sensible defaults based on what you provide.

## How It Works

When you create a symbol, the API:
1. Accepts partial or missing configuration
2. Applies base defaults for any missing fields
3. **Intelligently adjusts** defaults based on provided values
4. Validates the final configuration

## Examples

### Minimal Configuration

#### 1. No Configuration (Complete Defaults)
```json
POST /symbols
{
  "symbol": "TEST"
}
```
Results in:
- Starting price: 100.0
- Min price: 1.0
- Max price: 10,000.0
- Volatility: 2%
- Trend: Sideways
- Time interval: 1 minute

#### 2. Only Starting Price
```json
POST /symbols
{
  "symbol": "BTCUSD",
  "config": {
    "starting_price": "50000"
  }
}
```
Smart defaults applied:
- Min price: 500 (1% of starting)
- Max price: 5,000,000 (100x starting)
- Volatility: 5% (inferred crypto from high price)
- Other defaults remain

#### 3. Forex Detection
```json
POST /symbols
{
  "symbol": "EURUSD",
  "config": {
    "starting_price": "1.2"
  }
}
```
Smart defaults applied:
- Min price: 0.012 (1% of starting)
- Max price: 120 (100x starting)
- Volatility: 0.5% (inferred forex from low price)

## Smart Adjustments

### Price Range Intelligence
- If `starting_price` > 1000 and `min_price` not set → `min_price = starting_price * 0.01`
- If `starting_price` set and `max_price` not set → `max_price = starting_price * 100`
- Always ensures: `min_price < starting_price < max_price`

### Volatility Inference
Based on starting price:
- Price > 10,000 → 5% volatility (crypto assets like BTC)
- Price < 10 → 0.5% volatility (forex pairs)
- Otherwise → 2% volatility (stocks)

### Trend Strength
- If `trend_direction = "up"` but no strength → `trend_strength = 0.0001`
- If `trend_direction = "down"` but no strength → `trend_strength = -0.0001`
- If `trend_direction = "sideways"` → `trend_strength = 0`

## Full Configuration Reference

All fields are optional. Here's what each defaults to:

| Field | Default | Smart Adjustment |
|-------|---------|------------------|
| `starting_price` | 100.0 | - |
| `min_price` | 1.0 | Adjusted based on starting_price |
| `max_price` | 1e15 | Adjusted based on starting_price |
| `trend_direction` | "sideways" | - |
| `trend_strength` | 0.0 | Set if trend_direction is up/down |
| `volatility` | 0.02 | Adjusted based on asset type |
| `time_interval` | "1m" | - |
| `num_points` | 100 | - |
| `seed` | null | - |
| `base_volume` | 100000 | - |
| `volume_volatility` | 0.3 | - |

## Trend Direction Values

The `trend_direction` field accepts:
- `"up"` or `"bullish"` → Upward trend
- `"down"` or `"bearish"` → Downward trend  
- `"flat"` or `"sideways"` → No trend

## Use Cases

### Quick Testing
```json
{
  "symbol": "TEST"
}
```
Creates a symbol with all defaults for quick testing.

### Crypto Asset
```json
{
  "symbol": "BTCUSD",
  "config": {
    "starting_price": "50000"
  }
}
```
Automatically gets crypto-appropriate volatility and price ranges.

### Forex Pair
```json
{
  "symbol": "GBPUSD", 
  "config": {
    "starting_price": "1.35",
    "trend_direction": "up"
  }
}
```
Gets forex-appropriate low volatility and tight price ranges.

### Stock
```json
{
  "symbol": "AAPL",
  "config": {
    "starting_price": "150",
    "volatility": "0.025"
  }
}
```
Price ranges adjusted for stock-like behavior.

## Validation

After applying smart defaults, the configuration is validated to ensure:
- All prices are positive
- Min price < starting price < max price
- Volatility is non-negative
- Trend strength is between -100% and +100%
- Volume parameters are positive

If validation fails, the API returns a 400 error with details about what failed.

## Benefits

1. **Faster Development**: Start with minimal config, refine later
2. **Fewer Errors**: Smart defaults prevent common misconfigurations
3. **Asset-Aware**: Different asset types get appropriate defaults
4. **Backward Compatible**: Full configuration still works exactly as before
5. **Discoverable**: Response shows the applied configuration

## Migration

Existing code with full configuration continues to work unchanged. The smart defaults only apply to missing fields, so you can gradually simplify your API calls.

### Before (Required All Fields)
```json
{
  "symbol": "BTCUSD",
  "config": {
    "starting_price": "50000",
    "min_price": "100",
    "max_price": "100000", 
    "trend_direction": "sideways",
    "trend_strength": "0",
    "volatility": "0.05",
    "time_interval": "1m",
    "num_points": 100,
    "base_volume": 100000,
    "volume_volatility": 0.3
  }
}
```

### After (Only What You Need)
```json
{
  "symbol": "BTCUSD",
  "config": {
    "starting_price": "50000",
    "volatility": "0.05"
  }
}
```

## Testing

Use the provided PowerShell test script to verify smart defaults:

```powershell
.\test-server.ps1
```

This runs multiple test cases including:
- Symbol with no config
- Symbol with only starting price
- Symbol with partial config
- Symbol with trend but no strength

All tests should pass, demonstrating the flexibility of the smart defaults system.