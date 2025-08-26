#!/bin/bash

# 为不同平台构建二进制文件
echo "Building for multiple platforms..."

# 为Linux构建
echo "Building for Linux..."
cargo build --release --target x86_64-unknown-linux-gnu

# 为Windows构建
echo "Building for Windows..."
cargo build --release --target x86_64-pc-windows-gnu

# 为macOS构建
echo "Building for macOS..."
cargo build --release --target x86_64-apple-darwin

echo "Build complete! Check the target directory for binaries."