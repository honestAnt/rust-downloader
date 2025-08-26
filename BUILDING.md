# Building for Multiple Platforms

This document explains how to build the downloader tool for various platforms.

## Prerequisites

Before building, ensure you have the following installed:
- Rust toolchain (latest stable version)
- Git

## Quick Build

For a quick build on your current platform:

```bash
cargo build --release
```

The binary will be located at `target/release/downloader` (or `target/release/downloader.exe` on Windows).

## Building for Multiple Platforms

### Using the Build Script

You can use our provided build script to build for multiple platforms:

```bash
./build-release.sh
```

This will:
1. Build the project for Windows, Linux, and macOS
2. Package each build with documentation
3. Create archives in the `releases` directory

### Manual Cross-Compilation

If you prefer to build manually for specific platforms:

1. Add the required targets:
   ```bash
   rustup target add x86_64-pc-windows-gnu
   rustup target add x86_64-unknown-linux-gnu
   rustup target add x86_64-apple-darwin
   ```

2. Build for each target:
   ```bash
   # For Windows
   cargo build --release --target x86_64-pc-windows-gnu
   
   # For Linux
   cargo build --release --target x86_64-unknown-linux-gnu
   
   # For macOS
   cargo build --release --target x86_64-apple-darwin
   ```

## Platform-specific Notes

### Linux

On Linux systems, the binary should run on most distributions without additional dependencies.

### Windows

For Windows builds, you might need to install the Windows GNU toolchain:
```bash
rustup toolchain install stable-x86_64-pc-windows-gnu
```

### macOS

Building for macOS might require specific versions of the toolchain. The generated binary will be compatible with the macOS version used for building and newer versions.

## Optimization

The release profile in `Cargo.toml` is configured with optimizations:
- Link Time Optimization (LTO)
- Reduced codegen units for better optimization
- Panic abort to reduce binary size
- Stripped debug symbols

These settings produce smaller and faster binaries.