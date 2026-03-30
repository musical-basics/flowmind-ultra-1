#!/bin/bash

# FlowMind Ultra Swarm Launcher
# (c) 2026 FlowMind Architectural Engine

APP_DIR="flowmind-ultra"
ENV_FILE="$APP_DIR/.env.local"

echo "----------------------------------------------------"
echo "  🌪️  FLOWMIND ULTRA: ARCHITECTURAL SWARM ENTRANCE"
echo "----------------------------------------------------"

# 0. Force Compatible Node.js v22.21.1 (Mac Studio Fix)
NODE_V22_BIN="/Users/lionelyu/.nvm/versions/node/v22.21.1/bin"
if [ -d "$NODE_V22_BIN" ]; then
    export PATH="$NODE_V22_BIN:$PATH"
    # Verify version
    NODE_VERSION=$(node -v)
    echo "🔍 [SESSION] Enforced Node.js $NODE_VERSION"
    if [[ "$NODE_VERSION" != v22* ]]; then
        echo "❌ [ERROR] Failed to enforce Node v22. Current: $NODE_VERSION"
        exit 1
    fi
fi

# 0.5 Detect Rust/Cargo Toolchain (Mac/Unix Fix)
if [ -d "$HOME/.cargo/bin" ]; then
    export PATH="$HOME/.cargo/bin:$PATH"
fi
if [ -d "/opt/homebrew/bin" ]; then
    export PATH="/opt/homebrew/bin:$PATH"
fi
if [ -d "/usr/local/bin" ]; then
    export PATH="/usr/local/bin:$PATH"
fi

# 0.7 Ensure Protobuf Compiler (protoc) for Swarm Memory
if ! command -v protoc &> /dev/null; then
    echo "❌ [ERROR] protoc is not installed. Please run 'brew install protobuf'."
    exit 1
fi

if ! command -v cmake &> /dev/null; then
    echo "❌ [ERROR] cmake is not installed. Please run 'brew install cmake'."
    exit 1
fi

# 1. Load Session Environment (Supabase / LLM Keys)
if [ -f "$ENV_FILE" ]; then
    echo "🔍 [SESSION] Loading .env.local configuration..."
    # Gracefully export variables without breaking on spaces/comments
    while read -r line || [ -n "$line" ]; do
        [[ "$line" =~ ^#.*$ ]] && continue
        [[ -z "$line" ]] && continue
        export "$line"
    done < "$ENV_FILE"
else
    echo "⚠️  [WARNING] .env.local not found. Collaborative mode disabled."
fi

# 2. Check for Dependencies
if ! command -v pnpm &> /dev/null; then
    echo "❌ [ERROR] pnpm is not installed. Please install it to continue."
    exit 1
fi

# 3. Launch Platform
echo "🚀 [LAUNCH] Starting Tauri 2.0 Swarm Handover..."
cd "$APP_DIR" || exit
pnpm tauri dev
