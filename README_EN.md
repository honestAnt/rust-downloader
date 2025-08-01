[中文](README.md)|**English**
# High-Performance Multi-File Downloader

A powerful Rust-based multi-file downloader with concurrent downloads, resume support, progress tracking, and performance optimizations.

## 🚀 Features

- ✅ **Concurrent Downloads**: Download multiple files simultaneously
- ✅ **High-Performance**: Optimized for maximum download speed
- ✅ **Resume Support**: Resume interrupted downloads
- ✅ **Progress Tracking**: Real-time progress bars for overall and individual files
- ✅ **Auto Filename Extraction**: Automatically extract filenames from URLs
- ✅ **Custom Output Names**: Specify custom output filenames
- ✅ **Error Handling**: Comprehensive error handling and statistics
- ✅ **Large File Support**: Efficient handling of large files
- ✅ **Connection Pooling**: Optimized connection management
- ✅ **Buffer Optimization**: Configurable buffer sizes for performance
- ✅ **TCP Keep-Alive**: Maintain persistent connections
- ✅ **Configurable Timeouts**: Adjustable timeout settings

## 📦 Installation

Ensure you have Rust and Cargo installed, then run in the project directory:

```bash
cargo build --release
```

## 🎯 Usage

### Basic Usage

```bash
# Download a single file
./target/release/util -u "https://example.com/file.zip"

# Download multiple files
./target/release/util -u "https://example.com/file1.zip" "https://example.com/file2.zip" "https://example.com/file3.zip"

# Specify output filename (single file)
./target/release/util -u "https://example.com/file.zip" -o "my_file.zip"

# Specify output filename (multiple files will auto-add numbers)
./target/release/util -u "https://example.com/file1.zip" "https://example.com/file2.zip" -o "download"

# Enable resume support
./target/release/util -u "https://example.com/file.zip" -r

# Show verbose information
./target/release/util -u "https://example.com/file.zip" -v
```

### High-Performance Configuration

```bash
# Set high concurrency (20 concurrent downloads)
./target/release/util -u "https://example.com/file1.zip" "https://example.com/file2.zip" -t 20

# Set connections per file (5 connections)
./target/release/util -u "https://example.com/file.zip" -c 5

# Set large buffer size (16KB)
./target/release/util -u "https://example.com/file.zip" -b 16384

# Set timeout (10 minutes)
./target/release/util -u "https://example.com/file.zip" --timeout 600
```

### Command Line Options

- `-u, --url`: Download URLs (supports multiple URLs, space-separated)
- `-o, --output`: Output filename (optional)
- `-t, --threads`: Number of concurrent downloads (default: 10)
- `-c, --connections`: Connections per file (default: 3)
- `-v, --verbose`: Show detailed information
- `-r, --resume`: Enable resume support
- `--timeout`: Download timeout in seconds (default: 300)
- `-b, --buffer-size`: Buffer size in bytes (default: 8192)

### Performance Optimization Examples

```bash
# Download multiple large files with high-performance settings
./target/release/util \
  -u "https://speed.hetzner.de/100MB.bin" \
     "https://speed.hetzner.de/1GB.bin" \
     "https://speed.hetzner.de/10GB.bin" \
  -t 20 \
  -c 5 \
  -b 32768 \
  --timeout 1800 \
  -r

# Batch download images
./target/release/util \
  -u "https://example.com/image1.jpg" \
     "https://example.com/image2.jpg" \
     "https://example.com/image3.jpg" \
  -o "image" \
  -t 15
```

## ⚡ Performance Features

### Concurrent Downloads
- Support for simultaneous file downloads
- Configurable concurrency levels (default: 10)
- Semaphore-based concurrency control

### Connection Optimization
- Connection pool reuse
- TCP keep-alive connections
- Multiple connections per file
- Configurable connection timeouts

### Buffer Optimization
- Configurable buffer sizes
- Batch writing to reduce I/O operations
- Limited progress bar update frequency

### Memory Management
- Arc and DashMap for reduced memory allocation
- Smart buffer management
- Timely resource cleanup

## 📊 Progress Display

The tool provides two types of progress display:

