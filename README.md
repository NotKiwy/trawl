# trawl

Downloads video and audio from YouTube, SoundCloud, Spotify, and anything else yt-dlp supports. No accounts, no browser extensions, just a terminal command.

![CI](https://github.com/NotKiwy/trawl/actions/workflows/ci.yml/badge.svg)
![Build](https://github.com/NotKiwy/trawl/actions/workflows/build.yml/badge.svg)
![Security](https://github.com/NotKiwy/trawl/actions/workflows/security.yml/badge.svg)
![License](https://img.shields.io/badge/license-MIT-blue)
![Release](https://img.shields.io/github/v/release/NotKiwy/trawl?include_prereleases)

## Install

**Prebuilt binary** (Linux, macOS, Windows) from [Releases](https://github.com/NotKiwy/trawl/releases).

**Via Cargo:**
```
cargo install trawl
```

trawl needs `yt-dlp` to run. If it is not found on your system, trawl will offer to download it automatically on first launch.

## Usage

```
trawl <url>
trawl <url> [options]
```

```
trawl "https://youtu.be/..."                      download best quality
trawl "https://youtu.be/..." -a -F mp3            extract as mp3
trawl "https://youtu.be/..." -Q 1080p             cap at 1080p
trawl "https://youtu.be/..." -L                   list available formats
trawl "https://youtu.be/..." -d                   dry run, print command only
trawl "https://open.spotify.com/track/..."        spotify track via youtube search
```

## Flags

### Output
| Flag | Default | Description |
|------|---------|-------------|
| `-o, --output <DIR>` | `.` | Directory to save files into |
| `-n, --filename <TEMPLATE>` | | yt-dlp filename template, e.g. `%(uploader)s - %(title)s.%(ext)s` |

### Video
| Flag | Default | Description |
|------|---------|-------------|
| `-Q, --video-quality <QUALITY>` | `best` | `best` `1080p` `720p` `480p` `360p` `worst` |
| `-f, --video-format <FORMAT>` | `mp4` | `mp4` `mkv` `webm` |

### Audio
| Flag | Default | Description |
|------|---------|-------------|
| `-a, --audio-only` | | Extract audio only |
| `-F, --audio-format <FORMAT>` | `opus` | `opus` `mp3` `flac` `m4a` `wav` |
| `-A, --audio-quality <QUALITY>` | `best` | `best` `320k` `192k` `128k` |

### Playlist
| Flag | Default | Description |
|------|---------|-------------|
| `-N, --no-playlist` | | Download single item, skip playlist |
| `-s, --playlist-start <N>` | | Start at index N (1-based) |
| `-e, --playlist-end <N>` | | End at index N (inclusive) |
| `-i, --playlist-items <ITEMS>` | | Specific items, e.g. `1,3,5-10` |

### Metadata
| Flag | Default | Description |
|------|---------|-------------|
| `-S, --subs` | | Download subtitles |
| `-l, --subs-lang <LANG>` | `en` | Subtitle language code |
| `-E, --embed-subs` | | Embed subtitles into file (requires ffmpeg) |
| `-t, --embed-thumbnail` | | Embed thumbnail (requires ffmpeg) |
| `-m, --embed-metadata` | | Embed tags (requires ffmpeg) |

### Network
| Flag | Default | Description |
|------|---------|-------------|
| `-j, --concurrent <N>` | | Parallel download fragments per file |
| `-r, --retries <N>` | `3` | Retry attempts on failure |
| `-c, --cookies <FILE>` | | Cookies file in Netscape format |
| `-p, --proxy <URL>` | | Proxy URL, e.g. `socks5://127.0.0.1:1080` |

### General
| Flag | Description |
|------|-------------|
| `-L, --list-formats` | List all available formats for a URL |
| `-d, --dry-run` | Print the yt-dlp command without running it |
| `-q, --quiet` | No output at all |
| `--log` | Show yt-dlp status lines alongside the progress bar |
| `-v, --verbose` | Full yt-dlp output |

## Spotify

Spotify tracks work without any API keys. trawl reads the track metadata from the Spotify page and searches YouTube to find and download it.

Albums and playlists require a Spotify API client ID and secret, which you can get for free at [developer.spotify.com](https://developer.spotify.com).

```
export SPOTIFY_CLIENT_ID=your_id
export SPOTIFY_CLIENT_SECRET=your_secret

trawl "https://open.spotify.com/album/..."
trawl "https://open.spotify.com/playlist/..."
```

## Star History

[![Star History Chart](https://api.star-history.com/svg?repos=NotKiwy/trawl&type=Date)](https://star-history.com/#NotKiwy/trawl&Date)

## Contributors

<a href="https://github.com/NotKiwy/trawl/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=NotKiwy/trawl" />
</a>

---

If you find this tool useful, a star goes a long way. If something is broken or missing, open an issue. Pull requests are welcome too.

[Give a star](https://github.com/NotKiwy/trawl/stargazers) · [Report a bug](https://github.com/NotKiwy/trawl/issues/new) · [Open a PR](https://github.com/NotKiwy/trawl/pulls)
