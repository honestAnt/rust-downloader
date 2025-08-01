use clap::Parser;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::io::SeekFrom;
use std::path::Path;
use std::time::Duration;
use tokio::io::{AsyncWriteExt, AsyncSeekExt};
use url::Url;
use futures::StreamExt;
use std::sync::Arc;
use tokio::sync::Semaphore;
use dashmap::DashMap;

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// 下载链接（支持多个URL，用空格分隔）
    #[arg(short, long, num_args = 1.., value_delimiter = ' ')]
    url: Vec<String>,

    /// 输出文件名（可选，默认从URL提取）
    #[arg(short, long)]
    output: Option<String>,

    /// 并发下载数量
    #[arg(short, long, default_value = "10")]
    threads: usize,

    /// 每个文件的并发连接数
    #[arg(short, long, default_value = "3")]
    connections: usize,

    /// 是否显示详细信息
    #[arg(short, long)]
    verbose: bool,

    /// 是否支持断点续传
    #[arg(short, long)]
    resume: bool,

    /// 下载超时时间（秒）
    #[arg(long, default_value = "300")]
    timeout: u64,

    /// 缓冲区大小（字节）
    #[arg(short, long, default_value = "8192")]
    buffer_size: usize,
}

#[derive(Debug, Clone)]
struct DownloadTask {
    url: String,
    filename: String,
    size: Option<u64>,
    downloaded: u64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    if args.url.is_empty() {
        eprintln!("错误: 请提供至少一个下载链接");
        std::process::exit(1);
    }

    println!("🚀 高性能多文件下载工具");
    println!("📥 待下载文件数: {}", args.url.len());
    println!("⚡ 并发下载数: {}", args.threads);
    println!("🔗 每文件连接数: {}", args.connections);

    // 创建多进度条管理器
    let multi_progress = Arc::new(MultiProgress::new());
    let overall_progress = multi_progress.add(ProgressBar::new(args.url.len() as u64));
    overall_progress.set_style(
        ProgressStyle::default_bar()
            .template("📊 总体进度: [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
            .unwrap()
            .progress_chars("#>-")
    );

    // 创建下载任务
    let mut tasks = Vec::new();
    for (i, url_str) in args.url.iter().enumerate() {
        let url = Url::parse(url_str)?;
        let filename = if let Some(ref output) = args.output {
            if args.url.len() == 1 {
                output.clone()
            } else {
                format!("{}_{}", output, i + 1)
            }
        } else {
            url.path_segments()
                .and_then(|segments| segments.last())
                .unwrap_or(&format!("file_{}", i + 1))
                .to_string()
        };

        tasks.push(DownloadTask {
            url: url_str.clone(),
            filename,
            size: None,
            downloaded: 0,
        });
    }

    // 创建并发限制器
    let semaphore = Arc::new(Semaphore::new(args.threads));
    let progress_bars = Arc::new(DashMap::new());

    // 并发下载所有文件
    let mut download_futures = Vec::new();
    
    for task in tasks {
        let semaphore = semaphore.clone();
        let progress_bars = progress_bars.clone();
        let overall_progress = overall_progress.clone();
        let args_clone = args.clone();
        let multi_progress = multi_progress.clone();
        
        let future = async move {
            let _permit = semaphore.acquire().await.unwrap();
            
            // 为每个文件创建进度条
            let file_progress = multi_progress.add(ProgressBar::new_spinner());
            file_progress.set_style(
                ProgressStyle::default_spinner()
                    .template("{spinner:.green} {msg}")
                    .unwrap()
            );
            progress_bars.insert(task.filename.clone(), file_progress.clone());
            
            let result = download_file_optimized(&task, &file_progress, &args_clone).await;
            
            if result.is_ok() {
                overall_progress.inc(1);
            }
            
            file_progress.finish_with_message(format!("✅ {} 下载完成", task.filename));
            result
        };
        
        download_futures.push(future);
    }

    // 等待所有下载完成
    let results = futures::future::join_all(download_futures).await;
    
    // 统计结果
    let mut success_count = 0;
    let mut error_count = 0;
    
    for result in results {
        match result {
            Ok(_) => success_count += 1,
            Err(e) => {
                error_count += 1;
                eprintln!("❌ 下载失败: {}", e);
            }
        }
    }

    overall_progress.finish_with_message("🎉 所有下载完成！");
    
    println!("\n📈 下载统计:");
    println!("✅ 成功: {}", success_count);
    println!("❌ 失败: {}", error_count);
    println!("📊 成功率: {:.1}%", (success_count as f64 / (success_count + error_count) as f64) * 100.0);

    if error_count > 0 {
        std::process::exit(1);
    }

    Ok(())
}

async fn download_file_optimized(
    task: &DownloadTask,
    progress_bar: &ProgressBar,
    args: &Args,
) -> Result<(), Box<dyn std::error::Error>> {
    // 创建优化的HTTP客户端
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(args.timeout))
        .pool_max_idle_per_host(args.connections)
        .pool_idle_timeout(Duration::from_secs(30))
        .tcp_keepalive(Some(Duration::from_secs(60)))
        .build()?;

