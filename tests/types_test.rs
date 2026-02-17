use std::path::PathBuf;

use emby_cli::emby::types::{
    ActivityLogResponse, BaseItemDto, DevicesResponse, QueryResultBaseItemDto, Session, SystemInfo,
    TaskInfo, User, VirtualFolder,
};

fn fixture_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join(name)
}

fn load_fixture(name: &str) -> String {
    std::fs::read_to_string(fixture_path(name)).unwrap()
}

// --- Existing fixtures ---

#[test]
fn deserialize_sessions() {
    let data = load_fixture("sessions.json");
    let sessions: Vec<Session> = serde_json::from_str(&data).unwrap();
    assert_eq!(sessions.len(), 3);
    assert_eq!(sessions[0].user_name.as_deref(), Some("josh"));
    assert!(sessions[0].now_playing_item.is_some());
    assert!(sessions[2].now_playing_item.is_none());
}

#[test]
fn deserialize_users() {
    let data = load_fixture("users.json");
    let users: Vec<User> = serde_json::from_str(&data).unwrap();
    assert_eq!(users.len(), 2);
    assert_eq!(users[0].name.as_deref(), Some("josh"));
    assert_eq!(users[0].id.as_deref(), Some("abc123"));
    assert_eq!(
        users[0].policy.as_ref().and_then(|p| p.is_administrator),
        Some(true)
    );
    assert_eq!(
        users[1].policy.as_ref().and_then(|p| p.is_administrator),
        Some(false)
    );
}

#[test]
fn deserialize_devices() {
    let data = load_fixture("devices.json");
    let response: DevicesResponse = serde_json::from_str(&data).unwrap();
    let devices = response.items.unwrap();
    assert_eq!(devices.len(), 2);
    assert_eq!(devices[0].name.as_deref(), Some("Apple TV"));
    assert_eq!(devices[0].ip_address.as_deref(), Some("192.168.1.100"));
    assert_eq!(devices[1].app_name.as_deref(), Some("Emby Mobile"));
}

#[test]
fn deserialize_libraries() {
    let data = load_fixture("libraries.json");
    let libraries: Vec<VirtualFolder> = serde_json::from_str(&data).unwrap();
    assert_eq!(libraries.len(), 4);
    assert_eq!(libraries[0].name.as_deref(), Some("Movies"));
    assert_eq!(libraries[0].collection_type.as_deref(), Some("movies"));
    assert_eq!(libraries[0].item_id.as_deref(), Some("lib-001"));
}

#[test]
fn deserialize_system_info() {
    let data = load_fixture("system_info.json");
    let info: SystemInfo = serde_json::from_str(&data).unwrap();
    assert_eq!(info.version.as_deref(), Some("4.8.0.0"));
    assert_eq!(info.server_name.as_deref(), Some("MediaServer"));
    assert_eq!(info.operating_system_display_name.as_deref(), Some("Linux"));
    assert_eq!(info.has_update_available, Some(false));
}

// --- New fixtures ---

#[test]
fn deserialize_activity_log() {
    let data = load_fixture("activity_log.json");
    let response: ActivityLogResponse = serde_json::from_str(&data).unwrap();
    let items = response.items.unwrap();
    assert_eq!(items.len(), 3);
    assert_eq!(items[0].name.as_deref(), Some("josh logged in"));
    assert_eq!(items[0].severity.as_deref(), Some("Info"));
    assert_eq!(
        items[0].short_overview.as_deref(),
        Some("Login from 192.168.1.100")
    );
    // Third entry has no Overview, only ShortOverview
    assert!(items[2].overview.is_none());
    assert_eq!(
        items[2].short_overview.as_deref(),
        Some("bob played The Matrix")
    );
}

#[test]
fn deserialize_search_hints() {
    let data = load_fixture("search_hints.json");
    let response: QueryResultBaseItemDto = serde_json::from_str(&data).unwrap();
    let items = response.items.unwrap();
    assert_eq!(items.len(), 3);

    // Episode
    assert_eq!(items[0].media_type.as_deref(), Some("Episode"));
    assert_eq!(items[0].series_name.as_deref(), Some("Friends"));
    assert_eq!(items[0].parent_index_number, Some(3));
    assert_eq!(items[0].index_number, Some(2));

    // Audio
    assert_eq!(items[1].media_type.as_deref(), Some("Audio"));
    assert_eq!(items[1].album_artist.as_deref(), Some("Queen"));

    // Movie
    assert_eq!(items[2].media_type.as_deref(), Some("Movie"));
    assert_eq!(items[2].id.as_deref(), Some("1003"));
}

