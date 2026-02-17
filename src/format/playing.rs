use crate::emby::ticks;
use crate::emby::types::Session;
use crate::format::color::ColorConfig;

pub struct PlayingEntry {
    pub name: String,
    pub date: String,
    pub ip_address: String,
    pub user: String,
    pub device: String,
    pub client: String,
    pub media_type: String,
    pub rating: String,
    pub state: String,
    pub summary: String,
    pub progress_percent: u64,
    pub progress: String,
    pub duration: String,
    pub remaining: String,
    pub episode_code: String,
    pub stream: String,
    pub album: String,
    pub album_artist: String,
    pub album_track: String,
}

/// Filter sessions to those with `NowPlayingItem`, sort by IP, map to entries.
pub fn build_entries(sessions: &[Session]) -> Vec<PlayingEntry> {
    let mut active: Vec<&Session> = sessions
        .iter()
        .filter(|s| s.now_playing_item.is_some())
        .collect();

    active.sort_by(|a, b| {
        let ip_a = a.remote_end_point.as_deref().unwrap_or("");
        let ip_b = b.remote_end_point.as_deref().unwrap_or("");
        ip_to_decimal(ip_a).cmp(&ip_to_decimal(ip_b))
    });

    active.iter().map(|s| map_session(s)).collect()
}

/// Format entries as rich text output.
pub fn format_text(entries: &[PlayingEntry], colors: &ColorConfig) -> String {
    if entries.is_empty() {
        return "Nothing playing".to_string();
    }

    let bar = colors.bold(&"-".repeat(78));

    entries
        .iter()
        .map(|e| format_entry(e, colors, &bar))
        .collect::<Vec<_>>()
        .join("\n\n")
}

fn format_entry(e: &PlayingEntry, colors: &ColorConfig, bar: &str) -> String {
    let mut lines: Vec<String> = Vec::new();

    lines.push(bar.to_string());
    lines.push(String::new());

    lines.push(format!(
        "{}      {}",
        colors.blue_bold("Name:"),
        wrap_text(&e.name, 78, 11)
    ));

    if e.media_type == "Audio" {
        lines.push(format!("{}     {}", colors.blue_bold("Album:"), e.album));
        lines.push(format!(
            "{}     {}",
            colors.blue_bold("Track:"),
            e.album_track
        ));
    }

    let watched_or_listened = if e.media_type == "Audio" {
        "listened"
    } else {
        "watched"
    };

    lines.push(format!("{}  {}", colors.blue_bold("Duration:"), e.duration));
    lines.push(format!(
        "{}  {}% ({} {watched_or_listened} - {} remaining)",
        colors.blue_bold("Progress:"),
        e.progress_percent,
        e.progress,
        e.remaining
    ));
    lines.push(format!(
        "{}    {} ({}, {}@{})",
        colors.blue_bold("Player:"),
        e.client,
        e.device,
        e.user,
        e.ip_address
    ));
    lines.push(format!(
        "{}     {} ({})",
        colors.blue_bold("State:"),
        e.state,
        e.stream
    ));
    lines.push(format!("{}      {}", colors.blue_bold("Date:"), e.date));

    if e.media_type != "Audio" && e.rating != "None" {
        lines.push(format!("{}    {}", colors.blue_bold("Rating:"), e.rating));
    }

    if e.media_type != "Audio" {
        lines.push(format!(
            "{}   {}",
            colors.blue_bold("Summary:"),
            wrap_text(&e.summary, 78, 11)
        ));
    }

    lines.join("\n")
}

fn map_session(session: &Session) -> PlayingEntry {
    let npi = session.now_playing_item.as_ref().unwrap();
    let play_state = session.play_state.as_ref();
    let media_type = npi.media_type.as_deref().unwrap_or("Unknown").to_string();

    let (name, episode_code) = build_name_and_episode(npi, &media_type);
    let date = build_date(npi, &media_type);
    let (duration_seconds, progress_seconds, remaining_seconds, progress_percent) =
        build_progress(npi, play_state);

    let is_paused = play_state.and_then(|ps| ps.is_paused).unwrap_or(false);

    let client_raw = session.client.as_deref().unwrap_or("Unknown");
    let client = if client_raw.contains("Infuse") {
        "Infuse".to_string()
    } else {
        client_raw.to_string()
    };

    let summary = npi
        .overview
        .as_deref()
        .unwrap_or("")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");

    let is_audio = media_type == "Audio";

    PlayingEntry {
        name,
        date,
        ip_address: session
            .remote_end_point
            .as_deref()
            .unwrap_or("")
            .to_string(),
        user: session
            .user_name
            .as_deref()
            .unwrap_or("Unknown")
            .to_string(),
        device: session
            .device_name
            .as_deref()
            .unwrap_or("Unknown")
            .to_string(),
        client,
        rating: npi.official_rating.as_deref().unwrap_or("None").to_string(),
        state: if is_paused {
            "Paused".to_string()
        } else {
            "Playing".to_string()
        },
        summary,
        progress_percent,
        progress: ticks::seconds_to_time(progress_seconds),
        duration: ticks::seconds_to_time(duration_seconds),
        remaining: ticks::seconds_to_time(remaining_seconds),
        episode_code,
        stream: play_state
            .and_then(|ps| ps.play_method.as_deref())
            .unwrap_or("Unknown")
            .to_string(),
        album: if is_audio {
            npi.album.as_deref().unwrap_or("").to_string()
        } else {
            String::new()
        },
        album_artist: if is_audio {
            npi.album_artist.as_deref().unwrap_or("").to_string()
        } else {
            String::new()
        },
        album_track: if is_audio {
            npi.index_number.map_or_else(String::new, |n| n.to_string())
        } else {
            String::new()
        },
        media_type,
    }
}

