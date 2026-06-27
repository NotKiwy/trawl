use std::io::Write;
use std::path::Path;

use serde::Deserialize;

use crate::error::TrawlError;

#[derive(Deserialize)]
pub struct SearchResult {
    pub id: String,
    pub title: String,
    pub uploader: Option<String>,
    pub duration_string: Option<String>,
    pub view_count: Option<u64>,
}

pub async fn __fetch(_ytdlp: &Path, _query: &str, _n: u8) -> Result<Vec<SearchResult>, TrawlError> {
    let _search_url = format!("ytsearch{}:{}", _n, _query);

    let _out = tokio::process::Command::new(_ytdlp)
        .args([
            &_search_url,
            "--dump-json",
            "--no-playlist",
            "--flat-playlist",
        ])
        .output()
        .await?;

    let _stdout = String::from_utf8_lossy(&_out.stdout);

    let _results: Vec<SearchResult> = _stdout
        .lines()
        .filter(|_l| !_l.is_empty())
        .filter_map(|_l| serde_json::from_str(_l).ok())
        .collect();

    if _results.is_empty() {
        return Err(TrawlError::SearchNoResults(_query.to_string()));
    }

    Ok(_results)
}

pub fn __print_results(_results: &[SearchResult]) {
    use colored::Colorize;

    println!();
    for (_i, _r) in _results.iter().enumerate() {
        let _n = format!("[ {:02} ]", _i + 1);
        let _uploader = _r.uploader.as_deref().unwrap_or("unknown");
        let _duration = _r.duration_string.as_deref().unwrap_or("?:??");
        let _views = _r
            .view_count
            .map(__fmt_views)
            .unwrap_or_else(|| "—".to_string());

        println!(
            "{} {}  {}  {}  {}",
            _n.bright_black(),
            _r.title.white().bold(),
            _uploader.bright_black(),
            _duration.cyan(),
            _views.bright_black(),
        );
    }
    println!();
}

pub enum Selection {
    Indices(Vec<usize>),
    Abort,
}

pub fn __prompt_selection(_count: usize) -> Result<Selection, TrawlError> {
    use colored::Colorize;

    print!(
        "{} enter number(s), range, or {} to cancel -> ",
        "[?]".cyan().bold(),
        "q".bright_black(),
    );
    __flush();

    let mut _input = String::new();
    std::io::stdin()
        .read_line(&mut _input)
        .map_err(TrawlError::IoError)?;

    let _trimmed = _input.trim();

    if _trimmed.eq_ignore_ascii_case("q") || _trimmed.is_empty() {
        return Ok(Selection::Abort);
    }

    let mut _indices: Vec<usize> = Vec::new();

    for _token in _trimmed.split(',') {
        let _token = _token.trim();
        if _token.contains('-') {
            let _parts: Vec<&str> = _token.splitn(2, '-').collect();
            if _parts.len() == 2 {
                let _a = _parts[0].trim().parse::<usize>().ok();
                let _b = _parts[1].trim().parse::<usize>().ok();
                if let (Some(_a), Some(_b)) = (_a, _b) {
                    for _n in _a..=_b {
                        if _n >= 1 && _n <= _count {
                            _indices.push(_n - 1);
                        }
                    }
                    continue;
                }
            }
        }
        if let Ok(_n) = _token.parse::<usize>() {
            if _n >= 1 && _n <= _count {
                _indices.push(_n - 1);
            }
        }
    }

    _indices.dedup();

    if _indices.is_empty() {
        return Ok(Selection::Abort);
    }

    Ok(Selection::Indices(_indices))
}

fn __fmt_views(_n: u64) -> String {
    if _n >= 1_000_000_000 {
        format!("{:.1}B views", _n as f64 / 1_000_000_000.0)
    } else if _n >= 1_000_000 {
        format!("{:.1}M views", _n as f64 / 1_000_000.0)
    } else if _n >= 1_000 {
        format!("{:.1}K views", _n as f64 / 1_000.0)
    } else {
        format!("{} views", _n)
    }
}

fn __flush() {
    let _ = std::io::stdout().flush();
}
