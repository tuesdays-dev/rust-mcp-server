#!/usr/bin/env python3
"""
Simple Python client for testing the Rust MCP Server
"""

import json
import subprocess
import sys
from typing import Dict, Any, Optional

class McpClient:
    def __init__(self, server_path: str = "cargo run --release"):
        self.server_path = server_path
        self.next_id = 1
        
    def send_request(self, method: str, params: Optional[Dict[str, Any]] = None) -> Dict[str, Any]:
        """Send a JSON-RPC request to the server and return the response"""
        request = {
            "jsonrpc": "2.0",
            "id": self.next_id,
            "method": method,
        }
        
        if params is not None:
            request["params"] = params
            
        self.next_id += 1
        
        # Send request to server
        try:
            process = subprocess.Popen(
                self.server_path.split(),
                stdin=subprocess.PIPE,
                stdout=subprocess.PIPE,
                stderr=subprocess.PIPE,
                text=True,
                cwd=".."  # Run from parent directory
            )
            
            stdout, stderr = process.communicate(json.dumps(request) + "\n")
            
            if stderr:
                print(f"Server stderr: {stderr}", file=sys.stderr)
            
            if stdout.strip():
                return json.loads(stdout.strip())
            else:
                return {"error": "No response from server"}
                
        except Exception as e:
            return {"error": f"Failed to communicate with server: {e}"}
    
    def initialize(self) -> Dict[str, Any]:
        """Initialize the MCP server"""
        return self.send_request("initialize", {
            "protocolVersion": "2024-11-05",
            "capabilities": {},
            "clientInfo": {
                "name": "python-test-client",
                "version": "1.0.0"
            }
        })
    
    def initialized(self) -> Dict[str, Any]:
        """Confirm initialization"""
        return self.send_request("initialized")
    
    def list_tools(self) -> Dict[str, Any]:
        """List available tools"""
        return self.send_request("tools/list")
    
    def call_tool(self, name: str, arguments: Dict[str, Any]) -> Dict[str, Any]:
        """Call a specific tool"""
        return self.send_request("tools/call", {
            "name": name,
            "arguments": arguments
        })
    
    def ping(self) -> Dict[str, Any]:
        """Ping the server"""
        return self.send_request("ping")

def main():
    print("Rust MCP Server - Python Test Client")
    print("=" * 40)
    
    client = McpClient()
    
    # Test sequence
    print("1. Initializing server...")
    response = client.initialize()
    print(f"Response: {json.dumps(response, indent=2)}")
    print()
    
    print("2. Confirming initialization...")
    response = client.initialized()
    print(f"Response: {json.dumps(response, indent=2)}")
    print()
    
    print("3. Listing available tools...")
    response = client.list_tools()
    print(f"Response: {json.dumps(response, indent=2)}")
    
    if "result" in response and "tools" in response["result"]:
        tools = response["result"]["tools"]
        print(f"\nFound {len(tools)} tools:")
        for tool in tools:
            print(f"  - {tool['name']}: {tool['description']}")
    print()
    
    print("4. Testing echo tool...")
    response = client.call_tool("echo", {"text": "Hello from Python client!"})
    print(f"Response: {json.dumps(response, indent=2)}")
    print()
    
    print("5. Getting system information...")
    response = client.call_tool("get_system_info", {})
    print(f"Response: {json.dumps(response, indent=2)}")
    print()
    
    print("6. Listing files in current directory...")
    response = client.call_tool("list_files", {"path": "../"})
    print(f"Response: {json.dumps(response, indent=2)}")
    print()
    
    print("7. Testing ping...")
    response = client.ping()
    print(f"Response: {json.dumps(response, indent=2)}")
    print()
    
    print("All tests completed!")

if __name__ == "__main__":
    main()
