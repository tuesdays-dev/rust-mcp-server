use crate::tools::ToolRegistry;
use crate::types::*;
use anyhow::Result;
use tracing::{debug, info};

pub struct McpServer {
    pub name: String,
    pub version: String,
    pub protocol_version: String,
    pub initialized: bool,
    pub tool_registry: ToolRegistry,
}

impl McpServer {
    pub fn new(name: String, version: String) -> Self {
        Self {
            name,
            version,
            protocol_version: "2024-11-05".to_string(),
            initialized: false,
            tool_registry: ToolRegistry::new(),
        }
    }
    
    pub async fn handle_request(&mut self, request: JsonRpcRequest) -> Result<JsonRpcResponse> {
        debug!("Handling request: {} (id: {:?})", request.method, request.id);
        
        let result = match request.method.as_str() {
            "initialize" => self.handle_initialize(request.params).await,
            "initialized" => self.handle_initialized().await,
            "tools/list" => self.handle_list_tools().await,
            "tools/call" => self.handle_call_tool(request.params).await,
            "resources/list" => self.handle_list_resources().await,
            "prompts/list" => self.handle_list_prompts().await,
            "ping" => self.handle_ping().await,
            _ => {
                return Ok(JsonRpcResponse {
                    jsonrpc: "2.0".to_string(),
                    id: request.id,
                    result: None,
                    error: Some(JsonRpcError::method_not_found()),
                });
            }
        };
        
        match result {
            Ok(value) => Ok(JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                id: request.id,
                result: Some(value),
                error: None,
            }),
            Err(e) => {
                debug!("Request error: {}", e);
                Ok(JsonRpcResponse {
                    jsonrpc: "2.0".to_string(),
                    id: request.id,
                    result: None,
                    error: Some(JsonRpcError::internal_error()),
                })
            }
        }
    }
    
    async fn handle_initialize(&mut self, params: Option<serde_json::Value>) -> Result<serde_json::Value> {
        let request: InitializeRequest = if let Some(params) = params {
            serde_json::from_value(params)?
        } else {
            return Err(anyhow::anyhow!("Initialize request requires parameters"));
        };
        
        info!("Initializing MCP server for client: {} v{}", 
              request.client_info.name, request.client_info.version);
        
        self.initialized = true;
        
        let response = InitializeResponse {
            protocol_version: self.protocol_version.clone(),
            capabilities: ServerCapabilities {
                tools: Some(ToolsCapability {
                    list_changed: Some(false),
                }),
                resources: None,
                prompts: None,
                logging: None,
            },
            server_info: ServerInfo {
                name: self.name.clone(),
                version: self.version.clone(),
            },
        };
        
        Ok(serde_json::to_value(response)?)
    }
    
    async fn handle_initialized(&mut self) -> Result<serde_json::Value> {
        info!("Client confirmed initialization");
        Ok(serde_json::Value::Null)
    }
    
    async fn handle_list_tools(&self) -> Result<serde_json::Value> {
        if !self.initialized {
            return Err(anyhow::anyhow!("Server not initialized"));
        }
        
        let tools = self.tool_registry.list_tools();
        let response = ListToolsResponse { tools };
        
        debug!("Listing {} tools", response.tools.len());
        Ok(serde_json::to_value(response)?)
    }
    
    async fn handle_call_tool(&self, params: Option<serde_json::Value>) -> Result<serde_json::Value> {
        if !self.initialized {
            return Err(anyhow::anyhow!("Server not initialized"));
        }
        
        let request: CallToolRequest = if let Some(params) = params {
            serde_json::from_value(params)?
        } else {
            return Err(anyhow::anyhow!("Tool call request requires parameters"));
        };
        
        let response = self.tool_registry.call_tool(request).await?;
        Ok(serde_json::to_value(response)?)
    }
    
    async fn handle_ping(&self) -> Result<serde_json::Value> {
        Ok(serde_json::json!({"pong": true}))
    }
    
    async fn handle_list_resources(&self) -> Result<serde_json::Value> {
        // Return empty resources list since we don't implement resources yet
        Ok(serde_json::json!({"resources": []}))
    }
    
    async fn handle_list_prompts(&self) -> Result<serde_json::Value> {
        // Return empty prompts list since we don't implement prompts yet
        Ok(serde_json::json!({"prompts": []}))
    }
}
