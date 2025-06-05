# 🚀 Quick Start: Using Rust MCP Server with Claude Desktop

This guide will get you up and running with the Rust MCP Server in Claude Desktop in just a few minutes.

## ⚡ Quick Setup (3 Steps)

### 1. Build the Server
```bash
cd /path/to/rust-mcp
cargo build --release
```

### 2. Generate Configuration
```bash
./claude_config_helper.sh
```
This shows you the exact configuration to copy.

### 3. Configure Claude Desktop
- **macOS**: Open `~/Library/Application Support/Claude/claude_desktop_config.json`
- **Windows**: Open `%APPDATA%\Claude\claude_desktop_config.json`

Paste the configuration from step 2, save the file, and restart Claude Desktop.

## 🎯 Test the Connection

Once Claude Desktop restarts:
1. Look for the 🔌 (plug) icon in the interface
2. Try asking: **"Can you echo 'Hello World' for me?"**
3. If it works, you'll see Claude use the echo tool and return "Echo: Hello World"

## 🛠️ Available Tools

Ask Claude to:
- **"Echo some text"** → Uses `echo` tool
- **"What's my system info?"** → Uses `get_system_info` tool  
- **"List files in current directory"** → Uses `list_files` tool
- **"Read my README.md file"** → Uses `read_file` tool
- **"Run the date command"** → Uses `execute_command` tool

## 🔧 Example Configuration

```json
{
  "mcpServers": {
    "rust-mcp-server": {
      "command": "/Users/username/projects/rust-mcp/target/release/rust-mcp-server",
      "args": ["--name", "rust-tools"],
      "env": {}
    }
  }
}
```

## 🐛 Troubleshooting

**Not connecting?**
- Use absolute paths in the configuration
- Make sure the executable exists: `ls -la target/release/rust-mcp-server`
- Try adding `--debug` to the args array

**Tools not working?**
- Check Claude Desktop's logs
- Verify file permissions
- Ensure you completely quit and restarted Claude Desktop

## 🎉 Success!

If everything is working, you should see Claude using your Rust MCP tools to interact with your system. The server provides safe, controlled access to system operations through Claude's natural language interface.

---

*For detailed documentation, see the main [README.md](README.md)*