fn build_name_and_episode(
    npi: &crate::emby::types::NowPlayingItem,
    media_type: &str,
) -> (String, String) {
    let npi_name = npi.name.as_deref().unwrap_or("Unknown");

    let episode_code = if media_type == "Episode" {
        format!(
            "S{:02}E{:02}",
            npi.parent_index_number.unwrap_or(0),
            npi.index_number.unwrap_or(0)
        )
    } else {
        String::new()
    };

    let name = if media_type == "Episode" {
        let series = npi.series_name.as_deref().unwrap_or("Unknown");
        format!("{series} - {episode_code} - {npi_name}")
    } else if media_type == "Audio" {
        let artist = npi.album_artist.as_deref().unwrap_or("Unknown");
        format!("{artist} - {npi_name}")
    } else {
        npi_name.to_string()
    };

    (name, episode_code)
}

fn build_date(npi: &crate::emby::types::NowPlayingItem, media_type: &str) -> String {
    if media_type == "Audio" {
        npi.production_year
            .map_or_else(|| "Unknown".to_string(), |y| y.to_string())
    } else {
        npi.premiere_date
            .as_deref()
            .map_or_else(|| "Unknown".to_string(), ticks::format_premiere_date)
    }
}

fn build_progress(
    npi: &crate::emby::types::NowPlayingItem,
    play_state: Option<&crate::emby::types::PlayState>,
) -> (u64, u64, u64, u64) {
    let run_time_ticks = npi.run_time_ticks.unwrap_or(0);
    let position_ticks = play_state.and_then(|ps| ps.position_ticks).unwrap_or(0);

    let duration_seconds = ticks::ticks_to_seconds(run_time_ticks);
    let progress_seconds = ticks::ticks_to_seconds(position_ticks);
    let remaining_seconds = duration_seconds.saturating_sub(progress_seconds);

    let progress_percent = if duration_seconds > 0 {
        (progress_seconds * 100 + duration_seconds / 2) / duration_seconds
    } else {
        0
    };

    (
        duration_seconds,
        progress_seconds,
        remaining_seconds,
        progress_percent,
    )
}

fn ip_to_decimal(ip: &str) -> u64 {
    let octets: Vec<u64> = ip.split('.').filter_map(|s| s.parse().ok()).collect();
    if octets.len() == 4 {
        (octets[0] << 24) + (octets[1] << 16) + (octets[2] << 8) + octets[3]
    } else {
        0
    }
}

/// Word-wrap text at `max_width`, indenting continuation lines by `indent` spaces.
fn wrap_text(text: &str, max_width: usize, indent: usize) -> String {
    let content_width = max_width - indent;
    let words: Vec<&str> = text.split_whitespace().collect();

    if words.is_empty() {
        return String::new();
    }

    let mut lines: Vec<String> = Vec::new();
    let mut current_line = String::new();

    for word in &words {
        if current_line.is_empty() {
            current_line.push_str(word);
        } else if current_line.len() + 1 + word.len() <= content_width {
            current_line.push(' ');
            current_line.push_str(word);
        } else {
            lines.push(current_line);
            current_line = word.to_string();
        }
    }

    if !current_line.is_empty() {
        lines.push(current_line);
    }

    let indent_str = " ".repeat(indent);
    lines
        .iter()
        .enumerate()
        .map(|(i, line)| {
            if i == 0 {
                line.clone()
            } else {
                format!("{indent_str}{line}")
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ip_to_decimal() {
        assert_eq!(ip_to_decimal("192.168.1.1"), 3_232_235_777);
        assert_eq!(ip_to_decimal("10.0.0.1"), 167_772_161);
        assert_eq!(ip_to_decimal("invalid"), 0);
    }

    #[test]
    fn test_wrap_text_short() {
        let result = wrap_text("Hello world", 78, 11);
        assert_eq!(result, "Hello world");
    }

    #[test]
    fn test_wrap_text_long() {
        let long = "word ".repeat(20).trim().to_string();
        let result = wrap_text(&long, 78, 11);
        assert!(result.contains('\n'));
        // Continuation lines should be indented
        for (i, line) in result.lines().enumerate() {
            if i > 0 {
                assert!(line.starts_with("           "));
            }
        }
    }
}
