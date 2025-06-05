# Rust MCP Server

A Model Context Protocol (MCP) server implementation in Rust that provides various tools for interacting with the system.

## Features

- **JSON-RPC 2.0 Protocol**: Full implementation of MCP over stdio
- **Built-in Tools**: 
  - `echo`: Echo back provided text
  - `get_system_info`: Get basic system information
  - `list_files`: List files in a directory
  - `read_file`: Read file contents (with size limits)
  - `execute_command`: Execute safe system commands (restricted list)
- **Async/Await**: Built with Tokio for async operations
- **Logging**: Structured logging with tracing
- **Safety**: Restricted command execution for security

## Installation

Make sure you have Rust installed, then:

```bash
cargo build --release
```

## Usage

### Running the Server

```bash
# Run with default settings
cargo run

# Run with debug logging
cargo run -- --debug

# Custom server name and version
cargo run -- --name "my-server" --version "1.0.0"
```

### Command Line Options

- `--debug, -d`: Enable debug logging
- `--name, -n`: Set server name (default: "rust-mcp-server")
- `--version, -v`: Set server version (default: "0.1.0")
- `--help, -h`: Show help message

### Testing with MCP Client

You can test the server using any MCP-compatible client. Here's an example of the JSON-RPC messages:

1. **Initialize the server:**
```json
{
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
}
```

2. **Confirm initialization:**
```json
{
  "jsonrpc": "2.0",
  "id": 2,
  "method": "initialized"
}
```

3. **List available tools:**
```json
{
  "jsonrpc": "2.0",
  "id": 3,
  "method": "tools/list"
}
```

4. **Call a tool:**
```json
{
  "jsonrpc": "2.0",
  "id": 4,
  "method": "tools/call",
  "params": {
    "name": "echo",
    "arguments": {
      "text": "Hello, World!"
    }
  }
}
```

## Available Tools

### echo
Echo back the provided text.

**Parameters:**
- `text` (string, required): Text to echo back

**Example:**
```json
{
  "name": "echo",
  "arguments": {
    "text": "Hello, World!"
  }
}
```

### get_system_info
Get basic system information including OS, architecture, and hostname.

**Parameters:** None

### list_files
List files and directories in a specified path.

**Parameters:**
- `path` (string, optional): Directory path to list (default: ".")

**Example:**
```json
{
  "name": "list_files",
  "arguments": {
    "path": "/tmp"
  }
}
```

### read_file
Read the contents of a file with size restrictions for safety.

**Parameters:**
- `path` (string, required): Path to the file to read
- `max_size` (integer, optional): Maximum file size in bytes (default: 1MB)

**Example:**
```json
{
  "name": "read_file",
  "arguments": {
    "path": "./Cargo.toml"
  }
}
```

### execute_command
Execute safe system commands from a restricted whitelist.

**Allowed commands:** echo, date, whoami, pwd, ls, cat, head, tail, wc

**Parameters:**
- `command` (string, required): Command to execute
- `args` (array of strings, optional): Command arguments

**Example:**
```json
{
  "name": "execute_command",
  "arguments": {
    "command": "ls",
    "args": ["-la"]
  }
}
```

## Architecture

The server is organized into several modules:

- `main.rs`: CLI interface and application entry point
- `types.rs`: MCP protocol type definitions
- `mcp.rs`: Core MCP server implementation
- `server.rs`: Stdio transport layer
- `tools.rs`: Tool registry and implementations

## Security

This server implements several security measures:

1. **Command Restriction**: Only whitelisted commands can be executed
2. **File Size Limits**: File reading is limited to prevent memory exhaustion
3. **Input Validation**: All inputs are validated before processing
4. **Error Handling**: Comprehensive error handling prevents crashes

## Development

### Adding New Tools

To add a new tool:

1. Implement the `ToolHandler` trait in `tools.rs`
2. Register the tool in `ToolRegistry::new()`
3. Rebuild and test

Example:
```rust
struct MyTool;

#[async_trait::async_trait]
impl ToolHandler for MyTool {
    fn description(&self) -> String {
        "Description of my tool".to_string()
    }
    
    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "param": {
                    "type": "string",
                    "description": "Parameter description"
                }
            },
            "required": ["param"]
        })
    }
    
    async fn call(&self, args: Value) -> Result<CallToolResponse> {
        // Implementation here
    }
}
```

### Testing

```bash
# Run tests
cargo test

# Run with logging
RUST_LOG=debug cargo test

# Check formatting
cargo fmt --check

# Run clippy
cargo clippy
```

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests
5. Submit a pull request

## Troubleshooting

### Common Issues

1. **Permission Denied**: Make sure the executable has proper permissions
2. **Command Not Found**: Ensure all dependencies are installed
3. **JSON Parse Errors**: Verify JSON-RPC message format

### Debug Mode

Run with `--debug` flag to see detailed logging:

```bash
cargo run -- --debug
```

This will show all incoming and outgoing messages, making it easier to debug issues.
