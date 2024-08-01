#!/bin/bash

# Build for the current platform
cargo build --release

# Build for Windows
cargo build --release --target x86_64-pc-windows-gnu

# Build for macOS (only works on macOS)
if [[ "$OSTYPE" == "darwin"* ]]; then
    cargo build --release --target x86_64-apple-darwin
fi

# Create distribution directories
mkdir -p dist/windows
mkdir -p dist/macos
mkdir -p dist/linux

# Copy binaries to distribution directories
cp target/release/cognito-auth-cli dist/linux/
cp target/x86_64-pc-windows-gnu/release/cognito-auth-cli.exe dist/windows/
if [[ "$OSTYPE" == "darwin"* ]]; then
    cp target/x86_64-apple-darwin/release/cognito-auth-cli dist/macos/
fi

# Copy .env files
cp .env* dist/linux/
cp .env* dist/windows/
cp .env* dist/macos/

echo "Build complete. Binaries are in the 'dist' directory."