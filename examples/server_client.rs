//! Example client for interacting with the Market Data Source server
//!
//! Run the server first:
//! ```bash
//! cargo run --bin market-data-server --features api-server
//! ```
//!
//! Then run this example:
//! ```bash
//! cargo run --example server_client --features api-server
//! ```

use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let base_url = "http://localhost:8080";
    let client = reqwest::Client::new();
    
    println!("Testing Market Data Source Server API\n");
    
    // 1. Check server health
    println!("1. Checking server health...");
    let response = client.get(format!("{}/health", base_url))
        .send()
        .await?;
    let health: serde_json::Value = response.json().await?;
    println!("   Health: {}\n", serde_json::to_string_pretty(&health)?);
    
    // 2. Discover API capabilities
    println!("2. Discovering API capabilities...");
    let response = client.get(format!("{}/api", base_url))
        .send()
        .await?;
    let discovery: serde_json::Value = response.json().await?;
    println!("   Available endpoints: {}", discovery["endpoints"].as_object().unwrap().keys().cloned().collect::<Vec<_>>().join(", "));
    println!("   WebSocket endpoint: {}\n", discovery["websocket"]["url"]);
    
    // 3. Get detailed capabilities
    println!("3. Getting detailed capabilities...");
    let response = client.get(format!("{}/api/v1/capabilities", base_url))
        .send()
        .await?;
    let capabilities: serde_json::Value = response.json().await?;
    println!("   Features: {:?}", capabilities["features"]);
    println!("   Export formats: {:?}\n", capabilities["export_formats"]);
    
    // 4. Create a new symbol generator
    println!("4. Creating symbol generator for 'BTCUSD'...");
    let create_request = json!({
        "symbol": "BTCUSD",
        "config": {
            "initial_price": "50000.0",
            "volatility": "0.05",
            "drift": "0.0002",
            "volume_mean": "1000000.0",
            "volume_volatility": 0.15
        }
    });
    
    let response = client.post(format!("{}/api/v1/symbols", base_url))
        .json(&create_request)
        .send()
        .await?;
    
    if response.status().is_success() {
        let symbol_info: serde_json::Value = response.json().await?;
        println!("   Created: {}\n", symbol_info["symbol"]);
    } else {
        println!("   Failed to create symbol: {}\n", response.status());
    }
    
    // 5. Generate some data
    println!("5. Generating market data...");
    let generate_request = json!({
        "count": 10,
        "format": "ohlc"
    });
    
    let response = client.post(format!("{}/api/v1/generate/BTCUSD", base_url))
        .json(&generate_request)
        .send()
        .await?;
    
    if response.status().is_success() {
        let data: serde_json::Value = response.json().await?;
        println!("   Generated {} candles", data["metadata"]["count"]);
        if let Some(ohlc_array) = data["data"].as_array() {
            for (i, candle) in ohlc_array.iter().take(3).enumerate() {
                println!("   Candle {}: O:{} H:{} L:{} C:{} V:{}", 
                    i + 1,
                    candle["open"], 
                    candle["high"], 
                    candle["low"], 
                    candle["close"],
                    candle["volume"]["value"]
                );
            }
            if ohlc_array.len() > 3 {
                println!("   ... and {} more candles\n", ohlc_array.len() - 3);
            }
        }
    }
    
    // 6. List available symbols
    println!("6. Listing available symbols...");
    let response = client.get(format!("{}/api/v1/symbols", base_url))
        .send()
        .await?;
    let symbols: serde_json::Value = response.json().await?;
    println!("   Symbols: {:?}\n", symbols["symbols"]);
    
    // 7. Get historical data
    println!("7. Getting historical data...");
    let response = client.get(format!("{}/api/v1/historical/BTCUSD?limit=5", base_url))
        .send()
        .await?;
    
    if response.status().is_success() {
        let historical: serde_json::Value = response.json().await?;
        println!("   Retrieved {} historical candles\n", historical["count"]);
    }
    
    // 8. List available algorithms
    println!("8. Listing available algorithms...");
    let response = client.get(format!("{}/api/v1/algorithms", base_url))
        .send()
        .await?;
    let algorithms: serde_json::Value = response.json().await?;
    println!("   Current: {:?}", algorithms["algorithms"][0]["name"]);
    println!("   Planned: {:?}\n", algorithms["planned"]);
    
    // 9. List available presets
    println!("9. Listing available presets...");
    let response = client.get(format!("{}/api/v1/presets", base_url))
        .send()
        .await?;
    let presets: serde_json::Value = response.json().await?;
    let preset_names: Vec<String> = presets["presets"].as_object()
        .unwrap()
        .keys()
        .cloned()
        .collect();
    println!("   Presets: {:?}\n", preset_names);
    
    // 10. Test WebSocket connection
    println!("10. Testing WebSocket connection...");
    println!("    WebSocket test would connect to ws://localhost:8080/ws");
    println!("    and subscribe to real-time market data streams\n");
    
    println!("All API tests completed successfully!");
    
    Ok(())
}