#!/usr/bin/env python3
"""
Simple Python client for testing the Rust MCP Server
"""

import json
import subprocess
import time
from typing import Dict, Any, Optional, Union

class McpClient:
    def __init__(self) -> None:
        self.next_id = 1
        self.process: Optional[subprocess.Popen[str]] = None
        
    def start_server(self) -> bool:
        """Start the MCP server process with minimal logging"""
        try:
            # Use the release binary directly with quiet flag
            self.process = subprocess.Popen(
                ["../target/release/rust-mcp-server", "--quiet"],
                stdin=subprocess.PIPE,
                stdout=subprocess.PIPE,
                stderr=subprocess.PIPE,
                text=True
            )
            time.sleep(0.1)  # Give server time to start
            return True
        except Exception as e:
            print(f"Failed to start server: {e}")
            print("Make sure you've built the release binary with: cargo build --release")
            return False
    
    def stop_server(self) -> None:
        """Stop the MCP server process"""
        if self.process:
            self.process.terminate()
            self.process.wait()
            self.process = None
    
    def send_request(self, method: str, params: Optional[Dict[str, Any]] = None) -> Dict[str, Any]:
        """Send a JSON-RPC request to the server and return the response"""
        if not self.process:
            return {"error": "Server not started"}
            
        request = {
            "jsonrpc": "2.0",
            "id": self.next_id,
            "method": method,
        }
        
        if params is not None:
            request["params"] = params
            
        self.next_id += 1
        
        try:
            # Send request
            request_json = json.dumps(request)
            if self.process.stdin is None:
                return {"error": "Server stdin not available"}
            self.process.stdin.write(request_json + "\n")
            self.process.stdin.flush()
            
            # Read response
            if self.process.stdout is None:
                return {"error": "Server stdout not available"}
            response_line = self.process.stdout.readline()
            if not response_line.strip():
                return {"error": "No response from server"}
            
            try:
                response = json.loads(response_line.strip())
                return response
            except json.JSONDecodeError:
                return {"error": f"Invalid JSON response: {response_line.strip()[:100]}..."}
            
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


def test_server_initialization(client: McpClient) -> bool:
    """Test server initialization sequence."""
    print("\n1. Initializing server...")
    response = client.initialize()
    if "error" in response:
        print(f"❌ Error: {response['error']}")
        return False

    print("✅ Server initialized successfully")
    print(f"   Protocol: {response.get('result', {}).get('protocolVersion', 'unknown')}") 
    print(f"   Server: {response.get('result', {}).get('serverInfo', {}).get('name', 'unknown')}")

    print("\n2. Confirming initialization...")
    response = client.initialized()
    if "error" in response:
        print(f"❌ Error: {response['error']}")
        return False

    print("✅ Initialization confirmed")
    return True


def test_list_tools(client: McpClient) -> bool:
    """Test listing available tools."""
    print("\n3. Listing available tools...")
    response = client.list_tools()
    if "error" in response:
        print(f"❌ Error: {response['error']}")
        return False

    if "result" in response and "tools" in response["result"]:
        tools = response["result"]["tools"]
        print(f"✅ Found {len(tools)} tools:")
        for tool in tools:
            print(f"   - {tool['name']}: {tool['description']}")

    return True


def test_echo_tool(client: McpClient) -> bool:
    """Test the echo tool."""
    print("\n4. Testing echo tool...")
    response = client.call_tool("echo", {"text": "Hello from Python client!"})
    if "error" in response:
        print(f"❌ Error: {response['error']}")
        return False

    if "result" in response:
        content = response["result"].get("content", [])
        if content and content[0].get("type") == "text":
            print(f"✅ Echo response: {content[0]['text']}")
        else:
            print("✅ Echo tool called successfully")

    return True


def test_system_info(client: McpClient) -> bool:
    """Test getting system information."""
    print("\n5. Getting system information...")
    response = client.call_tool("get_system_info", {})
    if "error" in response:
        print(f"❌ Error: {response['error']}")
        return False

    if "result" in response:
        content = response["result"].get("content", [])
        if content and content[0].get("type") == "text":
            # Just show first line of system info
            sys_info = content[0]["text"].split("\n")[0]
            print(f"✅ {sys_info}")
        else:
            print("✅ System info retrieved successfully")

    return True


def test_file_listing(client: McpClient) -> bool:
    """Test file listing functionality."""
    print("\n6. Listing files in current directory...")
    response = client.call_tool("list_files", {"path": "../"})
    if "error" in response:
        print(f"❌ Error: {response['error']}")
        return False

    if "result" in response:
        content = response["result"].get("content", [])
        if content and content[0].get("type") == "text":
            lines = content[0]["text"].split("\n")
            file_count = len([l for l in lines if l.strip() and not l.startswith("Files in")])
            print(f"✅ Listed files - found {file_count} items")
        else:
            print("✅ File listing completed successfully")

    return True


def test_ping(client: McpClient) -> bool:
    """Test server ping functionality."""
    print("\n7. Testing ping...")
    response = client.ping()
    if "error" in response:
        print(f"❌ Error: {response['error']}")
        return False

    if "result" in response and response["result"].get("pong"):
        print("✅ Pong! Server is responsive")

    return True


def run_all_tests(client: McpClient) -> bool:
    """Run all test functions in sequence."""
    test_functions = [
        test_server_initialization,
        test_list_tools,
        test_echo_tool,
        test_system_info,
        test_file_listing,
        test_ping,
    ]
    
    for test_func in test_functions:
        if not test_func(client):
            return False
    
    return True


def main() -> None:
    """Main function - simplified and delegated to smaller functions."""
    print("Rust MCP Server - Python Test Client")
    print("=" * 40)
    
    client = McpClient()
    
    # Start the server
    print("Starting MCP server...")
    if not client.start_server():
        return
    
    try:
        if run_all_tests(client):
            print("\n🎉 All tests completed successfully!")
        else:
            print("\n❌ Some tests failed.")
    finally:
        # Clean up
        print("\nStopping server...")
        client.stop_server()


if __name__ == "__main__":
    main()
