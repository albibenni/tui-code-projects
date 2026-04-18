#!/bin/bash

# Define binary name from Cargo.toml
BINARY_NAME="new-project-tui"

# Get the project root relative to this script
PROJECT_ROOT="$(cd "$(dirname "$0")/.." && pwd)"

echo "🚀 Installing $BINARY_NAME from $PROJECT_ROOT..."

# 1. Install the binary
if cargo install --path "$PROJECT_ROOT"; then
    echo "✅ Binary installed to ~/.cargo/bin/"
else
    echo "❌ Failed to install binary. Make sure Rust/Cargo is installed."
    exit 1
fi

# 2. Get desired alias
read -p "Enter desired alias (default: np): " ALIAS_NAME
ALIAS_NAME=${ALIAS_NAME:-np}

# 3. Detect Shell and Config File
CURRENT_SHELL=$(basename "$SHELL")
CONFIG_FILE=""

case "$CURRENT_SHELL" in
    zsh)
        CONFIG_FILE="$HOME/.zshrc"
        ;;
    bash)
        if [ -f "$HOME/.bashrc" ]; then
            CONFIG_FILE="$HOME/.bashrc"
        elif [ -f "$HOME/.bash_profile" ]; then
            CONFIG_FILE="$HOME/.bash_profile"
        fi
        ;;
    fish)
        CONFIG_FILE="$HOME/.config/fish/config.fish"
        ;;
    *)
        echo "⚠️  Unsupported shell: $CURRENT_SHELL"
        read -p "Enter path to your shell config file (e.g., ~/.bashrc): " CONFIG_FILE
        ;;
esac

# 4. Add alias to config
if [ -n "$CONFIG_FILE" ]; then
    # Expand ~ to full path for check
    CONFIG_FILE_EXPANDED="${CONFIG_FILE/#\~/$HOME}"
    
    if [ ! -f "$CONFIG_FILE_EXPANDED" ]; then
        touch "$CONFIG_FILE_EXPANDED"
    fi

    if grep -q "alias $ALIAS_NAME=" "$CONFIG_FILE_EXPANDED" 2>/dev/null; then
        echo "ℹ️  Alias '$ALIAS_NAME' already exists in $CONFIG_FILE. Skipping..."
    else
        echo "" >> "$CONFIG_FILE_EXPANDED"
        echo "# Alias for $BINARY_NAME" >> "$CONFIG_FILE_EXPANDED"
        echo "alias $ALIAS_NAME='$BINARY_NAME'" >> "$CONFIG_FILE_EXPANDED"
        echo "✅ Added alias '$ALIAS_NAME' to $CONFIG_FILE"
    fi
fi

# 5. Check PATH
if [[ ":$PATH:" != *":$HOME/.cargo/bin:"* ]]; then
    echo "⚠️  Warning: ~/.cargo/bin is not in your PATH."
    echo "   Add 'export PATH=\"\$HOME/.cargo/bin:\$PATH\"' to your $CONFIG_FILE"
fi

echo "🎉 Done! Please run 'source $CONFIG_FILE' or restart your terminal."
