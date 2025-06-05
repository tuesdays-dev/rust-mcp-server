#!/bin/bash

# Test script for the Rust MCP Server
# This script demonstrates how to interact with the server via stdio

echo "Testing Rust MCP Server..."
echo "============================="

# Build the server first
echo "Building server..."
cargo build --release

if [ $? -ne 0 ]; then
    echo "Build failed!"
    exit 1
fi

echo "Server built successfully!"
echo ""

# Function to send JSON-RPC request and get response
test_request() {
    local name="$1"
    local request="$2"
    
    echo "Test: $name"
    echo "Request: $request"
    echo "Response:"
    echo "$request" | cargo run --release 2>/dev/null
    echo ""
    echo "---"
    echo ""
}

# Test 1: Initialize the server
test_request "Initialize Server" '{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "initialize",
  "params": {
    "protocolVersion": "2024-11-05",
    "capabilities": {},
    "clientInfo": {
      "name": "test-client",
      "version": "1.0.0"
    }
  }
}'

# Test 2: Confirm initialization
test_request "Confirm Initialization" '{
  "jsonrpc": "2.0",
  "id": 2,
  "method": "initialized"
}'

# Test 3: List available tools
test_request "List Tools" '{
  "jsonrpc": "2.0",
  "id": 3,
  "method": "tools/list"
}'

# Test 4: Call echo tool
test_request "Echo Tool" '{
  "jsonrpc": "2.0",
  "id": 4,
  "method": "tools/call",
  "params": {
    "name": "echo",
    "arguments": {
      "text": "Hello, MCP World!"
    }
  }
}'

# Test 5: Get system info
test_request "System Info Tool" '{
  "jsonrpc": "2.0",
  "id": 5,
  "method": "tools/call",
  "params": {
    "name": "get_system_info",
    "arguments": {}
  }
}'

# Test 6: List files in current directory
test_request "List Files Tool" '{
  "jsonrpc": "2.0",
  "id": 6,
  "method": "tools/call",
  "params": {
    "name": "list_files",
    "arguments": {
      "path": "."
    }
  }
}'

# Test 7: Execute safe command
test_request "Execute Command Tool" '{
  "jsonrpc": "2.0",
  "id": 7,
  "method": "tools/call",
  "params": {
    "name": "execute_command",
    "arguments": {
      "command": "echo",
      "args": ["Hello from command execution!"]
    }
  }
}'

# Test 8: Test ping
test_request "Ping Test" '{
  "jsonrpc": "2.0",
  "id": 8,
  "method": "ping"
}'

echo "All tests completed!"
