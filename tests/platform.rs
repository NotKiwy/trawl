use trawl::platform::{__detect, Platform};

#[test]
fn __test_youtube_full_url() {
    assert_eq!(
        __detect("https://www.youtube.com/watch?v=dQw4w9WgXcQ"),
        Platform::YouTube
    );
}

#[test]
fn __test_youtube_short_url() {
    assert_eq!(__detect("https://youtu.be/dQw4w9WgXcQ"), Platform::YouTube);
}

#[test]
fn __test_soundcloud() {
    assert_eq!(
        __detect("https://soundcloud.com/artist/track"),
        Platform::SoundCloud
    );
}

#[test]
fn __test_spotify() {
    assert_eq!(
        __detect("https://open.spotify.com/track/abc123"),
        Platform::Spotify
    );
}

#[test]
fn __test_generic_vimeo() {
    assert_eq!(__detect("https://vimeo.com/123456789"), Platform::Generic);
}

#[test]
fn __test_generic_twitch() {
    assert_eq!(
        __detect("https://www.twitch.tv/videos/123456789"),
        Platform::Generic
    );
}

#[test]
fn __test_generic_unknown() {
    assert_eq!(__detect("https://example.com/video"), Platform::Generic);
}
