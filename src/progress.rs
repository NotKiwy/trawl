use std::path::Path;
use std::process::Stdio;

use indicatif::{ProgressBar, ProgressStyle};
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;

use crate::error::TrawlError;
use crate::output;

pub async fn __run(_ytdlp: &Path, _args: &[String], _show_log: bool) -> Result<(), TrawlError> {
    let mut _child = Command::new(_ytdlp)
        .args(_args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    let _stdout = _child.stdout.take().expect("stdout handle");
    let _stderr = _child.stderr.take().expect("stderr handle");

    let _pb = ProgressBar::new(100);
    _pb.set_style(
        ProgressStyle::with_template("  [{bar:50.cyan/237}] {pos:>3}%")
            .unwrap()
            .progress_chars("━━─"),
    );

    let _pb_out = _pb.clone();
    let _stdout_task = tokio::spawn(async move {
        let mut _lines = BufReader::new(_stdout).lines();
        while let Ok(Some(_line)) = _lines.next_line().await {
            if _show_log && !_line.is_empty() {
                _pb_out.println(output::__info_str(__strip_prefix(&_line)));
            }
        }
    });

    let _pb_err = _pb.clone();
    let _stderr_task = tokio::spawn(async move {
        let mut _lines = BufReader::new(_stderr).lines();
        while let Ok(Some(_line)) = _lines.next_line().await {
            if let Some(_pct) = __parse_percent(&_line) {
                _pb_err.set_position(_pct);
            } else if _show_log && __is_stderr_status(&_line) {
                _pb_err.println(output::__info_str(__strip_prefix(&_line)));
            }
        }
    });

    let _status = _child.wait().await?;
    let _ = tokio::join!(_stdout_task, _stderr_task);
    _pb.finish_and_clear();

    if !_status.success() {
        return Err(TrawlError::YtDlpFailed(_status.code().unwrap_or(-1)));
    }

    Ok(())
}

pub async fn __run_quiet(_ytdlp: &Path, _args: &[String]) -> Result<(), TrawlError> {
    let _status = Command::new(_ytdlp)
        .args(_args)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .await?;

    if !_status.success() {
        return Err(TrawlError::YtDlpFailed(_status.code().unwrap_or(-1)));
    }

    Ok(())
}

pub async fn __run_verbose(_ytdlp: &Path, _args: &[String]) -> Result<(), TrawlError> {
    let _status = Command::new(_ytdlp).args(_args).status().await?;

    if !_status.success() {
        return Err(TrawlError::YtDlpFailed(_status.code().unwrap_or(-1)));
    }

    Ok(())
}

fn __parse_percent(_line: &str) -> Option<u64> {
    if !_line.contains("[download]") {
        return None;
    }
    let _pct_token = _line.split_whitespace().find(|_s| _s.ends_with('%'))?;
    _pct_token.trim_end_matches('%').parse::<f64>().ok().map(|_p| _p.clamp(0.0, 100.0) as u64)
}

fn __is_stderr_status(_line: &str) -> bool {
    _line.starts_with("[ExtractAudio]")
        || _line.starts_with("[ffmpeg]")
        || _line.starts_with("[Metadata]")
        || _line.starts_with("[EmbedThumbnail]")
}

fn __strip_prefix(_line: &str) -> &str {
    _line.split("] ").nth(1).unwrap_or(_line)
}
