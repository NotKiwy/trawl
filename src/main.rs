use clap::Parser;
use trawl::{args::Args, downloader, output, platform, search, spotify, ytdlp};

#[tokio::main]
async fn main() {
    let _args = Args::parse();

    if !_args.quiet {
        output::__logo();
    }

    let _ytdlp_path = match ytdlp::__ensure().await {
        Ok(_p) => _p,
        Err(_e) => {
            output::__err(&_e.to_string());
            std::process::exit(1);
        }
    };

    let _version = match ytdlp::__version(&_ytdlp_path).await {
        Ok(_v) => _v,
        Err(_e) => {
            output::__err(&_e.to_string());
            std::process::exit(1);
        }
    };

    if !_args.quiet {
        output::__info(&format!("using yt-dlp {}", _version));
    }

    if let Some(ref _q) = _args.search {
        __handle_search(&_args, &_ytdlp_path, _q).await;
        return;
    }

    let _url = _args.url.clone().unwrap_or_default();
    let _platform = platform::__detect(&_url);

    if !_args.quiet {
        output::__info(&format!("platform {}", platform::__name(&_platform)));
    }

    if _platform == platform::Platform::Spotify {
        __handle_spotify(&_args, &_ytdlp_path, &_url).await;
        return;
    }

    if _args.dry_run {
        let mut _cmd_args = downloader::__build_args(&_args);
        _cmd_args.push(_url.clone());
        output::__cmd(&format!(
            "{} {}",
            _ytdlp_path.display(),
            _cmd_args.join(" ")
        ));
        output::__warn("dry run — nothing downloaded");
        return;
    }

    if !_args.quiet {
        output::__action(&format!("downloading {}", _url));
    }

    match downloader::__run(&_ytdlp_path, &_args, &_url).await {
        Ok(_) => {
            if !_args.quiet {
                output::__ok("done");
            }
        }
        Err(_e) => {
            output::__err(&_e.to_string());
            std::process::exit(1);
        }
    }
}

async fn __handle_search(_args: &Args, _ytdlp_path: &std::path::Path, _query: &str) {
    if !_args.quiet {
        output::__info(&format!("searching YouTube for {}", _query));
    }

    let _results = match search::__fetch(_ytdlp_path, _query, 10).await {
        Ok(_r) => _r,
        Err(_e) => {
            output::__err(&_e.to_string());
            std::process::exit(1);
        }
    };

    search::__print_results(&_results);

    let _selection = match search::__prompt_selection(_results.len()) {
        Ok(_s) => _s,
        Err(_e) => {
            output::__err(&_e.to_string());
            std::process::exit(1);
        }
    };

    let _indices = match _selection {
        search::Selection::Abort => {
            output::__warn("cancelled");
            return;
        }
        search::Selection::Indices(_i) => _i,
    };

    let _total = _indices.len();
    for (_pos, _idx) in _indices.iter().enumerate() {
        let _result = &_results[*_idx];
        let _url = format!("https://www.youtube.com/watch?v={}", _result.id);

        if _total > 1 && !_args.quiet {
            output::__action(&format!("[{}/{}] {}", _pos + 1, _total, _result.title));
        }

        if _args.dry_run {
            let mut _cmd_args = downloader::__build_args(_args);
            _cmd_args.push(_url.clone());
            output::__cmd(&format!(
                "{} {}",
                _ytdlp_path.display(),
                _cmd_args.join(" ")
            ));
            continue;
        }

        if _total == 1 && !_args.quiet {
            output::__action(&format!("downloading {}", _result.title));
        }

        if let Err(_e) = downloader::__run(_ytdlp_path, _args, &_url).await {
            if _total > 1 {
                output::__warn(&format!("skipped: {}", _e));
            } else {
                output::__err(&_e.to_string());
                std::process::exit(1);
            }
        }
    }

    if _args.dry_run {
        output::__warn("dry run — nothing downloaded");
        return;
    }

    if !_args.quiet {
        let _msg = if _total > 1 {
            format!("done {} tracks", _total)
        } else {
            "done".to_string()
        };
        output::__ok(&_msg);
    }
}

async fn __handle_spotify(_args: &Args, _ytdlp_path: &std::path::Path, _url: &str) {
    if !_args.quiet {
        output::__action("resolving Spotify metadata...");
    }

    let _resource = match spotify::__resolve(_url).await {
        Ok(_r) => _r,
        Err(_e) => {
            output::__err(&_e.to_string());
            std::process::exit(1);
        }
    };

    match _resource {
        spotify::SpotifyResource::Track { query: _query } => {
            if !_args.quiet {
                output::__info(&format!(
                    "search {}",
                    _query.trim_start_matches("ytsearch1:")
                ));
            }
            if _args.dry_run {
                output::__cmd(&format!("{} {}", _ytdlp_path.display(), _query));
                output::__warn("dry run — nothing downloaded");
                return;
            }
            match downloader::__run(_ytdlp_path, _args, &_query).await {
                Ok(_) => {
                    if !_args.quiet {
                        output::__ok("done");
                    }
                }
                Err(_e) => {
                    output::__err(&_e.to_string());
                    std::process::exit(1);
                }
            }
        }
        spotify::SpotifyResource::Multi {
            name: _name,
            queries: _queries,
        } => {
            if !_args.quiet {
                output::__info(&format!("collection {} ({} tracks)", _name, _queries.len()));
            }
            if _args.dry_run {
                for _q in &_queries {
                    output::__cmd(&format!("{} {}", _ytdlp_path.display(), _q));
                }
                output::__warn("dry run — nothing downloaded");
                return;
            }
            let _total = _queries.len();
            for (_idx, _query) in _queries.iter().enumerate() {
                if !_args.quiet {
                    output::__action(&format!(
                        "[{}/{}] {}",
                        _idx + 1,
                        _total,
                        _query.trim_start_matches("ytsearch1:")
                    ));
                }
                if let Err(_e) = downloader::__run(_ytdlp_path, _args, _query).await {
                    output::__warn(&format!("skipped: {}", _e));
                }
            }
            if !_args.quiet {
                output::__ok(&format!("done — {}/{} tracks", _total, _total));
            }
        }
    }
}