#[test]
fn deserialize_scheduled_tasks() {
    let data = load_fixture("scheduled_tasks.json");
    let tasks: Vec<TaskInfo> = serde_json::from_str(&data).unwrap();
    assert_eq!(tasks.len(), 3);

    assert_eq!(tasks[0].name.as_deref(), Some("Scan Media Library"));
    assert_eq!(tasks[0].category.as_deref(), Some("Library"));
    assert_eq!(tasks[0].is_hidden, Some(false));
    assert_eq!(
        tasks[0]
            .last_execution_result
            .as_ref()
            .and_then(|r| r.status.as_deref()),
        Some("Completed")
    );

    // Hidden task
    assert_eq!(tasks[2].is_hidden, Some(true));
}

#[test]
fn deserialize_latest_items() {
    let data = load_fixture("latest_items.json");
    let items: Vec<BaseItemDto> = serde_json::from_str(&data).unwrap();
    assert_eq!(items.len(), 3);

    assert_eq!(items[0].media_type.as_deref(), Some("Movie"));
    assert_eq!(items[0].name.as_deref(), Some("Inception"));

    assert_eq!(items[1].media_type.as_deref(), Some("Episode"));
    assert_eq!(items[1].series_name.as_deref(), Some("Friends"));

    assert_eq!(items[2].media_type.as_deref(), Some("Audio"));
    assert_eq!(items[2].album_artist.as_deref(), Some("Led Zeppelin"));
}

#[test]
fn deserialize_next_up() {
    let data = load_fixture("next_up.json");
    let response: QueryResultBaseItemDto = serde_json::from_str(&data).unwrap();
    assert_eq!(response.total_record_count, Some(2));
    let items = response.items.unwrap();
    assert_eq!(items.len(), 2);
    assert_eq!(items[0].series_name.as_deref(), Some("Friends"));
    assert_eq!(items[1].series_name.as_deref(), Some("Breaking Bad"));
}

#[test]
fn deserialize_upcoming() {
    let data = load_fixture("upcoming.json");
    let response: QueryResultBaseItemDto = serde_json::from_str(&data).unwrap();
    assert_eq!(response.total_record_count, Some(2));
    let items = response.items.unwrap();
    assert_eq!(items.len(), 2);
    assert_eq!(items[0].series_name.as_deref(), Some("The Last of Us"));
    assert_eq!(items[1].series_name.as_deref(), Some("Severance"));
}

// --- Edge cases ---

#[test]
fn deserialize_empty_items_array() {
    let data = r#"{"Items": [], "TotalRecordCount": 0}"#;
    let response: QueryResultBaseItemDto = serde_json::from_str(data).unwrap();
    assert!(response.items.unwrap().is_empty());
    assert_eq!(response.total_record_count, Some(0));
}

#[test]
fn deserialize_missing_optional_fields() {
    let data = r#"[{"Name": "Test"}]"#;
    let users: Vec<User> = serde_json::from_str(data).unwrap();
    assert_eq!(users.len(), 1);
    assert_eq!(users[0].name.as_deref(), Some("Test"));
    assert!(users[0].id.is_none());
    assert!(users[0].policy.is_none());
}

#[test]
fn deserialize_session_without_play_state() {
    let data = r#"[{"UserName": "test", "DeviceName": "browser"}]"#;
    let sessions: Vec<Session> = serde_json::from_str(data).unwrap();
    assert_eq!(sessions.len(), 1);
    assert!(sessions[0].now_playing_item.is_none());
    assert!(sessions[0].play_state.is_none());
}

#[test]
fn deserialize_empty_search_hints() {
    let data = r#"{"Items": [], "TotalRecordCount": 0}"#;
    let response: QueryResultBaseItemDto = serde_json::from_str(data).unwrap();
    assert!(response.items.unwrap().is_empty());
}

#[test]
fn deserialize_empty_activity_log() {
    let data = r#"{"Items": []}"#;
    let response: ActivityLogResponse = serde_json::from_str(data).unwrap();
    assert!(response.items.unwrap().is_empty());
}

#[test]
fn deserialize_empty_devices() {
    let data = r#"{"Items": []}"#;
    let response: DevicesResponse = serde_json::from_str(data).unwrap();
    assert!(response.items.unwrap().is_empty());
}
