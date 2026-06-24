use reqwest::Client;
use serde::Deserialize;

use crate::error::TrawlError;

pub enum SpotifyResource {
    Track { query: String },
    Multi { name: String, queries: Vec<String> },
}

pub fn __parse_url(_url: &str) -> Result<(&str, &str), TrawlError> {
    let _host_and_path = _url
        .strip_prefix("https://")
        .or_else(|| _url.strip_prefix("http://"))
        .unwrap_or(_url);

    let _valid = _host_and_path.starts_with("open.spotify.com/")
        || _host_and_path.starts_with("spotify.com/");

    if !_valid {
        return Err(TrawlError::SpotifyInvalidUrl(_url.to_string()));
    }

    let _path = _host_and_path
        .split_once('/')
        .map(|x| x.1)
        .ok_or_else(|| TrawlError::SpotifyInvalidUrl(_url.to_string()))?;

    let _parts: Vec<&str> = _path.splitn(2, '/').collect();
    if _parts.len() < 2 {
        return Err(TrawlError::SpotifyInvalidUrl(_url.to_string()));
    }

    let _kind = _parts[0];
    let _id = _parts[1].split('?').next().unwrap_or(_parts[1]);

    Ok((_kind, _id))
}

pub async fn __resolve(_url: &str) -> Result<SpotifyResource, TrawlError> {
    let (_kind, _id) = __parse_url(_url)?;

    match _kind {
        "track" => {
            let (_title, _artist) = __scrape_track(_url).await?;
            Ok(SpotifyResource::Track {
                query: format!("ytsearch1:{} - {}", _artist, _title),
            })
        }
        "album" | "playlist" => __resolve_with_api(_url, _kind, _id).await,
        _ => Err(TrawlError::SpotifyInvalidUrl(_url.to_string())),
    }
}

async fn __scrape_track(_url: &str) -> Result<(String, String), TrawlError> {
    let _client = Client::builder()
        .user_agent("Mozilla/5.0 (compatible; Googlebot/2.1; +http://www.google.com/bot.html)")
        .build()?;

    let _html = _client.get(_url).send().await?.text().await?;

    let _title = __extract_meta(&_html, "og:title")
        .ok_or_else(|| TrawlError::SpotifyParseError("og:title not found".into()))?;

    let _desc = __extract_meta(&_html, "og:description")
        .ok_or_else(|| TrawlError::SpotifyParseError("og:description not found".into()))?;

    let _artist = _desc
        .split(" \u{00b7} ")
        .next()
        .unwrap_or(&_desc)
        .trim()
        .to_string();

    Ok((__clean(&_title), __clean(&_artist)))
}

fn __extract_meta(_html: &str, _property: &str) -> Option<String> {
    let _needle = format!("\"{}\"", _property);
    let _tag_pos = _html.find(&_needle)?;

    let _before = &_html[.._tag_pos];
    let _tag_start = _before.rfind('<')?;
    let _tag_end = _html[_tag_start..].find('>')?;
    let _tag = &_html[_tag_start.._tag_start + _tag_end + 1];

    __extract_attr(_tag, "content")
}

fn __extract_attr(_tag: &str, _attr: &str) -> Option<String> {
    let _needle = format!("{}=\"", _attr);
    let _pos = _tag.find(&_needle)?;
    let _start = _pos + _needle.len();
    let _rest = &_tag[_start..];
    let _end = _rest.find('"')?;
    Some(__decode_entities(&_rest[.._end]))
}

