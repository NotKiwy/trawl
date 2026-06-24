use std::path::Path;

use crate::args::Args;
use crate::error::TrawlError;
use crate::progress;

pub fn __build_args(_args: &Args) -> Vec<String> {
    let mut _cmd: Vec<String> = Vec::new();

    let _tmpl = match &_args.filename {
        Some(_t) => format!("{}/{}", _args.output.trim_end_matches('/'), _t),
        None => format!("{}/%(title)s.%(ext)s", _args.output.trim_end_matches('/')),
    };
    _cmd.push("-o".into());
    _cmd.push(_tmpl);

    _cmd.push("--retries".into());
    _cmd.push(_args.retries.to_string());

    if _args.no_playlist {
        _cmd.push("--no-playlist".into());
    }

    if let Some(ref _items) = _args.playlist_items {
        _cmd.push("--playlist-items".into());
        _cmd.push(_items.clone());
    } else {
        if let Some(_start) = _args.playlist_start {
            _cmd.push("--playlist-start".into());
            _cmd.push(_start.to_string());
        }
        if let Some(_end) = _args.playlist_end {
            _cmd.push("--playlist-end".into());
            _cmd.push(_end.to_string());
        }
    }

    if let Some(ref _cookie_file) = _args.cookies {
        _cmd.push("--cookies".into());
        _cmd.push(_cookie_file.clone());
    }

    if let Some(ref _proxy_url) = _args.proxy {
        _cmd.push("--proxy".into());
        _cmd.push(_proxy_url.clone());
    }

    if let Some(_n) = _args.concurrent {
        _cmd.push("--concurrent-fragments".into());
        _cmd.push(_n.to_string());
    }

    if _args.list_formats {
        _cmd.push("--list-formats".into());
    } else if _args.audio_only {
        _cmd.push("-x".into());
        _cmd.push("--audio-format".into());
        _cmd.push(_args.audio_format.clone());
        if _args.audio_quality != "best" {
            _cmd.push("--audio-quality".into());
            _cmd.push(_args.audio_quality.clone());
        }
        if _args.embed_thumbnail {
            _cmd.push("--embed-thumbnail".into());
        }
        if _args.embed_metadata {
            _cmd.push("--embed-metadata".into());
        }
    } else {
        _cmd.push("-f".into());
        _cmd.push(__format_selector(&_args.video_quality, &_args.video_format));
        _cmd.push("--merge-output-format".into());
        _cmd.push(_args.video_format.clone());
        if _args.embed_thumbnail {
            _cmd.push("--embed-thumbnail".into());
        }
        if _args.embed_metadata {
            _cmd.push("--embed-metadata".into());
        }
    }

    if _args.subs {
        _cmd.push("--write-subs".into());
        _cmd.push("--sub-langs".into());
        _cmd.push(_args.subs_lang.clone());
        if _args.embed_subs {
            _cmd.push("--embed-subs".into());
        }
    }

    if _args.verbose {
        _cmd.push("--verbose".into());
    } else if !_args.quiet {
        _cmd.push("--newline".into());
        _cmd.push("--progress".into());
    }

    _cmd
}

fn __format_selector(_quality: &str, _format: &str) -> String {
    match (_quality, _format) {
        ("best", "mp4") => {
            "bestvideo[ext=mp4][vcodec^=avc1]+bestaudio[ext=m4a]/bestvideo[ext=mp4]+bestaudio[ext=m4a]/best[ext=mp4]/best".into()
        }
        ("worst", "mp4") => {
            "worstvideo[ext=mp4]+worstaudio[ext=m4a]/worst[ext=mp4]/worst".into()
        }
        ("best", _) => "bestvideo+bestaudio/best".into(),
        ("worst", _) => "worstvideo+worstaudio/worst".into(),
        (_, "mp4") => {
            let _h = _quality.trim_end_matches('p');
            format!(
                "bestvideo[height<={}][ext=mp4][vcodec^=avc1]+bestaudio[ext=m4a]/bestvideo[height<={}][ext=mp4]+bestaudio[ext=m4a]/best[height<={}]",
                _h, _h, _h
            )
        }
        _ => {
            let _h = _quality.trim_end_matches('p');
            format!("bestvideo[height<={}]+bestaudio/best[height<={}]", _h, _h)
        }
    }
}

pub async fn __run(_ytdlp: &Path, _args: &Args, _url: &str) -> Result<(), TrawlError> {
    let mut _cmd_args = __build_args(_args);
    _cmd_args.push(_url.to_string());

    if _args.quiet {
        progress::__run_quiet(_ytdlp, &_cmd_args).await
    } else if _args.verbose {
        progress::__run_verbose(_ytdlp, &_cmd_args).await
    } else {
        progress::__run(_ytdlp, &_cmd_args, _args.log).await
    }
}
