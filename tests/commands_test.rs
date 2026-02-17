use assert_cmd::cargo::cargo_bin_cmd;
use predicates::str::contains;
use std::path::PathBuf;

fn fixture_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join(name)
}

fn load_fixture(name: &str) -> String {
    std::fs::read_to_string(fixture_path(name)).unwrap()
}

fn emby_cmd(server: &mockito::ServerGuard) -> assert_cmd::Command {
    let mut cmd = cargo_bin_cmd!("emby");
    cmd.env("EMBY_API_URL", server.url());
    cmd.env("EMBY_API_KEY", "test-api-key");
    cmd
}

// --- system ---

#[test]
fn system_shows_info() {
    let mut server = mockito::Server::new();
    let body = load_fixture("system_info.json");
    let _mock = server
        .mock("GET", "/emby/System/Info")
        .match_header("X-Emby-Token", "test-api-key")
        .with_body(&body)
        .with_header("content-type", "application/json")
        .create();

    emby_cmd(&server)
        .arg("system")
        .assert()
        .success()
        .stdout(contains("4.8.0.0"))
        .stdout(contains("MediaServer"))
        .stdout(contains("Linux"))
        .stdout(contains("Update Available: No"))
        .stdout(contains(&server.url()));
}

// --- users ---

#[test]
fn users_shows_table() {
    let mut server = mockito::Server::new();
    let body = load_fixture("users.json");
    let _mock = server
        .mock("GET", "/emby/Users")
        .with_body(&body)
        .with_header("content-type", "application/json")
        .create();

    emby_cmd(&server)
        .arg("users")
        .assert()
        .success()
        .stdout(contains("josh"))
        .stdout(contains("abc123"))
        .stdout(contains("true"))
        .stdout(contains("bob"))
        .stdout(contains("def456"))
        .stdout(contains("false"));
}

// --- devices ---

#[test]
fn devices_shows_table() {
    let mut server = mockito::Server::new();
    let body = load_fixture("devices.json");
    let _mock = server
        .mock("GET", "/emby/Devices")
        .with_body(&body)
        .with_header("content-type", "application/json")
        .create();

    emby_cmd(&server)
        .arg("devices")
        .assert()
        .success()
        .stdout(contains("Apple TV"))
        .stdout(contains("192.168.1.100"))
        .stdout(contains("Infuse"))
        .stdout(contains("device-001"));
}

// --- libraries ---

#[test]
fn libraries_shows_table() {
    let mut server = mockito::Server::new();
    let body = load_fixture("libraries.json");
    let _mock = server
        .mock("GET", "/emby/Library/VirtualFolders")
        .with_body(&body)
        .with_header("content-type", "application/json")
        .create();

    emby_cmd(&server)
        .arg("libraries")
        .assert()
        .success()
        .stdout(contains("Movies"))
        .stdout(contains("movies"))
        .stdout(contains("lib-001"))
        .stdout(contains("TV Shows"))
        .stdout(contains("tvshows"));
}

// --- activity ---

#[test]
fn activity_shows_table() {
    let mut server = mockito::Server::new();
    let body = load_fixture("activity_log.json");
    let _mock = server
        .mock("GET", "/emby/System/ActivityLog/Entries")
        .match_query(mockito::Matcher::UrlEncoded("Limit".into(), "25".into()))
        .with_body(&body)
        .with_header("content-type", "application/json")
        .create();

    emby_cmd(&server)
        .arg("activity")
        .assert()
        .success()
        .stdout(contains("josh logged in"))
        .stdout(contains("Info"))
        .stdout(contains("Login from 192.168.1.100"));
}

#[test]
fn activity_with_limit() {
    let mut server = mockito::Server::new();
    let body = load_fixture("activity_log.json");
    let _mock = server
        .mock("GET", "/emby/System/ActivityLog/Entries")
        .match_query(mockito::Matcher::UrlEncoded("Limit".into(), "5".into()))
        .with_body(&body)
        .with_header("content-type", "application/json")
        .create();

    emby_cmd(&server)
        .args(["activity", "--limit", "5"])
        .assert()
        .success();
}

// --- search ---

#[test]
fn search_shows_results() {
    let mut server = mockito::Server::new();
    let body = load_fixture("search_hints.json");
    let _mock = server
        .mock("GET", "/emby/Search/Hints")
        .match_query(mockito::Matcher::AllOf(vec![
            mockito::Matcher::UrlEncoded("SearchTerm".into(), "friends".into()),
            mockito::Matcher::UrlEncoded("Limit".into(), "25".into()),
        ]))
        .with_body(&body)
        .with_header("content-type", "application/json")
        .create();

    emby_cmd(&server)
        .args(["search", "friends"])
        .assert()
        .success()
        .stdout(contains("Friends - S03E02 - The One Where No One's Ready"))
        .stdout(contains("Queen - Bohemian Rhapsody"))
        .stdout(contains("The Matrix"));
}