fn __clean(_s: &str) -> String {
    _s.replace('\u{060C}', ",")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

fn __decode_entities(_s: &str) -> String {
    _s.replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
        .replace("&apos;", "'")
}

async fn __resolve_with_api(
    _url: &str,
    _kind: &str,
    _id: &str,
) -> Result<SpotifyResource, TrawlError> {
    let _client_id =
        std::env::var("SPOTIFY_CLIENT_ID").map_err(|_| TrawlError::SpotifyCredentialsMissing)?;
    let _client_secret = std::env::var("SPOTIFY_CLIENT_SECRET")
        .map_err(|_| TrawlError::SpotifyCredentialsMissing)?;

    let _client = Client::new();
    let _token = __get_token(&_client, &_client_id, &_client_secret).await?;

    match _kind {
        "album" => {
            let _album = __fetch_album(&_client, &_token, _id).await?;
            let _queries = _album
                .tracks
                .items
                .iter()
                .map(|_t| {
                    let _artist = _t.artists.first().map(|_a| _a.name.as_str()).unwrap_or("");
                    format!("ytsearch1:{} - {}", _artist, _t.name)
                })
                .collect();
            Ok(SpotifyResource::Multi {
                name: _album.name,
                queries: _queries,
            })
        }
        "playlist" => {
            let _playlist = __fetch_playlist(&_client, &_token, _id).await?;
            let _queries = _playlist
                .tracks
                .items
                .iter()
                .filter_map(|_item| _item.track.as_ref())
                .map(|_t| {
                    let _artist = _t.artists.first().map(|_a| _a.name.as_str()).unwrap_or("");
                    format!("ytsearch1:{} - {}", _artist, _t.name)
                })
                .collect();
            Ok(SpotifyResource::Multi {
                name: _playlist.name,
                queries: _queries,
            })
        }
        _ => Err(TrawlError::SpotifyInvalidUrl(_url.to_string())),
    }
}

#[derive(Deserialize)]
struct TokenResponse {
    access_token: String,
}

#[derive(Deserialize)]
struct SpotifyArtist {
    name: String,
}

#[derive(Deserialize)]
struct SpotifyTrack {
    name: String,
    artists: Vec<SpotifyArtist>,
}

#[derive(Deserialize)]
struct SpotifyAlbum {
    name: String,
    tracks: AlbumTracks,
}

#[derive(Deserialize)]
struct AlbumTracks {
    items: Vec<AlbumTrackItem>,
}

#[derive(Deserialize)]
struct AlbumTrackItem {
    name: String,
    artists: Vec<SpotifyArtist>,
}

#[derive(Deserialize)]
struct SpotifyPlaylist {
    name: String,
    tracks: PlaylistTracks,
}

#[derive(Deserialize)]
struct PlaylistTracks {
    items: Vec<PlaylistTrackItem>,
}

#[derive(Deserialize)]
struct PlaylistTrackItem {
    track: Option<SpotifyTrack>,
}

async fn __get_token(_client: &Client, _id: &str, _secret: &str) -> Result<String, TrawlError> {
    let _resp = _client
        .post("https://accounts.spotify.com/api/token")
        .basic_auth(_id, Some(_secret))
        .form(&[("grant_type", "client_credentials")])
        .send()
        .await?;

    if !_resp.status().is_success() {
        return Err(TrawlError::SpotifyApiError(format!(
            "token request failed with {}",
            _resp.status()
        )));
    }

    Ok(_resp.json::<TokenResponse>().await?.access_token)
}

async fn __fetch_album(
    _client: &Client,
    _token: &str,
    _id: &str,
) -> Result<SpotifyAlbum, TrawlError> {
    let _resp = _client
        .get(format!("https://api.spotify.com/v1/albums/{}", _id))
        .bearer_auth(_token)
        .send()
        .await?;

    if !_resp.status().is_success() {
        return Err(TrawlError::SpotifyApiError(format!(
            "album fetch failed with {}",
            _resp.status()
        )));
    }

    Ok(_resp.json::<SpotifyAlbum>().await?)
}

async fn __fetch_playlist(
    _client: &Client,
    _token: &str,
    _id: &str,
) -> Result<SpotifyPlaylist, TrawlError> {
    let _resp = _client
        .get(format!("https://api.spotify.com/v1/playlists/{}", _id))
        .bearer_auth(_token)
        .send()
        .await?;

    if !_resp.status().is_success() {
        return Err(TrawlError::SpotifyApiError(format!(
            "playlist fetch failed with {}",
            _resp.status()
        )));
    }

    Ok(_resp.json::<SpotifyPlaylist>().await?)
}
