use trawl::{args::Args, downloader::__build_args};

fn __base(_url: &str) -> Args {
    Args {
        url: _url.to_string(),
        audio_only: false,
        output: ".".to_string(),
        filename: None,
        video_quality: "best".to_string(),
        video_format: "mp4".to_string(),
        audio_format: "opus".to_string(),
        audio_quality: "best".to_string(),
        no_playlist: false,
        playlist_start: None,
        playlist_end: None,
        playlist_items: None,
        subs: false,
        subs_lang: "en".to_string(),
        embed_subs: false,
        embed_thumbnail: false,
        embed_metadata: false,
        concurrent: None,
        retries: 3,
        cookies: None,
        proxy: None,
        list_formats: false,
        dry_run: false,
        quiet: false,
        log: false,
        verbose: false,
    }
}

#[test]
fn __test_default_format_selector() {
    let _args = __base("https://youtube.com/watch?v=x");
    let _cmd = __build_args(&_args);
    let _idx = _cmd.iter().position(|_s| _s == "-f").unwrap();
    assert!(_cmd[_idx + 1].contains("bestvideo") && _cmd[_idx + 1].contains("ext=mp4"));
}

#[test]
fn __test_quality_1080p() {
    let mut _args = __base("https://youtube.com/watch?v=x");
    _args.video_quality = "1080p".to_string();
    let _cmd = __build_args(&_args);
    let _idx = _cmd.iter().position(|_s| _s == "-f").unwrap();
    assert!(_cmd[_idx + 1].contains("1080"));
}

#[test]
fn __test_worst_quality() {
    let mut _args = __base("https://youtube.com/watch?v=x");
    _args.video_quality = "worst".to_string();
    let _cmd = __build_args(&_args);
    let _idx = _cmd.iter().position(|_s| _s == "-f").unwrap();
    assert!(_cmd[_idx + 1].contains("worstvideo") && _cmd[_idx + 1].contains("ext=mp4"));
}

#[test]
fn __test_audio_only() {
    let mut _args = __base("https://youtube.com/watch?v=x");
    _args.audio_only = true;
    _args.audio_format = "mp3".to_string();
    let _cmd = __build_args(&_args);
    assert!(_cmd.contains(&"-x".to_string()));
    assert!(_cmd.contains(&"--audio-format".to_string()));
    assert!(_cmd.contains(&"mp3".to_string()));
}

#[test]
fn __test_no_playlist() {
    let mut _args = __base("https://youtube.com/watch?v=x");
    _args.no_playlist = true;
    let _cmd = __build_args(&_args);
    assert!(_cmd.contains(&"--no-playlist".to_string()));
}

#[test]
fn __test_list_formats() {
    let mut _args = __base("https://youtube.com/watch?v=x");
    _args.list_formats = true;
    let _cmd = __build_args(&_args);
    assert!(_cmd.contains(&"--list-formats".to_string()));
}

#[test]
fn __test_output_template() {
    let mut _args = __base("https://youtube.com/watch?v=x");
    _args.output = "/tmp/downloads".to_string();
    let _cmd = __build_args(&_args);
    let _idx = _cmd.iter().position(|_s| _s == "-o").unwrap();
    assert!(_cmd[_idx + 1].starts_with("/tmp/downloads"));
}

#[test]
fn __test_custom_filename_template() {
    let mut _args = __base("https://youtube.com/watch?v=x");
    _args.filename = Some("%(uploader)s - %(title)s.%(ext)s".to_string());
    let _cmd = __build_args(&_args);
    let _idx = _cmd.iter().position(|_s| _s == "-o").unwrap();
    assert!(_cmd[_idx + 1].contains("%(uploader)s"));
}

#[test]
fn __test_verbose_flag() {
    let mut _args = __base("https://youtube.com/watch?v=x");
    _args.verbose = true;
    let _cmd = __build_args(&_args);
    assert!(_cmd.contains(&"--verbose".to_string()));
}

