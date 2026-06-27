use std::fmt;

#[derive(Debug)]
pub enum TrawlError {
    YtDlpNotFound,
    YtDlpFailed(i32),
    YtDlpDownloadFailed(String),
    SpotifyUnsupported,
    SpotifyCredentialsMissing,
    SpotifyApiError(String),
    SpotifyInvalidUrl(String),
    SpotifyParseError(String),
    SearchNoResults(String),
    IoError(std::io::Error),
    HttpError(reqwest::Error),
}

impl fmt::Display for TrawlError {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TrawlError::YtDlpNotFound => {
                write!(
                    _f,
                    "yt-dlp not found — install it: https://github.com/yt-dlp/yt-dlp"
                )
            }
            TrawlError::YtDlpFailed(_code) => {
                write!(_f, "yt-dlp exited with code {}", _code)
            }
            TrawlError::YtDlpDownloadFailed(_reason) => {
                write!(_f, "failed to download yt-dlp: {}", _reason)
            }
            TrawlError::SpotifyUnsupported => {
                write!(
                    _f,
                    "set SPOTIFY_CLIENT_ID and SPOTIFY_CLIENT_SECRET to enable Spotify support"
                )
            }
            TrawlError::SpotifyCredentialsMissing => {
                write!(
                    _f,
                    "SPOTIFY_CLIENT_ID and SPOTIFY_CLIENT_SECRET env vars are required for Spotify"
                )
            }
            TrawlError::SpotifyApiError(_msg) => {
                write!(_f, "Spotify API error: {}", _msg)
            }
            TrawlError::SpotifyInvalidUrl(_url) => {
                write!(_f, "cannot parse Spotify URL: {}", _url)
            }
            TrawlError::SpotifyParseError(_msg) => {
                write!(_f, "failed to read Spotify page: {}", _msg)
            }
            TrawlError::SearchNoResults(_query) => {
                write!(_f, "no results found for \"{}\"", _query)
            }
            TrawlError::IoError(_e) => {
                write!(_f, "IO error: {}", _e)
            }
            TrawlError::HttpError(_e) => {
                write!(_f, "HTTP error: {}", _e)
            }
        }
    }
}

impl From<std::io::Error> for TrawlError {
    fn from(_e: std::io::Error) -> Self {
        TrawlError::IoError(_e)
    }
}

impl From<reqwest::Error> for TrawlError {
    fn from(_e: reqwest::Error) -> Self {
        TrawlError::HttpError(_e)
    }
}
