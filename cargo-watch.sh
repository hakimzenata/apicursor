#!/bin/bash

# Install cargo-watch if not already installed
if ! command -v cargo-watch &> /dev/null; then
    echo "🔧 Installing cargo-watch..."
    cargo install cargo-watch
fi

# Install cargo-check if not already installed
if ! command -v cargo-check &> /dev/null; then
    echo "🔧 Installing cargo-check..."
    cargo install cargo-check
fi

# Install rustfmt if not already installed
if ! rustup component list | grep -q "rustfmt"; then
    echo "🔧 Installing rustfmt..."
    rustup component add rustfmt
fi

# Install clippy if not already installed
if ! rustup component list | grep -q "clippy"; then
    echo "🔧 Installing clippy..."
    rustup component add clippy
fi

echo "🚀 Starting cargo watch..."
echo "👀 Watching for changes..."
echo "✨ Will run: check → fmt → clippy → test"

# Run cargo watch with multiple commands
# -x: execute command
# -c: clear screen before each run
# -q: suppress output from cargo-watch itself
cargo watch \
    -c \
    -q \
    -x check \
    -x "fmt --all -- --check" \
    -x clippy \
    -x test \
    -x run
