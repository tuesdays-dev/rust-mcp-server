use crate::types::{CallToolRequest, CallToolResponse, Tool, ToolContent};
use anyhow::Result;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::process::Command;
use tracing::debug;

pub struct ToolRegistry {
    tools: HashMap<String, Box<dyn ToolHandler>>,
}

impl ToolRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            tools: HashMap::new(),
        };
        
        // Register built-in tools
        registry.register_tool("echo", Box::new(EchoTool));
        registry.register_tool("get_system_info", Box::new(SystemInfoTool));
        registry.register_tool("list_files", Box::new(ListFilesTool));
        registry.register_tool("read_file", Box::new(ReadFileTool));
        registry.register_tool("execute_command", Box::new(ExecuteCommandTool));
        
        registry
    }
    
    pub fn register_tool(&mut self, name: &str, handler: Box<dyn ToolHandler>) {
        self.tools.insert(name.to_string(), handler);
    }
    
    pub fn list_tools(&self) -> Vec<Tool> {
        self.tools.iter().map(|(name, handler)| {
            Tool {
                name: name.clone(),
                description: handler.description(),
                input_schema: handler.input_schema(),
            }
        }).collect()
    }
    
    pub async fn call_tool(&self, request: CallToolRequest) -> Result<CallToolResponse> {
        debug!("Calling tool: {}", request.name);
        
        if let Some(handler) = self.tools.get(&request.name) {
            handler.call(request.arguments.unwrap_or(json!({}))).await
        } else {
            Ok(CallToolResponse {
                content: vec![ToolContent::Text {
                    text: format!("Tool '{}' not found", request.name),
                }],
                is_error: Some(true),
            })
        }
    }
}

#[async_trait::async_trait]
pub trait ToolHandler: Send + Sync {
    fn description(&self) -> String;
    fn input_schema(&self) -> Value;
    async fn call(&self, args: Value) -> Result<CallToolResponse>;
}

// Echo tool - simple example
struct EchoTool;

#[async_trait::async_trait]
impl ToolHandler for EchoTool {
    fn description(&self) -> String {
        "Echo back the provided text".to_string()
    }
    
    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "text": {
                    "type": "string",
                    "description": "Text to echo back"
                }
            },
            "required": ["text"]
        })
    }
    
    async fn call(&self, args: Value) -> Result<CallToolResponse> {
        let text = args.get("text")
            .and_then(|v| v.as_str())
            .unwrap_or("No text provided");
            
        Ok(CallToolResponse {
            content: vec![ToolContent::Text {
                text: format!("Echo: {}", text),
            }],
            is_error: None,
        })
    }
}

// System info tool
struct SystemInfoTool;

#[async_trait::async_trait]
impl ToolHandler for SystemInfoTool {
    fn description(&self) -> String {
        "Get basic system information".to_string()
    }
    
    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {},
            "additionalProperties": false
        })
    }
    
    async fn call(&self, _args: Value) -> Result<CallToolResponse> {
        let os = std::env::consts::OS;
        let arch = std::env::consts::ARCH;
        let hostname = gethostname::gethostname();
        
        let info = format!(
            "System Information:\n- OS: {}\n- Architecture: {}\n- Hostname: {}",
            os, 
            arch, 
            hostname.to_string_lossy()
        );
        
        Ok(CallToolResponse {
            content: vec![ToolContent::Text { text: info }],
            is_error: None,
        })
    }
}

// List files tool
struct ListFilesTool;

#[async_trait::async_trait]
impl ToolHandler for ListFilesTool {
    fn description(&self) -> String {
        "List files in a directory".to_string()
    }
    
    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "path": {
                    "type": "string",
                    "description": "Directory path to list",
                    "default": "."
                }
            }
        })
    }
    
    async fn call(&self, args: Value) -> Result<CallToolResponse> {
        let path = args.get("path")
            .and_then(|v| v.as_str())
            .unwrap_or(".");
            
        match std::fs::read_dir(path) {
            Ok(entries) => {
                let mut files = Vec::new();
                for entry in entries {
                    if let Ok(entry) = entry {
                        let name = entry.file_name().to_string_lossy().to_string();
                        let file_type = if entry.file_type().map(|t| t.is_dir()).unwrap_or(false) {
                            "directory"
                        } else {
                            "file"
                        };
                        files.push(format!("{} ({})", name, file_type));
                    }
                }
                
                let result = if files.is_empty() {
                    "Directory is empty".to_string()
                } else {
                    format!("Files in {}:\n{}", path, files.join("\n"))
                };
                
                Ok(CallToolResponse {
                    content: vec![ToolContent::Text { text: result }],
                    is_error: None,
                })
            }
            Err(e) => {
                Ok(CallToolResponse {
                    content: vec![ToolContent::Text {
                        text: format!("Error listing directory: {}", e),
                    }],
                    is_error: Some(true),
                })
            }
        }
    }
}

