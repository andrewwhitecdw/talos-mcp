#!/bin/bash

# Talos MCP Server Installation Script
# Usage: ./install.sh [options]

set -e

# Default values
VERSION="latest"
INSTALL_DIR="$HOME/.local/bin"
CONFIGURE_CURSOR=true
REPO="5dlabs/talos-mcp"
BINARY_NAME="talos-mcp-server"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Print colored message
print_msg() {
    local color=$1
    shift
    echo -e "${color}$*${NC}"
}

# Show help
show_help() {
    cat << EOF
Talos MCP Server Installer

Usage: ./install.sh [options]

Options:
    -h, --help              Show this help message
    -v, --version VERSION   Install specific version (default: latest)
    -d, --dir DIRECTORY     Installation directory (default: ~/.local/bin)
    -n, --no-config         Skip Cursor configuration
    --uninstall             Remove installed binary

Examples:
    ./install.sh                         # Install latest version
    ./install.sh --version v1.0.0        # Install specific version
    ./install.sh --dir /usr/local/bin    # Custom install directory
    ./install.sh --no-config             # Skip Cursor config

EOF
    exit 0
}

# Parse arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        -h|--help)
            show_help
            ;;
        -v|--version)
            VERSION="$2"
            shift 2
            ;;
        -d|--dir)
            INSTALL_DIR="$2"
            shift 2
            ;;
        -n|--no-config)
            CONFIGURE_CURSOR=false
            shift
            ;;
        --uninstall)
            UNINSTALL=true
            shift
            ;;
        *)
            print_msg $RED "Unknown option: $1"
            show_help
            ;;
    esac
done

# Uninstall function
uninstall() {
    print_msg $YELLOW "Uninstalling Talos MCP Server..."
    
    if [ -f "$INSTALL_DIR/$BINARY_NAME" ]; then
        rm -f "$INSTALL_DIR/$BINARY_NAME"
        print_msg $GREEN "Removed $INSTALL_DIR/$BINARY_NAME"
    else
        print_msg $YELLOW "Binary not found at $INSTALL_DIR/$BINARY_NAME"
    fi
    
    print_msg $GREEN "Uninstallation complete"
    exit 0
}

# Run uninstall if requested
if [ "$UNINSTALL" = true ]; then
    uninstall
fi

# Detect OS and architecture
detect_platform() {
    OS=$(uname -s | tr '[:upper:]' '[:lower:]')
    ARCH=$(uname -m)
    
    case $ARCH in
        x86_64|amd64)
            ARCH="x86_64"
            ;;
        aarch64|arm64)
            ARCH="aarch64"
            ;;
        *)
            print_msg $RED "Unsupported architecture: $ARCH"
            exit 1
            ;;
    esac
    
    case $OS in
        linux|darwin)
            ;;
        *)
            print_msg $RED "Unsupported OS: $OS"
            exit 1
            ;;
    esac
    
    print_msg $BLUE "Detected platform: $OS-$ARCH"
}

