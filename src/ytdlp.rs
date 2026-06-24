use std::path::{Path, PathBuf};

use indicatif::{ProgressBar, ProgressStyle};
use reqwest::Client;
use tokio::io::AsyncWriteExt;

use crate::error::TrawlError;
use crate::output;

pub async fn __ensure() -> Result<PathBuf, TrawlError> {
    if let Some(_p) = __in_path() {
        return Ok(_p);
    }

    let _local = __local_path();
    if _local.exists() {
        return Ok(_local);
    }

    output::__warn("yt-dlp not found");
    output::__action("download yt-dlp automatically? [y/N] ");
    __flush_stdout();

    let mut _input = String::new();
    std::io::stdin().read_line(&mut _input).map_err(TrawlError::IoError)?;

    if _input.trim().to_lowercase() != "y" {
        return Err(TrawlError::YtDlpNotFound);
    }

    __download(&_local).await?;
    Ok(_local)
}

pub async fn __version(_path: &Path) -> Result<String, TrawlError> {
    let _out = tokio::process::Command::new(_path)
        .arg("--version")
        .output()
        .await
        .map_err(|_| TrawlError::YtDlpNotFound)?;

    Ok(String::from_utf8_lossy(&_out.stdout).trim().to_string())
}

fn __in_path() -> Option<PathBuf> {
    std::env::var_os("PATH").and_then(|_paths| {
        std::env::split_paths(&_paths).find_map(|_dir| {
            let _candidate = _dir.join(__binary_name());
            _candidate.exists().then_some(_candidate)
        })
    })
}

fn __local_path() -> PathBuf {
    __trawl_dir().join(__binary_name())
}

fn __trawl_dir() -> PathBuf {
    #[cfg(windows)]
    let _base = std::env::var("APPDATA")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("C:\\trawl"));

    #[cfg(not(windows))]
    let _base = std::env::var("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("/tmp"));

    _base.join(".trawl")
}

fn __binary_name() -> &'static str {
    if cfg!(windows) { "yt-dlp.exe" } else { "yt-dlp" }
}

fn __release_url() -> &'static str {
    match (std::env::consts::OS, std::env::consts::ARCH) {
        ("macos", _) => {
            "https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp_macos"
        }
        ("linux", "aarch64") => {
            "https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp_linux_aarch64"
        }
        ("linux", _) => {
            "https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp_linux"
        }
        ("windows", _) => {
            "https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp.exe"
        }
        _ => "https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp",
    }
}

async fn __download(_dest: &Path) -> Result<(), TrawlError> {
    if let Some(_parent) = _dest.parent() {
        tokio::fs::create_dir_all(_parent).await?;
    }

    output::__action(&format!("downloading yt-dlp → {}", _dest.display()));

    let _client = Client::builder()
        .user_agent("trawl")
        .build()?;

    let _response = _client
        .get(__release_url())
        .send()
        .await?;

    if !_response.status().is_success() {
        return Err(TrawlError::YtDlpDownloadFailed(format!(
            "HTTP {}",
            _response.status()
        )));
    }

    let _total = _response.content_length().unwrap_or(0);
    let _pb = ProgressBar::new(_total);
    _pb.set_style(
        ProgressStyle::with_template(
            "{spinner:.cyan} [{bar:38.cyan/237}] {bytes}/{total_bytes} {binary_bytes_per_sec}",
        )
        .unwrap()
        .progress_chars("━╸─"),
    );

    let _tmp = _dest.with_extension("tmp");
    let mut _file = tokio::fs::File::create(&_tmp).await?;
    let mut _resp = _response;

    while let Some(_chunk) = _resp.chunk().await? {
        _file.write_all(&_chunk).await?;
        _pb.inc(_chunk.len() as u64);
    }

    _file.flush().await?;
    drop(_file);

    tokio::fs::rename(&_tmp, _dest).await?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut _perms = tokio::fs::metadata(_dest).await?.permissions();
        _perms.set_mode(0o755);
        tokio::fs::set_permissions(_dest, _perms).await?;
    }

    _pb.finish_and_clear();
    output::__ok("yt-dlp downloaded");

    Ok(())
}

fn __flush_stdout() {
    use std::io::Write;
    let _ = std::io::stdout().flush();
}