// Read file tool
struct ReadFileTool;

#[async_trait::async_trait]
impl ToolHandler for ReadFileTool {
    fn description(&self) -> String {
        "Read the contents of a file".to_string()
    }
    
    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "path": {
                    "type": "string",
                    "description": "Path to the file to read"
                },
                "max_size": {
                    "type": "integer",
                    "description": "Maximum file size to read in bytes",
                    "default": 1048576
                }
            },
            "required": ["path"]
        })
    }
    
    async fn call(&self, args: Value) -> Result<CallToolResponse> {
        let path = args.get("path")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Path is required"))?;
            
        let max_size = args.get("max_size")
            .and_then(|v| v.as_u64())
            .unwrap_or(1048576); // 1MB default
            
        match std::fs::metadata(path) {
            Ok(metadata) => {
                if metadata.len() > max_size {
                    return Ok(CallToolResponse {
                        content: vec![ToolContent::Text {
                            text: format!("File is too large ({} bytes, max: {} bytes)", metadata.len(), max_size),
                        }],
                        is_error: Some(true),
                    });
                }
                
                match std::fs::read_to_string(path) {
                    Ok(content) => {
                        Ok(CallToolResponse {
                            content: vec![ToolContent::Text {
                                text: format!("Contents of {}:\n{}", path, content),
                            }],
                            is_error: None,
                        })
                    }
                    Err(e) => {
                        Ok(CallToolResponse {
                            content: vec![ToolContent::Text {
                                text: format!("Error reading file: {}", e),
                            }],
                            is_error: Some(true),
                        })
                    }
                }
            }
            Err(e) => {
                Ok(CallToolResponse {
                    content: vec![ToolContent::Text {
                        text: format!("Error accessing file: {}", e),
                    }],
                    is_error: Some(true),
                })
            }
        }
    }
}

// Execute command tool (with safety restrictions)
struct ExecuteCommandTool;

#[async_trait::async_trait]
impl ToolHandler for ExecuteCommandTool {
    fn description(&self) -> String {
        "Execute a safe system command (restricted for security)".to_string()
    }
    
    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "command": {
                    "type": "string",
                    "description": "Command to execute"
                },
                "args": {
                    "type": "array",
                    "items": {
                        "type": "string"
                    },
                    "description": "Command arguments"
                }
            },
            "required": ["command"]
        })
    }
    
    async fn call(&self, args: Value) -> Result<CallToolResponse> {
        let command = args.get("command")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Command is required"))?;
            
        // Safety: Only allow specific safe commands
        let allowed_commands = vec!["echo", "date", "whoami", "pwd", "ls", "cat", "head", "tail", "wc"];
        
        if !allowed_commands.contains(&command) {
            return Ok(CallToolResponse {
                content: vec![ToolContent::Text {
                    text: format!("Command '{}' is not allowed. Allowed commands: {}", 
                        command, allowed_commands.join(", ")),
                }],
                is_error: Some(true),
            });
        }
        
        let cmd_args: Vec<String> = args.get("args")
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect())
            .unwrap_or_default();
            
        match Command::new(command).args(&cmd_args).output() {
            Ok(output) => {
                let stdout = String::from_utf8_lossy(&output.stdout);
                let stderr = String::from_utf8_lossy(&output.stderr);
                
                let result = if !stderr.is_empty() {
                    format!("Command: {} {}\nSTDOUT:\n{}\nSTDERR:\n{}", 
                        command, cmd_args.join(" "), stdout, stderr)
                } else {
                    format!("Command: {} {}\nOutput:\n{}", 
                        command, cmd_args.join(" "), stdout)
                };
                
                Ok(CallToolResponse {
                    content: vec![ToolContent::Text { text: result }],
                    is_error: if output.status.success() { None } else { Some(true) },
                })
            }
            Err(e) => {
                Ok(CallToolResponse {
                    content: vec![ToolContent::Text {
                        text: format!("Error executing command: {}", e),
                    }],
                    is_error: Some(true),
                })
            }
        }
    }
}
