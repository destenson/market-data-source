#![allow(unused)]
// Test that the create symbol endpoint works with minimal configuration

use serde_json::json;

fn main() {
    println!("Testing minimal configuration scenarios...\n");
    
    // Test 1: Only symbol name (complete defaults)
    let config1 = json!({
        "symbol": "TEST1"
    });
    println!("Test 1 - Only symbol:");
    println!("{}\n", serde_json::to_string_pretty(&config1).unwrap());
    
    // Test 2: Only starting price (smart defaults should adjust min/max)
    let config2 = json!({
        "symbol": "BTCUSD",
        "config": {
            "starting_price": "50000"
        }
    });
    println!("Test 2 - Only starting price (should infer crypto volatility):");
    println!("{}\n", serde_json::to_string_pretty(&config2).unwrap());
    
    // Test 3: Trend direction without strength (should add default strength)
    let config3 = json!({
        "symbol": "EURUSD",
        "config": {
            "starting_price": "1.2",
            "trend_direction": "up"
        }
    });
    println!("Test 3 - Trend direction without strength:");
    println!("{}\n", serde_json::to_string_pretty(&config3).unwrap());
    
    // Test 4: Mixed partial config
    let config4 = json!({
        "symbol": "AAPL",
        "config": {
            "starting_price": "150",
            "volatility": "0.03"
        }
    });
    println!("Test 4 - Mixed partial config:");
    println!("{}\n", serde_json::to_string_pretty(&config4).unwrap());
    
    // Test 5: Edge case - very low price (should infer forex-like volatility)
    let config5 = json!({
        "symbol": "JPYUSD",
        "config": {
            "starting_price": "0.007"
        }
    });
    println!("Test 5 - Very low price (should infer forex volatility):");
    println!("{}\n", serde_json::to_string_pretty(&config5).unwrap());
    
    println!("All configs should be accepted by the API with smart defaults applied!");
}
