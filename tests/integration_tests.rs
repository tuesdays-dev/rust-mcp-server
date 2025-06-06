use rust_mcp_server::mcp::McpServer;
use rust_mcp_server::types::*;
use serde_json::json;

#[tokio::test]
async fn test_mcp_server_initialization() {
    let mut server = McpServer::new("test-server".to_string(), "1.0.0".to_string());
    
    let init_request = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        id: Some(json!(1)),
        method: "initialize".to_string(),
        params: Some(json!({
            "protocolVersion": "2024-11-05",
            "capabilities": {},
            "clientInfo": {
                "name": "test-client",
                "version": "1.0.0"
            }
        })),
    };
    
    let response = server.handle_request(init_request).await.unwrap().unwrap();
    
    assert_eq!(response.jsonrpc, "2.0");
    assert_eq!(response.id, Some(json!(1)));
    assert!(response.result.is_some());
    assert!(response.error.is_none());
    assert!(server.initialized);
}

#[tokio::test]
async fn test_list_tools() {
    let mut server = McpServer::new("test-server".to_string(), "1.0.0".to_string());
    server.initialized = true; // Skip initialization for this test
    
    let request = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        id: Some(json!(2)),
        method: "tools/list".to_string(),
        params: None,
    };
    
    let response = server.handle_request(request).await.unwrap().unwrap();
    
    assert_eq!(response.jsonrpc, "2.0");
    assert_eq!(response.id, Some(json!(2)));
    assert!(response.result.is_some());
    assert!(response.error.is_none());
    
    // Verify we have the expected tools
    if let Some(result) = response.result {
        let tools_response: ListToolsResponse = serde_json::from_value(result).unwrap();
        assert!(!tools_response.tools.is_empty());
        
        let tool_names: Vec<&str> = tools_response.tools.iter().map(|t| t.name.as_str()).collect();
        assert!(tool_names.contains(&"echo"));
        assert!(tool_names.contains(&"get_system_info"));
        assert!(tool_names.contains(&"list_files"));
        assert!(tool_names.contains(&"read_file"));
        assert!(tool_names.contains(&"execute_command"));
    }
}

#[tokio::test]
async fn test_echo_tool() {
    let mut server = McpServer::new("test-server".to_string(), "1.0.0".to_string());
    server.initialized = true;
    
    let request = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        id: Some(json!(3)),
        method: "tools/call".to_string(),
        params: Some(json!({
            "name": "echo",
            "arguments": {
                "text": "Hello, World!"
            }
        })),
    };
    
    let response = server.handle_request(request).await.unwrap().unwrap();
    
    assert_eq!(response.jsonrpc, "2.0");
    assert_eq!(response.id, Some(json!(3)));
    assert!(response.result.is_some());
    assert!(response.error.is_none());
    
    if let Some(result) = response.result {
        let tool_response: CallToolResponse = serde_json::from_value(result).unwrap();
        assert!(!tool_response.content.is_empty());
        
        if let ToolContent::Text { text } = &tool_response.content[0] {
            assert!(text.contains("Echo: Hello, World!"));
        } else {
            panic!("Expected text content");
        }
    }
}

#[tokio::test]
async fn test_method_not_found() {
    let mut server = McpServer::new("test-server".to_string(), "1.0.0".to_string());
    
    let request = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        id: Some(json!(4)),
        method: "nonexistent_method".to_string(),
        params: None,
    };
    
    let response = server.handle_request(request).await.unwrap().unwrap();
    
    assert_eq!(response.jsonrpc, "2.0");
    assert_eq!(response.id, Some(json!(4)));
    assert!(response.result.is_none());
    assert!(response.error.is_some());
    
    if let Some(error) = response.error {
        assert_eq!(error.code, -32601); // METHOD_NOT_FOUND
    }
}

#[tokio::test]
async fn test_ping() {
    let mut server = McpServer::new("test-server".to_string(), "1.0.0".to_string());
    
    let request = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        id: Some(json!(5)),
        method: "ping".to_string(),
        params: None,
    };
    
    let response = server.handle_request(request).await.unwrap().unwrap();
    
    assert_eq!(response.jsonrpc, "2.0");
    assert_eq!(response.id, Some(json!(5)));
    assert!(response.result.is_some());
    assert!(response.error.is_none());
    
    if let Some(result) = response.result {
        let ping_response: serde_json::Value = result;
        assert_eq!(ping_response["pong"], json!(true));
    }
}