#[test]
fn __test_playlist_start_end() {
    let mut _args = __base("https://youtube.com/playlist?list=x");
    _args.playlist_start = Some(3);
    _args.playlist_end = Some(10);
    let _cmd = __build_args(&_args);
    assert!(_cmd.contains(&"--playlist-start".to_string()));
    assert!(_cmd.contains(&"3".to_string()));
    assert!(_cmd.contains(&"--playlist-end".to_string()));
    assert!(_cmd.contains(&"10".to_string()));
}

#[test]
fn __test_playlist_items() {
    let mut _args = __base("https://youtube.com/playlist?list=x");
    _args.playlist_items = Some("1,3,5-10".to_string());
    let _cmd = __build_args(&_args);
    assert!(_cmd.contains(&"--playlist-items".to_string()));
    assert!(_cmd.contains(&"1,3,5-10".to_string()));
    assert!(!_cmd.contains(&"--playlist-start".to_string()));
}

#[test]
fn __test_subs() {
    let mut _args = __base("https://youtube.com/watch?v=x");
    _args.subs = true;
    _args.subs_lang = "ru".to_string();
    let _cmd = __build_args(&_args);
    assert!(_cmd.contains(&"--write-subs".to_string()));
    assert!(_cmd.contains(&"--sub-langs".to_string()));
    assert!(_cmd.contains(&"ru".to_string()));
}

#[test]
fn __test_embed_subs_requires_subs() {
    let mut _args = __base("https://youtube.com/watch?v=x");
    _args.subs = true;
    _args.embed_subs = true;
    let _cmd = __build_args(&_args);
    assert!(_cmd.contains(&"--embed-subs".to_string()));
}

#[test]
fn __test_embed_thumbnail() {
    let mut _args = __base("https://youtube.com/watch?v=x");
    _args.embed_thumbnail = true;
    let _cmd = __build_args(&_args);
    assert!(_cmd.contains(&"--embed-thumbnail".to_string()));
}

#[test]
fn __test_embed_metadata() {
    let mut _args = __base("https://youtube.com/watch?v=x");
    _args.embed_metadata = true;
    let _cmd = __build_args(&_args);
    assert!(_cmd.contains(&"--embed-metadata".to_string()));
}

#[test]
fn __test_concurrent_fragments() {
    let mut _args = __base("https://youtube.com/watch?v=x");
    _args.concurrent = Some(4);
    let _cmd = __build_args(&_args);
    assert!(_cmd.contains(&"--concurrent-fragments".to_string()));
    assert!(_cmd.contains(&"4".to_string()));
}

#[test]
fn __test_retries() {
    let mut _args = __base("https://youtube.com/watch?v=x");
    _args.retries = 10;
    let _cmd = __build_args(&_args);
    let _idx = _cmd.iter().position(|_s| _s == "--retries").unwrap();
    assert_eq!(_cmd[_idx + 1], "10");
}

#[test]
fn __test_cookies() {
    let mut _args = __base("https://youtube.com/watch?v=x");
    _args.cookies = Some("/home/user/cookies.txt".to_string());
    let _cmd = __build_args(&_args);
    assert!(_cmd.contains(&"--cookies".to_string()));
    assert!(_cmd.contains(&"/home/user/cookies.txt".to_string()));
}

#[test]
fn __test_proxy() {
    let mut _args = __base("https://youtube.com/watch?v=x");
    _args.proxy = Some("socks5://127.0.0.1:1080".to_string());
    let _cmd = __build_args(&_args);
    assert!(_cmd.contains(&"--proxy".to_string()));
    assert!(_cmd.contains(&"socks5://127.0.0.1:1080".to_string()));
}

#[test]
fn __test_progress_flags_in_normal_mode() {
    let _args = __base("https://youtube.com/watch?v=x");
    let _cmd = __build_args(&_args);
    assert!(_cmd.contains(&"--newline".to_string()));
    assert!(_cmd.contains(&"--progress".to_string()));
}

#[test]
fn __test_no_progress_flags_in_verbose_mode() {
    let mut _args = __base("https://youtube.com/watch?v=x");
    _args.verbose = true;
    let _cmd = __build_args(&_args);
    assert!(!_cmd.contains(&"--newline".to_string()));
}
