#![allow(dead_code)]

use serde::{Deserialize, Serialize};

// --- Sessions ---

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Session {
    pub user_name: Option<String>,
    pub device_name: Option<String>,
    pub client: Option<String>,
    pub remote_end_point: Option<String>,
    pub now_playing_item: Option<NowPlayingItem>,
    pub play_state: Option<PlayState>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct NowPlayingItem {
    pub name: Option<String>,
    #[serde(rename = "Type")]
    pub media_type: Option<String>,
    pub series_name: Option<String>,
    pub parent_index_number: Option<u32>,
    pub index_number: Option<u32>,
    pub run_time_ticks: Option<u64>,
    pub premiere_date: Option<String>,
    pub official_rating: Option<String>,
    pub overview: Option<String>,
    pub production_year: Option<u32>,
    pub album: Option<String>,
    pub album_artist: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct PlayState {
    pub position_ticks: Option<u64>,
    pub is_paused: Option<bool>,
    pub play_method: Option<String>,
}

// --- Virtual Folders (Libraries) ---

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct VirtualFolder {
    pub name: Option<String>,
    pub collection_type: Option<String>,
    pub item_id: Option<String>,
}

// --- System Info ---

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SystemInfo {
    pub version: Option<String>,
    pub server_name: Option<String>,
    pub operating_system_display_name: Option<String>,
    pub has_update_available: Option<bool>,
}

// --- Users ---

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct User {
    pub name: Option<String>,
    pub id: Option<String>,
    pub policy: Option<UserPolicy>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UserPolicy {
    pub is_administrator: Option<bool>,
}

// --- Activity Log ---

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ActivityLogResponse {
    pub items: Option<Vec<ActivityLogEntry>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ActivityLogEntry {
    pub name: Option<String>,
    pub overview: Option<String>,
    pub short_overview: Option<String>,
    #[serde(rename = "Type")]
    pub entry_type: Option<String>,
    pub date: Option<String>,
    pub severity: Option<String>,
}

// --- BaseItemDto (shared by latest, next-up, upcoming) ---

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BaseItemDto {
    pub name: Option<String>,
    pub id: Option<String>,
    #[serde(rename = "Type")]
    pub media_type: Option<String>,
    pub series_name: Option<String>,
    pub index_number: Option<u32>,
    pub parent_index_number: Option<u32>,
    pub production_year: Option<u32>,
    pub premiere_date: Option<String>,
    pub date_created: Option<String>,
    pub run_time_ticks: Option<u64>,
    pub overview: Option<String>,
    pub container: Option<String>,
    pub official_rating: Option<String>,
    pub album: Option<String>,
    pub album_artist: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct QueryResultBaseItemDto {
    pub items: Option<Vec<BaseItemDto>>,
    pub total_record_count: Option<u32>,
}

// --- Scheduled Tasks ---

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TaskInfo {
    pub name: Option<String>,
    pub state: Option<String>,
    pub current_progress_percentage: Option<f64>,
    pub id: Option<String>,
    pub last_execution_result: Option<TaskResult>,
    pub description: Option<String>,
    pub category: Option<String>,
    pub is_hidden: Option<bool>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TaskResult {
    pub start_time_utc: Option<String>,
    pub end_time_utc: Option<String>,
    pub status: Option<String>,
}

// --- Devices ---

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DevicesResponse {
    pub items: Option<Vec<Device>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Device {
    pub name: Option<String>,
    pub ip_address: Option<String>,
    pub last_user_name: Option<String>,
    pub app_name: Option<String>,
    pub app_version: Option<String>,
    pub id: Option<String>,
}
