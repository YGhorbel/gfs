#!/usr/bin/env bash

set -e

echo "Building GFS Website..."

# Check if trunk is installed
if ! command -v trunk &> /dev/null; then
    echo "Error: trunk is not installed. Install it with: cargo install trunk"
    exit 1
fi

# Check if wasm32-unknown-unknown target is installed
if ! rustup target list --installed | grep -q "wasm32-unknown-unknown"; then
    echo "Installing wasm32-unknown-unknown target..."
    rustup target add wasm32-unknown-unknown
fi

# Build the website
# Using debug build to avoid wasm-opt bulk memory issues
# The WASM binary is still optimized by rustc
trunk build

echo "✅ Build complete! Output is in dist/"
echo "To serve locally: trunk serve"
