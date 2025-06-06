#!/bin/bash

# Helper script to generate Claude Desktop MCP configuration
# Run this from your rust-mcp project directory

PROJECT_DIR=$(pwd)
EXECUTABLE_PATH="$PROJECT_DIR/target/release/rust-mcp-server"

echo "🔧 Rust MCP Server Configuration Helper"
echo "======================================="
echo ""

# Check if the executable exists
if [ ! -f "$EXECUTABLE_PATH" ]; then
    echo "❌ Executable not found at: $EXECUTABLE_PATH"
    echo "🔨 Please build the project first:"
    echo "   cargo build --release"
    echo ""
    exit 1
fi

echo "✅ Found executable at: $EXECUTABLE_PATH"
echo ""

# Determine the OS and show the config file location
echo "📁 Claude Desktop configuration file location:"
case "$(uname -s)" in
    Darwin*)
        CONFIG_PATH="~/Library/Application Support/Claude/claude_desktop_config.json"
        echo "   macOS: $CONFIG_PATH"
        ;;
    CYGWIN*|MINGW32*|MSYS*|MINGW*)
        CONFIG_PATH="%APPDATA%\\Claude\\claude_desktop_config.json"
        echo "   Windows: $CONFIG_PATH"
        ;;
    *)
        echo "   ⚠️  Claude Desktop is only available for macOS and Windows"
        echo "   Please use macOS or Windows to run Claude Desktop with MCP servers"
        ;;
esac

echo ""
echo "📋 Add this configuration to your claude_desktop_config.json:"
echo ""

# Generate the JSON configuration
cat << EOF
{
  "mcpServers": {
    "rust-mcp-server": {
      "command": "$EXECUTABLE_PATH",
      "args": ["--name", "rust-tools", "--quiet"],
      "env": {}
    }
  }
}
EOF

echo ""
echo "🚀 Optional: For debug mode, use this configuration instead:"
echo ""

cat << EOF
{
  "mcpServers": {
    "rust-mcp-server": {
      "command": "$EXECUTABLE_PATH",
      "args": ["--name", "rust-tools", "--quiet", "--debug"],
      "env": {}
    }
  }
}
EOF

echo ""
echo "📝 Next steps:"
echo "1. Copy one of the configurations above"
echo "2. Add it to your claude_desktop_config.json file"
echo "3. Completely quit and restart Claude Desktop"
echo "4. Look for the 🔌 icon in Claude to confirm connection"
echo ""
echo "💡 Test the connection by asking Claude: 'Can you echo Hello World for me?'"
