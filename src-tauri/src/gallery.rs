//! 图集打包：并发取图后顺序写入 ZIP（不依赖外置 zip/ffmpeg）。
use std::io::Write;
use std::path::{Path, PathBuf};

use futures_util::StreamExt;

const CONC: usize = 4;
const MAX_IMAGE: u64 = 40 << 20;

fn client() -> Result<reqwest::Client, String> {
    crate::net::async_builder()
        .timeout(std::time::Duration::from_secs(120))
        .user_agent("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 Chrome/126 Safari/537.36")
        .build()
        .map_err(|e| e.to_string())
}

async fn fetch(c: &reqwest::Client, url: String) -> Result<Vec<u8>, String> {
    let r = c
        .get(&url)
        .header("Referer", "https://www.douyin.com/")
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if !r.status().is_success() {
        return Err(format!("HTTP {}", r.status()));
    }
    if r.content_length().unwrap_or(0) > MAX_IMAGE {
        return Err("单图超过 40MB".into());
    }
    r.bytes()
        .await
        .map(|b| b.to_vec())
        .map_err(|e| e.to_string())
}

/// 下载 `urls` 到 `<dir>/<name>.zip`，返回 zip 路径。
pub async fn zip(urls: Vec<String>, dir: String, name: String) -> Result<String, String> {
    if urls.is_empty() {
        return Err("没有可打包的图片".into());
    }
    let started = std::time::Instant::now();
    crate::logging::info("gallery", format!("开始打包 {name} {} 张", urls.len()));
    let dir = crate::paths::expand(&dir);
    tokio::fs::create_dir_all(&dir)
        .await
        .map_err(|e| e.to_string())?;
    let out = dir.join(format!("{name}.zip"));
    let part = PathBuf::from(format!("{}.part", out.to_string_lossy()));

    let c = client()?;
    let total = urls.len();
    let mut got: Vec<(usize, Vec<u8>)> = Vec::with_capacity(total);
    let mut stream = futures_util::stream::iter(urls.into_iter().enumerate().map(|(i, u)| {
        let c = c.clone();
        async move { (i, fetch(&c, u).await) }
    }))
    .buffer_unordered(CONC);
    while let Some((i, r)) = stream.next().await {
        match r {
            Ok(b) => {
                crate::logging::info("gallery", format!("{name} 第{}张 {}B", i + 1, b.len()));
                got.push((i, b));
            }
            Err(e) => {
                crate::logging::error("gallery", format!("{name} 第{}张失败：{e}", i + 1));
                return Err(format!("第 {} 张取图失败：{e}", i + 1));
            }
        }
    }
    got.sort_by_key(|(i, _)| *i);

    let path = part.clone();
    let zipped =
        tokio::task::spawn_blocking(move || -> Result<(), String> { write_zip(&path, &got) })
            .await
            .map_err(|e| e.to_string())?;
    zipped?;
    tokio::fs::rename(&part, &out)
        .await
        .map_err(|e| e.to_string())?;
    crate::logging::info(
        "gallery",
        format!(
            "完成 {name} {} 张 {}ms",
            total,
            started.elapsed().as_millis()
        ),
    );
    Ok(out.to_string_lossy().to_string())
}

fn write_zip(path: &Path, files: &[(usize, Vec<u8>)]) -> Result<(), String> {
    let f = std::fs::File::create(path).map_err(|e| e.to_string())?;
    let mut zw = zip::ZipWriter::new(f);
    let opts: zip::write::SimpleFileOptions =
        zip::write::SimpleFileOptions::default().compression_method(zip::CompressionMethod::Stored);
    for (i, b) in files {
        zw.start_file(format!("{:03}.jpg", i + 1), opts)
            .map_err(|e| e.to_string())?;
        zw.write_all(b).map_err(|e| e.to_string())?;
    }
    zw.finish().map_err(|e| e.to_string())?;
    Ok(())
}
