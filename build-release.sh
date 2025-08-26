#!/bin/bash

# 多平台构建脚本

set -e  # 遇到错误时停止执行

echo "Rust Downloader - Multi-platform Build Script"
echo "============================================"

# 检查是否安装了rustup
if command -v rustup &> /dev/null; then
    echo "Adding required targets..."
    rustup target add x86_64-pc-windows-gnu
    rustup target add x86_64-unknown-linux-gnu
    rustup target add x86_64-apple-darwin
else
    echo "Warning: rustup not found. Only building for native target."
    echo "To cross-compile for other platforms, please install rustup."
fi

# 创建发布目录
mkdir -p releases

# 为当前平台构建
echo "Building for native platform..."
cargo build --release

# 如果安装了rustup，则进行跨平台构建
if command -v rustup &> /dev/null; then
    # 为不同平台构建
    echo "Building for Windows..."
    cargo build --release --target x86_64-pc-windows-gnu

    echo "Building for Linux..."
    cargo build --release --target x86_64-unknown-linux-gnu

    echo "Building for macOS..."
    cargo build --release --target x86_64-apple-darwin

    # 复制和打包发布文件
    echo "Packaging releases..."

    # Windows
    mkdir -p releases/windows
    cp target/x86_64-pc-windows-gnu/release/downloader.exe releases/windows/
    cp README.md releases/windows/
    cp README_EN.md releases/windows/
    cp QUICKSTART.md releases/windows/
    cd releases/windows && zip -r ../downloader-windows.zip . && cd ../../..

    # Linux
    mkdir -p releases/linux
    cp target/x86_64-unknown-linux-gnu/release/downloader releases/linux/
    cp README.md releases/linux/
    cp README_EN.md releases/linux/
    cp QUICKSTART.md releases/linux/
    cd releases/linux && tar -czf ../downloader-linux.tar.gz . && cd ../../..

    # macOS
    mkdir -p releases/macos
    cp target/x86_64-apple-darwin/release/downloader releases/macos/
    cp README.md releases/macos/
    cp README_EN.md releases/macos/
    cp QUICKSTART.md releases/macos/
    cd releases/macos && tar -czf ../downloader-macos.tar.gz . && cd ../../..
else
    # 只为当前平台打包
    echo "Packaging release for native platform..."
    mkdir -p releases/native
    cp target/release/downloader releases/native/
    cp README.md releases/native/
    cp README_EN.md releases/native/
    cp QUICKSTART.md releases/native/
    cd releases/native && tar -czf ../downloader-native.tar.gz . && cd ../../..
fi

echo "Build and packaging complete!"
if command -v rustup &> /dev/null; then
    echo "Releases are available in the 'releases' directory:"
    echo "  - releases/downloader-windows.zip"
    echo "  - releases/downloader-linux.tar.gz" 
    echo "  - releases/downloader-macos.tar.gz"
else
    echo "Release is available in the 'releases' directory:"
    echo "  - releases/downloader-native.tar.gz"
fi