    progress_bar.set_message(format!("🔍 获取 {} 信息...", task.filename));

    // 获取文件信息
    let response = client.head(&task.url).send().await?;
    
    if !response.status().is_success() && !response.status().is_redirection() {
        return Err(format!("HTTP错误 {}: {}", response.status(), task.url).into());
    }

    let content_length = response
        .headers()
        .get("content-length")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.parse::<u64>().ok());

    let supports_range = response
        .headers()
        .get("accept-ranges")
        .map(|v| v == "bytes")
        .unwrap_or(false);

    // 检查断点续传
    let mut start_byte = 0u64;
    if args.resume && supports_range && Path::new(&task.filename).exists() {
        if let Ok(metadata) = std::fs::metadata(&task.filename) {
            start_byte = metadata.len();
            if start_byte > 0 {
                progress_bar.set_message(format!("🔄 {} 续传中 (已下载: {} bytes)", task.filename, start_byte));
            }
        }
    }

    // 创建输出文件
    let file = if start_byte > 0 {
        tokio::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .open(&task.filename)
            .await?
    } else {
        tokio::fs::File::create(&task.filename).await?
    };

    let mut file = tokio::io::BufWriter::with_capacity(args.buffer_size, file);

    if start_byte > 0 {
        file.seek(SeekFrom::Start(start_byte)).await?;
    }

    // 开始下载
    progress_bar.set_message(format!("📥 下载 {}...", task.filename));
    
    let mut request = client.get(&task.url);
    if start_byte > 0 && supports_range {
        request = request.header("Range", format!("bytes={}-", start_byte));
    }
    
    let response = request.send().await?;
    let mut downloaded: u64 = start_byte;
    let mut stream = response.bytes_stream();

    // 创建下载进度条
    let download_bar = if let Some(total_size) = content_length {
        let adjusted_total = if start_byte > 0 { total_size - start_byte } else { total_size };
        ProgressBar::new(adjusted_total)
    } else {
        ProgressBar::new_spinner()
    };

    download_bar.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} {msg} [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
            .unwrap()
            .progress_chars("#>-")
    );

    download_bar.set_message(format!("{}", task.filename));

    // 高性能下载循环
    let mut buffer = Vec::with_capacity(args.buffer_size);
    let mut last_update = std::time::Instant::now();
    
    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        buffer.extend_from_slice(&chunk);
        
        downloaded += chunk.len() as u64;
        
        // 批量写入以提高性能
        if buffer.len() >= args.buffer_size {
            file.write_all(&buffer).await?;
            buffer.clear();
        }
        
        // 更新进度条（限制更新频率以提高性能）
        if last_update.elapsed() > Duration::from_millis(100) {
            download_bar.set_position(downloaded - start_byte);
            last_update = std::time::Instant::now();
        }
    }

    // 写入剩余数据
    if !buffer.is_empty() {
        file.write_all(&buffer).await?;
    }

    download_bar.finish_with_message(format!("✅ {} 完成", task.filename));
    file.flush().await?;

    Ok(())
}
