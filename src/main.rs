use clap::Parser;
use std::sync::Arc;
use tracing::{info, warn};

pub mod mcp;
pub mod server;
pub mod tools;
pub mod types;

use mcp::McpServer;
use server::StdioServer;

#[derive(Parser)]
#[command(name = "rust-mcp-server")]
#[command(about = "A Model Context Protocol (MCP) server implementation in Rust")]
struct Cli {
    /// Enable debug logging
    #[arg(short, long)]
    debug: bool,
    
    /// Disable all logging (for use with MCP clients)
    #[arg(short, long)]
    quiet: bool,
    
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
    
    // Initialize tracing only if not in quiet mode
    if !cli.quiet {
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
    }
    
    // Create the MCP server
    let mcp_server = Arc::new(McpServer::new(cli.name, cli.version));
    
    // Create and run the stdio server
    let stdio_server = StdioServer::new(mcp_server, cli.quiet);
    
    if let Err(e) = stdio_server.run().await {
        if !cli.quiet {
            warn!("Server error: {}", e);
        }
    }
    
    if !cli.quiet {
        info!("MCP server shutting down");
    }
    Ok(())
}
