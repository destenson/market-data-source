//! Quick demo of the Market Data Source Server
//!
//! This example shows how to start the server and interact with it.
//! 
//! Run this example:
//! ```bash
//! cargo run --example server_demo --features api-server
//! ```

#[cfg(not(feature = "api-server"))]
fn main() {
    eprintln!("This example requires the 'api-server' feature. Please run with '--features api-server'");
}

#[cfg(feature = "api-server")]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use market_data_source::server::{run_server, ServerConfig};
    
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter("market_data_server=info,tower_http=info")
        .init();
    
    println!("🚀 Starting Market Data Source Server Demo");
    println!("==========================================\n");
    
    // Configure the server
    let config = ServerConfig {
        port: 8080,
        host: "127.0.0.1".to_string(),
        enable_websocket: true,
        enable_swagger: true,
        ..Default::default()
    };
    
    println!("📍 Server Configuration:");
    println!("   Host: {}", config.host);
    println!("   Port: {}", config.port);
    println!("   WebSocket: {}", if config.enable_websocket { "✅ Enabled" } else { "❌ Disabled" });
    println!("   Swagger UI: {}", if config.enable_swagger { "✅ Enabled" } else { "❌ Disabled" });
    println!();
    
    println!("📚 Available Endpoints:");
    println!("   Home:           http://{}:{}/", config.host, config.port);
    println!("   Health:         http://{}:{}/health", config.host, config.port);
    println!("   API Discovery:  http://{}:{}/api", config.host, config.port);
    println!("   REST API:       http://{}:{}/api/v1/*", config.host, config.port);
    
    if config.enable_websocket {
        println!("   WebSocket:      ws://{}:{}/ws", config.host, config.port);
    }
    
    if config.enable_swagger {
        println!("   Swagger UI:     http://{}:{}/swagger-ui", config.host, config.port);
    }
    
    println!();
    println!("🔥 Server Features:");
    println!("   ✅ Runtime API discovery");
    println!("   ✅ Multiple symbol support");
    println!("   ✅ Real-time WebSocket streaming");
    println!("   ✅ Configurable data generators");
    println!("   ✅ Multiple export formats (JSON, CSV, PNG)");
    println!("   ✅ OpenAPI documentation");
    println!();
    
    println!("📝 Example API Calls:");
    println!("   Create symbol:  curl -X POST http://localhost:8080/api/v1/symbols \\");
    println!("                     -H 'Content-Type: application/json' \\");
    println!("                     -d '{{\"symbol\":\"BTCUSD\",\"config\":{{\"initial_price\":\"50000\"}}}}'");
    println!();
    println!("   Generate data:  curl -X POST http://localhost:8080/api/v1/generate/BTCUSD \\");
    println!("                     -H 'Content-Type: application/json' \\");
    println!("                     -d '{{\"count\":10,\"format\":\"ohlc\"}}'");
    println!();
    println!("   Stream data:    curl http://localhost:8080/api/v1/stream/BTCUSD");
    println!();
    
    println!("🔌 WebSocket Example:");
    println!("   Connect:        wscat -c ws://localhost:8080/ws");
    println!("   Subscribe:      {{\"type\":\"Subscribe\",\"payload\":{{\"symbol\":\"BTCUSD\",\"interval\":1000}}}}");
    println!("   Unsubscribe:    {{\"type\":\"Unsubscribe\",\"payload\":{{\"symbol\":\"BTCUSD\"}}}}");
    println!();
    
    println!("⚡ Starting server...");
    println!("Press Ctrl+C to stop\n");
    
    // Run the server
    run_server(config).await?;
    
    Ok(())
}