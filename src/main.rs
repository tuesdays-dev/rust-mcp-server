use clap::Parser;
use std::sync::Arc;
use tracing::{info, warn};

mod mcp;
mod server;
mod tools;
mod types;

use mcp::McpServer;
use server::StdioServer;

#[derive(Parser)]
#[command(name = "rust-mcp-server")]
#[command(about = "A Model Context Protocol (MCP) server implementation in Rust")]
struct Cli {
    /// Enable debug logging
    #[arg(short, long)]
    debug: bool,
    
    /// Server name
    #[arg(short, long, default_value = "rust-mcp-server")]
    name: String,
    
    /// Server version
    #[arg(short, long, default_value = "0.1.0")]
    version: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    
    // Initialize tracing
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(if cli.debug {
            tracing::Level::DEBUG
        } else {
            tracing::Level::INFO
        })
        .finish();
    
    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");
    
    info!("Starting MCP server: {} v{}", cli.name, cli.version);
    
    // Create the MCP server
    let mcp_server = Arc::new(McpServer::new(cli.name, cli.version));
    
    // Create and run the stdio server
    let stdio_server = StdioServer::new(mcp_server);
    
    if let Err(e) = stdio_server.run().await {
        warn!("Server error: {}", e);
    }
    
    info!("MCP server shutting down");
    Ok(())
}
