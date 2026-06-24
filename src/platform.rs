#[derive(Debug, PartialEq)]
pub enum Platform {
    YouTube,
    SoundCloud,
    Spotify,
    Generic,
}

pub fn __detect(_url: &str) -> Platform {
    if _url.contains("youtube.com") || _url.contains("youtu.be") {
        Platform::YouTube
    } else if _url.contains("soundcloud.com") {
        Platform::SoundCloud
    } else if _url.contains("spotify.com") {
        Platform::Spotify
    } else {
        Platform::Generic
    }
}

pub fn __name(_platform: &Platform) -> &'static str {
    match _platform {
        Platform::YouTube => "YouTube",
        Platform::SoundCloud => "SoundCloud",
        Platform::Spotify => "Spotify",
        Platform::Generic => "Generic (yt-dlp)",
    }
}
