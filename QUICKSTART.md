# Quick Start Guide

Get started with the High-Performance Multi-File Downloader in minutes!

## 🚀 Quick Installation

```bash
# Clone or download the project
cd util

# Build the release version
cargo build --release

# Test the installation
./target/release/util --help
```

## ⚡ Quick Examples

### Download a Single File
```bash
./target/release/util -u "https://example.com/file.zip"
```

### Download Multiple Files
```bash
./target/release/util -u "https://example.com/file1.zip" "https://example.com/file2.zip"
```

### High-Speed Download
```bash
./target/release/util -u "https://example.com/large_file.zip" -t 20 -c 5 -r
```

## 🎯 Common Use Cases

### 1. Download Multiple Images
```bash
./target/release/util \
  -u "https://example.com/image1.jpg" \
     "https://example.com/image2.jpg" \
     "https://example.com/image3.jpg" \
  -o "images" \
  -t 15
```

### 2. Download Large Files with Resume
```bash
./target/release/util \
  -u "https://example.com/large_file.zip" \
  -r \
  -t 10 \
  -b 32768
```

### 3. Batch Download with Custom Names
```bash
./target/release/util \
  -u "https://example.com/file1.pdf" \
     "https://example.com/file2.pdf" \
     "https://example.com/file3.pdf" \
  -o "document" \
  -t 20
```

## 📊 Performance Tips

### For Fast Downloads
- Use `-t 20` for high concurrency
- Use `-c 5` for multiple connections per file
- Use `-b 16384` for larger buffers

### For Large Files
- Always use `-r` for resume support
- Use `--timeout 1800` for longer timeouts
- Use `-b 32768` for larger buffers

### For Many Small Files
- Use `-t 30` for high concurrency
- Use `-b 8192` for standard buffers
- Use `-c 3` for standard connections

## 🔧 Troubleshooting

### Slow Downloads
```bash
# Increase concurrency and connections
./target/release/util -u "https://example.com/file.zip" -t 20 -c 5
```

### Connection Timeouts
```bash
# Increase timeout
./target/release/util -u "https://example.com/file.zip" --timeout 600
```

### Resume Interrupted Downloads
```bash
# Use the same command with -r flag
./target/release/util -u "https://example.com/file.zip" -r
```

## 📈 Progress Monitoring

The tool shows:
- Overall progress bar
- Individual file progress
- Download speed and ETA
- Success/failure statistics

## 🎉 You're Ready!

You now have a powerful, high-performance downloader at your fingertips. Enjoy fast, reliable downloads with resume support and real-time progress tracking! 