#[test]
fn search_no_results() {
    let mut server = mockito::Server::new();
    let _mock = server
        .mock("GET", "/emby/Search/Hints")
        .match_query(mockito::Matcher::Any)
        .with_body(r#"{"SearchHints": []}"#)
        .with_header("content-type", "application/json")
        .create();

    emby_cmd(&server)
        .args(["search", "nonexistent"])
        .assert()
        .success()
        .stdout(contains("No results found"));
}

// --- playing ---

#[test]
fn playing_shows_text_output() {
    let mut server = mockito::Server::new();
    let body = load_fixture("sessions.json");
    let _mock = server
        .mock("GET", "/emby/Sessions")
        .with_body(&body)
        .with_header("content-type", "application/json")
        .create();

    emby_cmd(&server)
        .args(["playing", "--plain"])
        .assert()
        .success()
        .stdout(contains("Friends - S03E02"))
        .stdout(contains("Queen - Bohemian Rhapsody"))
        .stdout(contains("Duration:"))
        .stdout(contains("Progress:"));
}

#[test]
fn playing_json_output() {
    let mut server = mockito::Server::new();
    let body = load_fixture("sessions.json");
    let _mock = server
        .mock("GET", "/emby/Sessions")
        .with_body(&body)
        .with_header("content-type", "application/json")
        .create();

    let output = emby_cmd(&server)
        .args(["playing", "--json"])
        .output()
        .unwrap();

    assert!(output.status.success());
    let json: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert!(json.is_array());
    assert_eq!(json.as_array().unwrap().len(), 2);
}

#[test]
fn playing_raw_output() {
    let mut server = mockito::Server::new();
    let body = load_fixture("sessions.json");
    let _mock = server
        .mock("GET", "/emby/Sessions")
        .with_body(&body)
        .with_header("content-type", "application/json")
        .create();

    let output = emby_cmd(&server)
        .args(["playing", "--raw"])
        .output()
        .unwrap();

    assert!(output.status.success());
    let json: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert!(json.is_array());
    // Raw shows only active sessions (with NowPlayingItem)
    assert_eq!(json.as_array().unwrap().len(), 2);
}

#[test]
fn playing_nothing_playing() {
    let mut server = mockito::Server::new();
    let _mock = server
        .mock("GET", "/emby/Sessions")
        .with_body(r#"[{"UserName": "idle", "DeviceName": "Browser"}]"#)
        .with_header("content-type", "application/json")
        .create();

    emby_cmd(&server)
        .args(["playing", "--plain"])
        .assert()
        .success()
        .stdout(contains("Nothing playing"));
}

// --- tasks ---

#[test]
fn tasks_shows_table() {
    let mut server = mockito::Server::new();
    let body = load_fixture("scheduled_tasks.json");
    let _mock = server
        .mock("GET", "/emby/ScheduledTasks")
        .with_body(&body)
        .with_header("content-type", "application/json")
        .create();

    emby_cmd(&server)
        .arg("tasks")
        .assert()
        .success()
        .stdout(contains("Scan Media Library"))
        .stdout(contains("Library"))
        .stdout(contains("Idle"))
        .stdout(contains("Download Subtitles"));
}

#[test]
fn tasks_hides_hidden_by_default() {
    let mut server = mockito::Server::new();
    let body = load_fixture("scheduled_tasks.json");
    let _mock = server
        .mock("GET", "/emby/ScheduledTasks")
        .with_body(&body)
        .with_header("content-type", "application/json")
        .create();

    let output = emby_cmd(&server).arg("tasks").output().unwrap();

    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Scan Media Library"));
    assert!(!stdout.contains("Clean Cache"));
}

#[test]
fn tasks_all_shows_hidden() {
    let mut server = mockito::Server::new();
    let body = load_fixture("scheduled_tasks.json");
    let _mock = server
        .mock("GET", "/emby/ScheduledTasks")
        .with_body(&body)
        .with_header("content-type", "application/json")
        .create();

    emby_cmd(&server)
        .args(["tasks", "--all"])
        .assert()
        .success()
        .stdout(contains("Clean Cache"));
}

// --- latest ---

#[test]
fn latest_shows_table() {
    let mut server = mockito::Server::new();
    let users_body = load_fixture("users.json");
    let latest_body = load_fixture("latest_items.json");

    let _users_mock = server
        .mock("GET", "/emby/Users")
        .with_body(&users_body)
        .with_header("content-type", "application/json")
        .create();

    let _latest_mock = server
        .mock("GET", "/emby/Users/abc123/Items/Latest")
        .match_query(mockito::Matcher::Any)
        .with_body(&latest_body)
        .with_header("content-type", "application/json")
        .create();

    emby_cmd(&server)
        .arg("latest")
        .assert()
        .success()
        .stdout(contains("Inception"))
        .stdout(contains("Friends - S05E14"))
        .stdout(contains("Led Zeppelin - Stairway to Heaven"));
}

#[test]
fn latest_with_user_flag() {
    let mut server = mockito::Server::new();
    let users_body = load_fixture("users.json");
    let latest_body = load_fixture("latest_items.json");

    let _users_mock = server
        .mock("GET", "/emby/Users")
        .with_body(&users_body)
        .with_header("content-type", "application/json")
        .create();

    let _latest_mock = server
        .mock("GET", "/emby/Users/def456/Items/Latest")
        .match_query(mockito::Matcher::Any)
        .with_body(&latest_body)
        .with_header("content-type", "application/json")
        .create();

    emby_cmd(&server)
        .args(["latest", "--user", "bob"])
        .assert()
        .success()
        .stdout(contains("Inception"));
}

#[test]
fn latest_empty() {
    let mut server = mockito::Server::new();
    let users_body = load_fixture("users.json");

    let _users_mock = server
        .mock("GET", "/emby/Users")
        .with_body(&users_body)
        .with_header("content-type", "application/json")
        .create();

    let _latest_mock = server
        .mock("GET", "/emby/Users/abc123/Items/Latest")
        .match_query(mockito::Matcher::Any)
        .with_body("[]")
        .with_header("content-type", "application/json")
        .create();

    emby_cmd(&server)
        .arg("latest")
        .assert()
        .success()
        .stdout(contains("No recently added items"));
}

// --- next-up ---

#[test]
fn next_up_shows_episodes() {
    let mut server = mockito::Server::new();
    let users_body = load_fixture("users.json");
    let next_up_body = load_fixture("next_up.json");

    let _users_mock = server
        .mock("GET", "/emby/Users")
        .with_body(&users_body)
        .with_header("content-type", "application/json")
        .create();

    let _next_up_mock = server
        .mock("GET", "/emby/Shows/NextUp")
        .match_query(mockito::Matcher::Any)
        .with_body(&next_up_body)
        .with_header("content-type", "application/json")
        .create();

    emby_cmd(&server)
        .arg("next-up")
        .assert()
        .success()
        .stdout(contains("Friends"))
        .stdout(contains("S04E12"))
        .stdout(contains("Breaking Bad"))
        .stdout(contains("S05E14"));
}

#[test]
fn next_up_empty() {
    let mut server = mockito::Server::new();
    let users_body = load_fixture("users.json");

    let _users_mock = server
        .mock("GET", "/emby/Users")
        .with_body(&users_body)
        .with_header("content-type", "application/json")
        .create();

    let _next_up_mock = server
        .mock("GET", "/emby/Shows/NextUp")
        .match_query(mockito::Matcher::Any)
        .with_body(r#"{"Items": [], "TotalRecordCount": 0}"#)
        .with_header("content-type", "application/json")
        .create();

    emby_cmd(&server)
        .arg("next-up")
        .assert()
        .success()
        .stdout(contains("No next up episodes"));
}

// --- upcoming ---

#[test]
fn upcoming_shows_episodes() {
    let mut server = mockito::Server::new();
    let users_body = load_fixture("users.json");
    let upcoming_body = load_fixture("upcoming.json");

    let _users_mock = server
        .mock("GET", "/emby/Users")
        .with_body(&users_body)
        .with_header("content-type", "application/json")
        .create();

    let _upcoming_mock = server
        .mock("GET", "/emby/Shows/Upcoming")
        .match_query(mockito::Matcher::Any)
        .with_body(&upcoming_body)
        .with_header("content-type", "application/json")
        .create();

    emby_cmd(&server)
        .arg("upcoming")
        .assert()
        .success()
        .stdout(contains("The Last of Us"))
        .stdout(contains("S02E01"))
        .stdout(contains("Severance"))
        .stdout(contains("S03E01"));
}

#[test]
fn upcoming_empty() {
    let mut server = mockito::Server::new();
    let users_body = load_fixture("users.json");

    let _users_mock = server
        .mock("GET", "/emby/Users")
        .with_body(&users_body)
        .with_header("content-type", "application/json")
        .create();

    let _upcoming_mock = server
        .mock("GET", "/emby/Shows/Upcoming")
        .match_query(mockito::Matcher::Any)
        .with_body(r#"{"Items": [], "TotalRecordCount": 0}"#)
        .with_header("content-type", "application/json")
        .create();

    emby_cmd(&server)
        .arg("upcoming")
        .assert()
        .success()
        .stdout(contains("No upcoming episodes"));
}

// --- scan ---

#[test]
fn scan_all_libraries() {
    let mut server = mockito::Server::new();
    let libs_body = load_fixture("libraries.json");

    let _libs_mock = server
        .mock("GET", "/emby/Library/VirtualFolders")
        .with_body(&libs_body)
        .with_header("content-type", "application/json")
        .create();

    // Expect POST for each supported library (movies, tvshows, music — not photos)
    let _scan1 = server
        .mock("POST", "/emby/Items/lib-001/Refresh")
        .with_status(204)
        .create();
    let _scan2 = server
        .mock("POST", "/emby/Items/lib-002/Refresh")
        .with_status(204)
        .create();
    let _scan3 = server
        .mock("POST", "/emby/Items/lib-003/Refresh")
        .with_status(204)
        .create();

    emby_cmd(&server)
        .arg("scan")
        .assert()
        .success()
        .stdout(contains("Scanning for movies in library ID lib-001"))
        .stdout(contains("Scanning for tvshows in library ID lib-002"))
        .stdout(contains("Scanning for music in library ID lib-003"));
}

#[test]
fn scan_specific_type() {
    let mut server = mockito::Server::new();
    let libs_body = load_fixture("libraries.json");

    let _libs_mock = server
        .mock("GET", "/emby/Library/VirtualFolders")
        .with_body(&libs_body)
        .with_header("content-type", "application/json")
        .create();

    let _scan = server
        .mock("POST", "/emby/Items/lib-001/Refresh")
        .with_status(204)
        .create();

    emby_cmd(&server)
        .args(["scan", "movies"])
        .assert()
        .success()
        .stdout(contains("Scanning for movies"));
}

#[test]
fn scan_type_alias_shows() {
    let mut server = mockito::Server::new();
    let libs_body = load_fixture("libraries.json");

    let _libs_mock = server
        .mock("GET", "/emby/Library/VirtualFolders")
        .with_body(&libs_body)
        .with_header("content-type", "application/json")
        .create();

    let _scan = server
        .mock("POST", "/emby/Items/lib-002/Refresh")
        .with_status(204)
        .create();

    emby_cmd(&server)
        .args(["scan", "shows"])
        .assert()
        .success()
        .stdout(contains("Scanning for tvshows"));
}

#[test]
fn scan_type_alias_tv() {
    let mut server = mockito::Server::new();
    let libs_body = load_fixture("libraries.json");

    let _libs_mock = server
        .mock("GET", "/emby/Library/VirtualFolders")
        .with_body(&libs_body)
        .with_header("content-type", "application/json")
        .create();

    let _scan = server
        .mock("POST", "/emby/Items/lib-002/Refresh")
        .with_status(204)
        .create();

    emby_cmd(&server)
        .args(["scan", "tv"])
        .assert()
        .success()
        .stdout(contains("Scanning for tvshows"));
}

// --- restart ---

#[test]
fn restart_sends_post() {
    let mut server = mockito::Server::new();
    let _mock = server
        .mock("POST", "/emby/System/Restart")
        .with_status(204)
        .create();

    emby_cmd(&server).arg("restart").assert().success();
}

// --- Error cases ---

#[test]
fn error_on_401() {
    let mut server = mockito::Server::new();
    let _mock = server
        .mock("GET", "/emby/System/Info")
        .with_status(401)
        .with_body("Unauthorized")
        .create();

    emby_cmd(&server)
        .arg("system")
        .assert()
        .failure()
        .stderr(contains("Error"));
}

#[test]
fn error_on_500() {
    let mut server = mockito::Server::new();
    let _mock = server
        .mock("GET", "/emby/Users")
        .with_status(500)
        .with_body("Internal Server Error")
        .create();

    emby_cmd(&server)
        .arg("users")
        .assert()
        .failure()
        .stderr(contains("Error"));
}

// NOTE: find-server uses UDP broadcast, not HTTP. Skipping — would require
// network-level mocking which isn't worth the complexity.
