use clap::builder::styling::{AnsiColor, Effects, Styles};
use clap::Parser;

fn __styles() -> Styles {
    Styles::styled()
        .header(AnsiColor::White.on_default() | Effects::BOLD)
        .usage(AnsiColor::White.on_default() | Effects::BOLD)
        .literal(AnsiColor::Cyan.on_default())
        .placeholder(AnsiColor::BrightBlack.on_default())
        .error(AnsiColor::Red.on_default() | Effects::BOLD)
        .valid(AnsiColor::Green.on_default())
        .invalid(AnsiColor::Red.on_default())
}

#[derive(Parser, Debug)]
#[command(
    name = "trawl",
    about = "Download media from YouTube, SoundCloud, Spotify, and more",
    version,
    styles = __styles(),
    help_template = "{about}\n\n{usage-heading} {usage}\n\n{all-args}",
)]
pub struct Args {
    #[arg(
        value_name = "URL",
        help = "YouTube, SoundCloud, Spotify, or any yt-dlp supported URL"
    )]
    pub url: String,

    #[arg(
        short = 'o',
        long,
        value_name = "DIR",
        default_value = ".",
        help = "Save to directory",
        help_heading = "Output"
    )]
    pub output: String,

    #[arg(
        short = 'n',
        long,
        value_name = "TEMPLATE",
        help = "Custom filename template (yt-dlp format)",
        help_heading = "Output"
    )]
    pub filename: Option<String>,

    #[arg(
        short = 'Q',
        long,
        value_name = "QUALITY",
        default_value = "best",
        help = "best · 1080p · 720p · 480p · 360p · worst",
        help_heading = "Video"
    )]
    pub video_quality: String,

    #[arg(
        short = 'f',
        long,
        value_name = "FORMAT",
        default_value = "mp4",
        help = "mp4 · mkv · webm",
        help_heading = "Video"
    )]
    pub video_format: String,

    #[arg(short = 'a', long, help = "Extract audio only", help_heading = "Audio")]
    pub audio_only: bool,

    #[arg(
        short = 'F',
        long,
        value_name = "FORMAT",
        default_value = "opus",
        help = "opus · mp3 · flac · m4a · wav",
        help_heading = "Audio"
    )]
    pub audio_format: String,

    #[arg(
        short = 'A',
        long,
        value_name = "QUALITY",
        default_value = "best",
        help = "best · 320k · 192k · 128k",
        help_heading = "Audio"
    )]
    pub audio_quality: String,

    #[arg(
        short = 'N',
        long,
        help = "Single item only, skip playlist",
        help_heading = "Playlist"
    )]
    pub no_playlist: bool,

    #[arg(
        short = 's',
        long,
        value_name = "N",
        help = "Start index (1-based)",
        help_heading = "Playlist"
    )]
    pub playlist_start: Option<u32>,

    #[arg(
        short = 'e',
        long,
        value_name = "N",
        help = "End index (inclusive)",
        help_heading = "Playlist"
    )]
    pub playlist_end: Option<u32>,

    #[arg(
        short = 'i',
        long,
        value_name = "ITEMS",
        help = "Items to download, e.g. 1,3,5-10",
        help_heading = "Playlist"
    )]
    pub playlist_items: Option<String>,

    #[arg(
        short = 'S',
        long,
        help = "Download subtitles",
        help_heading = "Metadata"
    )]
    pub subs: bool,

    #[arg(
        short = 'l',
        long,
        value_name = "LANG",
        default_value = "en",
        help = "Subtitle language code",
        help_heading = "Metadata"
    )]
    pub subs_lang: String,

    #[arg(
        short = 'E',
        long,
        help = "Embed subtitles into file (requires ffmpeg)",
        help_heading = "Metadata"
    )]
    pub embed_subs: bool,

    #[arg(
        short = 't',
        long,
        help = "Embed thumbnail (requires ffmpeg)",
        help_heading = "Metadata"
    )]
    pub embed_thumbnail: bool,

    #[arg(
        short = 'm',
        long,
        help = "Embed metadata tags (requires ffmpeg)",
        help_heading = "Metadata"
    )]
    pub embed_metadata: bool,

    #[arg(
        short = 'j',
        long,
        value_name = "N",
        help = "Parallel download fragments per file",
        help_heading = "Network"
    )]
    pub concurrent: Option<u32>,

    #[arg(
        short = 'r',
        long,
        value_name = "N",
        default_value = "3",
        help = "Retry attempts on failure",
        help_heading = "Network"
    )]
    pub retries: u32,

    #[arg(
        short = 'c',
        long,
        value_name = "FILE",
        help = "Cookies file (Netscape format)",
        help_heading = "Network"
    )]
    pub cookies: Option<String>,

    #[arg(
        short = 'p',
        long,
        value_name = "URL",
        help = "Proxy URL, e.g. socks5://127.0.0.1:1080",
        help_heading = "Network"
    )]
    pub proxy: Option<String>,

    #[arg(short = 'L', long, help = "List available formats for URL")]
    pub list_formats: bool,

    #[arg(short = 'd', long, help = "Show command without downloading")]
    pub dry_run: bool,

    #[arg(short = 'q', long, help = "Suppress all output")]
    pub quiet: bool,

    #[arg(long, help = "Show yt-dlp status lines alongside progress bar")]
    pub log: bool,

    #[arg(short = 'v', long, help = "Full yt-dlp verbose output")]
    pub verbose: bool,
}