- **Overall Progress**: Shows completed files vs total files
- **Individual File Progress**: Shows download progress, speed, and ETA for each file

### Progress Bar Example
```
📊 Overall Progress: [████████████████████████████████████████] 3/5 (00:30)
🔄 file1.zip resuming (downloaded: 1024 bytes)
📥 Downloading file2.zip...
✅ file3.zip completed
```

## 🔧 Development

### Dependencies

- `reqwest`: HTTP client (with rustls support)
- `tokio`: Async runtime
- `clap`: Command line argument parsing
- `indicatif`: Progress bar display
- `url`: URL parsing
- `futures`: Async stream processing
- `rayon`: Parallel processing
- `dashmap`: Concurrent HashMap

### Building

```bash
# Development build
cargo build

# Release build (optimized for performance)
cargo build --release

# Run tests
cargo test

# Check code
cargo check
```

## 📈 Performance Benchmarks

Under standard network conditions:

- **Single File Download**: Can achieve 90%+ of network bandwidth
- **Multi-File Concurrent**: 10 concurrent downloads can achieve 80%+ of bandwidth
- **Memory Usage**: ~8KB buffer per file
- **CPU Usage**: Low CPU usage, mostly I/O waiting

## 🐛 Troubleshooting

### Common Issues

1. **Slow Download Speed**
   - Increase concurrency: `-t 20`
   - Increase connections: `-c 5`
   - Increase buffer size: `-b 16384`

2. **Connection Timeouts**
   - Increase timeout: `--timeout 600`
   - Check network connection

3. **High Memory Usage**
   - Reduce concurrency: `-t 5`
   - Reduce buffer size: `-b 4096`

## 📋 Examples

### Download Multiple Files
```bash
./target/release/util \
  -u "https://example.com/file1.zip" \
     "https://example.com/file2.zip" \
     "https://example.com/file3.zip" \
  -o "downloads" \
  -t 15
```

### High-Speed Download
```bash
./target/release/util \
  -u "https://speed.hetzner.de/100MB.bin" \
  -t 20 \
  -c 5 \
  -b 32768 \
  --timeout 1800 \
  -r
```

### Resume Interrupted Download
```bash
# Start download
./target/release/util -u "https://example.com/large_file.zip" -r

# If interrupted, resume with same command
./target/release/util -u "https://example.com/large_file.zip" -r
```

## 🎯 Use Cases

- **Batch Downloads**: Download multiple files simultaneously
- **Large File Downloads**: Efficient handling of large files with resume support
- **High-Speed Downloads**: Optimized for maximum bandwidth utilization
- **Resume Downloads**: Continue interrupted downloads
- **Progress Monitoring**: Real-time progress tracking

## 📄 License

AGPL 3.0 License

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## 📞 Support

If you encounter any issues or have questions, please open an issue on GitHub.

## ⚠️ Legal Disclaimer

### Important Notice
This software is provided for **educational and research purposes only**. By using this software, you acknowledge and agree to the following terms:

### Terms of Use
- This software is designed and distributed for **educational and research purposes only**
- Users are solely responsible for how they use this software
- Users must comply with all applicable laws and regulations
- Users must respect the terms of service of any websites they download from
- Users must not use this software for any illegal activities

### Prohibited Uses
The following uses are strictly prohibited:
- Downloading copyrighted material without permission
- Accessing systems or files without authorization
- Any form of cyber attack or malicious behavior
- Commercial distribution without explicit permission
- Violating intellectual property rights

### Author's Liability
- The author is not responsible for any misuse of this software
- The author is not liable for any damages caused by the use of this software
- The author is not responsible for any legal consequences of software usage
- Any illegal use of this software is the sole responsibility of the user

### Intended Use Cases
This software is designed for legitimate use cases such as:
- Educational projects and learning about network programming
- Academic research on download performance and optimization
- Downloading files you have permission to access
- Testing and development of network applications
- Creating backups of files you own or have permission to backup

**By using this software, you acknowledge that you understand it is for educational purposes only and accept full responsibility for your actions.**

For complete legal terms, see [DISCLAIMER.md](DISCLAIMER.md). 