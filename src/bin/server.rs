use market_data_source::server::{run_server, ServerConfig};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(name = "market-data-server")]
#[clap(about = "Market Data Source Server - Synthetic market data generation with REST and WebSocket APIs")]
#[clap(version = env!("CARGO_PKG_VERSION"))]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
    
    #[clap(short, long, default_value = "8080")]
    port: u16,
    
    #[clap(short = 'H', long, default_value = "0.0.0.0")]
    host: String,
    
    #[clap(long, env = "LOG_LEVEL", default_value = "info")]
    log_level: String,
    
    #[clap(long)]
    no_websocket: bool,
    
    #[clap(long)]
    no_swagger: bool,
}

#[derive(Subcommand)]
enum Commands {
    Start {
        #[clap(short, long)]
        config: Option<String>,
    },
    Info,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("market_data_server={},tower_http=info", cli.log_level).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
    
    match cli.command {
        Some(Commands::Start { config }) => {
            let mut server_config = if let Some(config_path) = config {
                let config_str = std::fs::read_to_string(config_path)?;
                serde_json::from_str(&config_str)?
            } else {
                ServerConfig::from_env()
            };
            
            server_config.port = cli.port;
            server_config.host = cli.host;
            server_config.enable_websocket = !cli.no_websocket;
            server_config.enable_swagger = !cli.no_swagger;
            server_config.log_level = cli.log_level;
            
            println!("🚀 Market Data Source Server v{}", env!("CARGO_PKG_VERSION"));
            println!("📍 Starting server on http://{}:{}", server_config.host, server_config.port);
            println!("📊 API Discovery: http://{}:{}/api", server_config.host, server_config.port);
            
            if server_config.enable_swagger {
                println!("📚 Swagger UI: http://{}:{}/swagger-ui", server_config.host, server_config.port);
            }
            
            if server_config.enable_websocket {
                println!("🔌 WebSocket: ws://{}:{}/ws", server_config.host, server_config.port);
            }
            
            println!("\nPress Ctrl+C to stop the server\n");
            
            run_server(server_config).await?;
        }
        Some(Commands::Info) | None => {
            println!("Market Data Source Server v{}", env!("CARGO_PKG_VERSION"));
            println!("\nFeatures:");
            println!("  ✅ REST API with runtime discovery");
            println!("  ✅ WebSocket streaming");
            println!("  ✅ Multiple symbol support");
            println!("  ✅ Configurable data generators");
            println!("  ✅ Export to multiple formats");
            println!("  ✅ OpenAPI/Swagger documentation");
            println!("\nUsage:");
            println!("  market-data-server --port 8080");
            println!("  market-data-server start --config config.json");
            println!("\nEnvironment Variables:");
            println!("  SERVER_PORT - Server port (default: 8080)");
            println!("  SERVER_HOST - Server host (default: 0.0.0.0)");
            println!("  LOG_LEVEL - Log level (default: info)");
            println!("  ENABLE_WEBSOCKET - Enable WebSocket (default: true)");
            println!("  ENABLE_SWAGGER - Enable Swagger UI (default: true)");
        }
    }
    
    Ok(())
}