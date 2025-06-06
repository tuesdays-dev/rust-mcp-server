use crate::mcp::McpServer;
use crate::tools::ToolRegistry;
use crate::types::{JsonRpcRequest, JsonRpcResponse, JsonRpcError};
use anyhow::Result;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::sync::Mutex;
use tracing::{debug, error, info, warn};

pub struct StdioServer {
    mcp_server: Arc<Mutex<McpServer>>,
    quiet: bool,
}

impl StdioServer {
    pub fn new(mcp_server: Arc<McpServer>, quiet: bool) -> Self {
        Self {
            mcp_server: Arc::new(Mutex::new((*mcp_server).clone())),
            quiet,
        }
    }
    
    pub async fn run(&self) -> Result<()> {
        if !self.quiet {
            info!("Starting stdio server");
        }
        
        let stdin = tokio::io::stdin();
        let mut stdout = tokio::io::stdout();
        let mut reader = BufReader::new(stdin);
        let mut line = String::new();
        
        loop {
            line.clear();
            
            match reader.read_line(&mut line).await {
                Ok(0) => {
                    // EOF reached
                    if !self.quiet {
                        info!("Client disconnected");
                    }
                    break;
                }
                Ok(_) => {
                    let trimmed = line.trim();
                    if trimmed.is_empty() {
                        continue;
                    }
                    
                    debug!("Received: {}", trimmed);
                    
                    let response = self.process_message(trimmed).await;
                    
                    // Only send response if it's not None (notifications return None)
                    if let Some(actual_response) = response {
                        let response_json = serde_json::to_string(&actual_response)?;
                        
                        debug!("Sending: {}", response_json);
                        
                        stdout.write_all(response_json.as_bytes()).await?;
                        stdout.write_all(b"\n").await?;
                        stdout.flush().await?;
                    }
                }
                Err(e) => {
                    error!("Error reading from stdin: {}", e);
                    break;
                }
            }
        }
        
        if !self.quiet {
            info!("Stdio server stopped");
        }
        Ok(())
    }
    
    async fn process_message(&self, message: &str) -> Option<JsonRpcResponse> {
        // Parse the JSON-RPC request
        let request: JsonRpcRequest = match serde_json::from_str(message) {
            Ok(req) => req,
            Err(e) => {
                warn!("Failed to parse JSON-RPC request: {}", e);
                return Some(JsonRpcResponse {
                    jsonrpc: "2.0".to_string(),
                    id: None,
                    result: None,
                    error: Some(JsonRpcError::parse_error()),
                });
            }
        };
        
        // Validate JSON-RPC version
        if request.jsonrpc != "2.0" {
            return Some(JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                id: request.id,
                result: None,
                error: Some(JsonRpcError::invalid_request()),
            });
        }
        
        // Handle the request
        let mut server = self.mcp_server.lock().await;
        match server.handle_request(request).await {
            Ok(Some(response)) => Some(response),
            Ok(None) => {
                // No response needed (notification)
                None
            },
            Err(e) => {
                error!("Error handling request: {}", e);
                Some(JsonRpcResponse {
                    jsonrpc: "2.0".to_string(),
                    id: None,
                    result: None,
                    error: Some(JsonRpcError::internal_error()),
                })
            }
        }
    }
}

// We need to implement Clone for McpServer to use it with Arc<Mutex<>>
impl Clone for McpServer {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            version: self.version.clone(),
            protocol_version: self.protocol_version.clone(),
            initialized: self.initialized,
            tool_registry: ToolRegistry::new(), // Create new registry for cloned instance
        }
    }
}
