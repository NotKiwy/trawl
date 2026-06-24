use trawl::spotify::__parse_url;

#[test]
fn __test_parse_track_url() {
    let (_kind, _id) =
        __parse_url("https://open.spotify.com/track/4iV5W9uYEdYUVa79Axb7Rh").unwrap();
    assert_eq!(_kind, "track");
    assert_eq!(_id, "4iV5W9uYEdYUVa79Axb7Rh");
}

#[test]
fn __test_parse_album_url() {
    let (_kind, _id) =
        __parse_url("https://open.spotify.com/album/1DFixLWuPkv3KT3TnV35m3").unwrap();
    assert_eq!(_kind, "album");
    assert_eq!(_id, "1DFixLWuPkv3KT3TnV35m3");
}

#[test]
fn __test_parse_playlist_url() {
    let (_kind, _id) =
        __parse_url("https://open.spotify.com/playlist/37i9dQZF1DXcBWIGoYBM5M").unwrap();
    assert_eq!(_kind, "playlist");
    assert_eq!(_id, "37i9dQZF1DXcBWIGoYBM5M");
}

#[test]
fn __test_strips_query_params() {
    let (_kind, _id) = __parse_url(
        "https://open.spotify.com/track/4iV5W9uYEdYUVa79Axb7Rh?si=abc123&utm_source=copy-link",
    )
    .unwrap();
    assert_eq!(_kind, "track");
    assert_eq!(_id, "4iV5W9uYEdYUVa79Axb7Rh");
}

#[test]
fn __test_invalid_url_returns_error() {
    assert!(__parse_url("https://notspotify.com/track/abc").is_err());
}

#[test]
fn __test_missing_id_returns_error() {
    assert!(__parse_url("https://open.spotify.com/track").is_err());
}