# Get latest version from GitHub API
get_latest_version() {
    print_msg $BLUE "Fetching latest version..."
    
    LATEST=$(curl -s "https://api.github.com/repos/$REPO/releases/latest" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')
    
    if [ -z "$LATEST" ]; then
        print_msg $YELLOW "Could not fetch latest version, using 'latest'"
        VERSION="latest"
    else
        VERSION="$LATEST"
        print_msg $BLUE "Latest version: $VERSION"
    fi
}

# Download binary
download_binary() {
    print_msg $BLUE "Downloading Talos MCP Server $VERSION..."
    
    # Determine download URL
    if [ "$VERSION" = "latest" ]; then
        URL="https://github.com/$REPO/releases/latest/download/talos-mcp-server-$OS-$ARCH"
    else
        URL="https://github.com/$REPO/releases/download/$VERSION/talos-mcp-server-$OS-$ARCH"
    fi
    
    print_msg $BLUE "Download URL: $URL"
    
    # Create temp file for download
    TEMP_FILE=$(mktemp)
    
    if ! curl -fsSL "$URL" -o "$TEMP_FILE"; then
        print_msg $RED "Failed to download binary from $URL"
        rm -f "$TEMP_FILE"
        exit 1
    fi
    
    # Make executable
    chmod +x "$TEMP_FILE"
    
    print_msg $GREEN "Download complete"
}

# Install binary
install_binary() {
    print_msg $BLUE "Installing to $INSTALL_DIR..."
    
    # Create directory if it doesn't exist
    mkdir -p "$INSTALL_DIR"
    
    # Move binary to install location
    mv "$TEMP_FILE" "$INSTALL_DIR/$BINARY_NAME"
    
    print_msg $GREEN "Installed to $INSTALL_DIR/$BINARY_NAME"
}

# Configure Cursor
configure_cursor() {
    if [ "$CONFIGURE_CURSOR" = false ]; then
        return
    fi
    
    print_msg $BLUE "Configuring Cursor..."
    
    # Cursor config paths
    CURSOR_CONFIG="$HOME/.cursor/mcp.json"
    
    # Check if TALOSCONFIG is set
    TALOSCONFIG_PATH="${TALOSCONFIG:-$HOME/.talos/config}"
    
    # Create MCP config if it doesn't exist
    if [ ! -f "$CURSOR_CONFIG" ]; then
        mkdir -p "$(dirname "$CURSOR_CONFIG")"
        echo '{"mcpServers":{}}' > "$CURSOR_CONFIG"
    fi
    
    # Check if jq is available
    if command -v jq &> /dev/null; then
        # Use jq to add/update the talos-mcp config
        TMP_FILE=$(mktemp)
        jq --arg bin "$INSTALL_DIR/$BINARY_NAME" --arg tc "$TALOSCONFIG_PATH" \
            '.mcpServers["talos-mcp"] = {"command": $bin, "env": {"TALOSCONFIG": $tc}}' \
            "$CURSOR_CONFIG" > "$TMP_FILE"
        mv "$TMP_FILE" "$CURSOR_CONFIG"
        print_msg $GREEN "Cursor configuration updated"
    else
        print_msg $YELLOW "jq not found. Please manually add the following to $CURSOR_CONFIG:"
        echo ""
        echo '{
  "mcpServers": {
    "talos-mcp": {
      "command": "'$INSTALL_DIR/$BINARY_NAME'",
      "env": {
        "TALOSCONFIG": "'$TALOSCONFIG_PATH'"
      }
    }
  }
}'
    fi
}

# Verify installation
verify_installation() {
    print_msg $BLUE "Verifying installation..."
    
    if ! command -v "$INSTALL_DIR/$BINARY_NAME" &> /dev/null; then
        print_msg $RED "Binary not found in PATH"
        print_msg $YELLOW "Add $INSTALL_DIR to your PATH:"
        echo ""
        echo "    export PATH=\"$INSTALL_DIR:\$PATH\""
        echo ""
        return
    fi
    
    INSTALLED_VERSION=$("$INSTALL_DIR/$BINARY_NAME" --version 2>/dev/null || echo "unknown")
    print_msg $GREEN "Installation verified: $INSTALLED_VERSION"
}

# Check prerequisites
check_prerequisites() {
    print_msg $BLUE "Checking prerequisites..."
    
    # Check for talosctl
    if ! command -v talosctl &> /dev/null; then
        print_msg $YELLOW "talosctl not found. Install it from: https://www.talos.dev/latest/introduction/getting-started/"
    else
        print_msg $GREEN "talosctl found: $(talosctl version --client 2>/dev/null | head -1 || echo 'installed')"
    fi
    
    # Check for TALOSCONFIG
    if [ -z "$TALOSCONFIG" ]; then
        print_msg $YELLOW "TALOSCONFIG not set. Default location: ~/.talos/config"
    else
        print_msg $GREEN "TALOSCONFIG: $TALOSCONFIG"
    fi
}

# Main installation flow
main() {
    print_msg $GREEN ""
    print_msg $GREEN "╔══════════════════════════════════════════╗"
    print_msg $GREEN "║     Talos MCP Server Installer          ║"
    print_msg $GREEN "╚══════════════════════════════════════════╝"
    print_msg $GREEN ""
    
    detect_platform
    
    if [ "$VERSION" = "latest" ]; then
        get_latest_version
    fi
    
    download_binary
    install_binary
    verify_installation
    check_prerequisites
    configure_cursor
    
    print_msg $GREEN ""
    print_msg $GREEN "╔══════════════════════════════════════════╗"
    print_msg $GREEN "║       Installation Complete!            ║"
    print_msg $GREEN "╚══════════════════════════════════════════╝"
    print_msg $GREEN ""
    print_msg $BLUE "Binary installed to: $INSTALL_DIR/$BINARY_NAME"
    print_msg $BLUE "Documentation: https://github.com/$REPO"
    print_msg $GREEN ""
}

main
