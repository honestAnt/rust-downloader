**中文**|[english](README_EN.md)
# 高性能多文件下载工具

一个功能强大的Rust多文件下载工具，支持并发下载、断点续传、进度显示和性能优化。

## 🚀 功能特性

- ✅ 支持多个文件同时下载
- ✅ 高性能并发下载（默认10个并发）
- ✅ 实时进度显示（总体进度 + 单个文件进度）
- ✅ 断点续传支持
- ✅ 自动从URL提取文件名
- ✅ 自定义输出文件名
- ✅ 详细的错误处理和统计
- ✅ 支持大文件下载
- ✅ 连接池优化
- ✅ 缓冲区优化
- ✅ TCP保活连接
- ✅ 可配置超时时间

## 📦 安装

### 下载预编译版本

从 [Releases](https://github.com/honestAnt/rust-downloader/releases) 页面下载适用于您平台的预编译版本。

### 从源码构建

确保你的系统已安装Rust和Cargo，然后在项目目录中运行：

```bash
cargo build --release
```

构建后的可执行文件位于 `target/release/downloader` (在Windows上是 `target/release/downloader.exe`)。

## 🎯 使用方法

### 基本用法

```bash
# 下载单个文件
./target/release/downloader -u "https://example.com/file.zip"

# 下载多个文件
./target/release/downloader -u "https://example.com/file1.zip" "https://example.com/file2.zip" "https://example.com/file3.zip"

# 指定输出文件名（单个文件）
./target/release/downloader -u "https://example.com/file.zip" -o "my_file.zip"

# 指定输出文件名（多个文件会自动添加序号）
./target/release/downloader -u "https://example.com/file1.zip" "https://example.com/file2.zip" -o "download"

# 启用断点续传
./target/release/downloader -u "https://example.com/file.zip" -r

# 显示详细信息
./target/release/downloader -u "https://example.com/file.zip" -v
```

### 高性能配置

```bash
# 设置高并发下载（20个并发）
./target/release/util -u "https://example.com/file1.zip" "https://example.com/file2.zip" -t 20

# 设置每个文件的连接数（5个连接）
./target/release/util -u "https://example.com/file.zip" -c 5

# 设置大缓冲区（16KB）
./target/release/util -u "https://example.com/file.zip" -b 16384

# 设置超时时间（10分钟）
./target/release/util -u "https://example.com/file.zip" -T 600
```

### 命令行参数

- `-u, --url`: 下载链接（支持多个URL，用空格分隔）
- `-o, --output`: 输出文件名（可选）
- `-t, --threads`: 并发下载数量（默认10）
- `-c, --connections`: 每个文件的并发连接数（默认3）
- `-v, --verbose`: 显示详细信息
- `-r, --resume`: 启用断点续传
- `-T, --timeout`: 下载超时时间（秒，默认300）
- `-b, --buffer-size`: 缓冲区大小（字节，默认8192）

### 性能优化示例

```bash
# 下载多个大文件，使用高性能配置
./target/release/util \
  -u "https://speed.hetzner.de/100MB.bin" \
     "https://speed.hetzner.de/1GB.bin" \
     "https://speed.hetzner.de/10GB.bin" \
  -t 20 \
  -c 5 \
  -b 32768 \
  -T 1800 \
  -r

# 批量下载图片
./target/release/util \
  -u "https://example.com/image1.jpg" \
     "https://example.com/image2.jpg" \
     "https://example.com/image3.jpg" \
  -o "image" \
  -t 15
```

## ⚡ 性能特性

### 并发下载
- 支持同时下载多个文件
- 可配置并发数量（默认10个）
- 使用信号量控制并发

### 连接优化
- 连接池复用
- TCP保活连接
- 每个文件支持多个连接
- 可配置连接超时

### 缓冲区优化
- 可配置缓冲区大小
- 批量写入减少I/O操作
- 限制进度条更新频率

### 内存管理
- 使用Arc和DashMap减少内存分配
- 智能缓冲区管理
- 及时释放资源

## 📊 进度显示

工具提供两种进度显示：

- **总体进度**: 显示已完成文件数和总文件数
- **单个文件进度**: 显示每个文件的下载进度、速度和剩余时间

### 进度条示例
```
📊 总体进度: [████████████████████████████████████████] 3/5 (00:30)
🔄 file1.zip 续传中 (已下载: 1024 bytes)
📥 下载 file2.zip...
✅ file3.zip 完成
```

## 🔧 开发

### 依赖项

- `reqwest`: HTTP客户端（支持rustls）
- `tokio`: 异步运行时
- `clap`: 命令行参数解析
- `indicatif`: 进度条显示
- `url`: URL解析
- `futures`: 异步流处理
- `rayon`: 并行处理
- `dashmap`: 并发HashMap

### 构建

```bash
# 开发构建
cargo build

# 发布构建（优化性能）
cargo build --release

# 运行测试
cargo test

# 检查代码
cargo check
```

## 📈 性能基准

在标准网络环境下：

- **单文件下载**: 可达到网络带宽的90%+
- **多文件并发**: 10个并发可达到带宽的80%+
- **内存使用**: 每个文件约8KB缓冲区
- **CPU使用**: 低CPU占用，主要时间在I/O等待

## 🐛 故障排除

### 常见问题

1. **下载速度慢**
   - 增加并发数：`-t 20`
   - 增加连接数：`-c 5`
   - 增加缓冲区：`-b 16384`

2. **连接超时**
   - 增加超时时间：`-T 600`
   - 检查网络连接

3. **内存使用高**
   - 减少并发数：`-t 5`
   - 减少缓冲区：`-b 4096`

项目使用约定：
本项目基于 AGPL 3.0 协议开源，使用此项目时请遵守开源协议。
除此外，希望你在使用代码时已经了解以下额外说明：

打包、二次分发 请保留代码出处：https://github.com/honestAnt/rust-downloader
请不要用于商业用途，合法合规使用代码；
如果开源协议变更，将在此 Github 仓库更新，不另行通知。

免责声明



## 📄 许可证

本项目采用MIT许可证 - 查看 [LICENSE](LICENSE) 文件了解更多详情。

## 🏗️ 构建与发布

### 为不同平台构建

请参考 [BUILDING.md](BUILDING.md) 文件了解如何为不同平台构建可执行文件。

### 使用构建脚本

项目提供了构建脚本，可以自动为Windows、Linux和macOS构建可执行文件：

```bash
./build-release.sh
```

生成的可执行文件将位于 `releases/` 目录中。

## ⚠️ 免责声明

### 重要声明
本软件仅供**教育和研究目的**使用。使用本软件即表示您同意以下条款：

### 使用条款
- 本软件仅用于**教育和研究目的**
- 用户对如何使用本软件承担全部责任
- 用户必须遵守所有适用的法律和法规
- 用户必须尊重所下载网站的条款 of service
- 用户不得将本软件用于任何非法活动

### 禁止用途
以下用途严格禁止：
- 未经许可下载受版权保护的材料
- 未经授权访问系统或文件
- 任何形式的网络攻击或恶意行为
- 未经明确许可的商业分发
- 侵犯知识产权

### 作者责任
- 作者不对本软件的误用承担责任
- 作者不对使用本软件造成的任何损害承担责任
- 作者不对软件使用的法律后果承担责任
- 本软件的任何非法使用均由用户承担全部责任

### 预期用途
本软件设计用于合法用途，如：
- 教育项目和学习网络编程
- 下载性能优化的学术研究
- 下载您有权访问的文件
- 网络应用程序的测试和开发
- 创建您拥有或有权备份的文件备份

**使用本软件即表示您理解其仅用于教育目的，并接受对您行为的全部责任。**

完整法律条款请参见 [DISCLAIMER.md](DISCLAIMER.md)。