use std::path::PathBuf;

// We test the playing formatter by deserializing fixture data and running it
// through the public format functions. This doesn't require a live Emby server.

fn fixture_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join(name)
}

fn load_sessions() -> Vec<emby_cli::emby::types::Session> {
    let data = std::fs::read_to_string(fixture_path("sessions.json")).unwrap();
    serde_json::from_str(&data).unwrap()
}

#[test]
fn playing_builds_entries_from_sessions() {
    let sessions = load_sessions();
    let entries = emby_cli::format::playing::build_entries(&sessions);

    // Should have 2 active sessions (the idle one has no NowPlayingItem)
    assert_eq!(entries.len(), 2);
}

#[test]
fn playing_sorts_by_ip() {
    let sessions = load_sessions();
    let entries = emby_cli::format::playing::build_entries(&sessions);

    // 192.168.1.50 < 192.168.1.100
    assert_eq!(entries[0].ip_address, "192.168.1.50");
    assert_eq!(entries[1].ip_address, "192.168.1.100");
}

#[test]
fn playing_formats_episode_correctly() {
    let sessions = load_sessions();
    let entries = emby_cli::format::playing::build_entries(&sessions);

    let episode = entries.iter().find(|e| e.media_type == "Episode").unwrap();
    assert_eq!(
        episode.name,
        "Friends - S03E02 - The One Where No One's Ready"
    );
    assert_eq!(episode.episode_code, "S03E02");
    assert_eq!(episode.rating, "TV-PG");
    assert_eq!(episode.date, "Sep 26, 1996");
}

#[test]
fn playing_formats_audio_correctly() {
    let sessions = load_sessions();
    let entries = emby_cli::format::playing::build_entries(&sessions);

    let audio = entries.iter().find(|e| e.media_type == "Audio").unwrap();
    assert_eq!(audio.name, "Queen - Bohemian Rhapsody");
    assert_eq!(audio.album, "A Night at the Opera");
    assert_eq!(audio.album_track, "11");
    assert_eq!(audio.date, "1975");
    assert_eq!(audio.state, "Paused");
}

#[test]
fn playing_calculates_progress() {
    let sessions = load_sessions();
    let entries = emby_cli::format::playing::build_entries(&sessions);

    let episode = entries.iter().find(|e| e.media_type == "Episode").unwrap();
    // 7200000000 / 14400000000 = 50%
    assert_eq!(episode.progress_percent, 50);
    assert_eq!(episode.duration, "24:00");
    assert_eq!(episode.progress, "12:00");
    assert_eq!(episode.remaining, "12:00");
}

#[test]
fn playing_infuse_client_normalized() {
    let sessions = load_sessions();
    let entries = emby_cli::format::playing::build_entries(&sessions);

    let episode = entries.iter().find(|e| e.media_type == "Episode").unwrap();
    assert_eq!(episode.client, "Infuse");
}

#[test]
fn playing_text_output_nothing_playing() {
    let colors = emby_cli::format::color::ColorConfig::new(true);
    let output = emby_cli::format::playing::format_text(&[], &colors);
    assert_eq!(output, "Nothing playing");
}

#[test]
fn playing_text_output_contains_entries() {
    let sessions = load_sessions();
    let entries = emby_cli::format::playing::build_entries(&sessions);
    let colors = emby_cli::format::color::ColorConfig::new(true);
    let output = emby_cli::format::playing::format_text(&entries, &colors);

    assert!(output.contains("Friends - S03E02"));
    assert!(output.contains("Queen - Bohemian Rhapsody"));
    assert!(output.contains("Duration:"));
    assert!(output.contains("Progress:"));
    assert!(output.contains("Player:"));
    assert!(output.contains("Album:"));
}
