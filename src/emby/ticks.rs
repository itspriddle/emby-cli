/// Convert Emby ticks to seconds (rounded).
pub fn ticks_to_seconds(ticks: u64) -> u64 {
    (ticks + 5_000_000) / 10_000_000
}

/// Format seconds as `HH:MM:SS` or `MM:SS` if under an hour.
pub fn seconds_to_time(total_seconds: u64) -> String {
    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;

    if hours == 0 {
        format!("{minutes:02}:{seconds:02}")
    } else {
        format!("{hours:02}:{minutes:02}:{seconds:02}")
    }
}

/// Format season/episode numbers as "S01E02".
pub fn format_episode_code(season: Option<u32>, episode: Option<u32>) -> String {
    format!("S{:02}E{:02}", season.unwrap_or(0), episode.unwrap_or(0))
}

/// Parse an ISO date string like "2024-01-15T00:00:00.0000000Z" into "Jan 15, 2024".
pub fn format_premiere_date(date_str: &str) -> String {
    // Extract date part before T
    let date_part = date_str.split('T').next().unwrap_or(date_str);
    let parts: Vec<&str> = date_part.split('-').collect();

    if parts.len() != 3 {
        return date_str.to_string();
    }

    let year = parts[0];
    let month: u32 = parts[1].parse().unwrap_or(0);
    let day: u32 = parts[2].parse().unwrap_or(0);

    let month_name = match month {
        1 => "Jan",
        2 => "Feb",
        3 => "Mar",
        4 => "Apr",
        5 => "May",
        6 => "Jun",
        7 => "Jul",
        8 => "Aug",
        9 => "Sep",
        10 => "Oct",
        11 => "Nov",
        12 => "Dec",
        _ => return date_str.to_string(),
    };

    format!("{month_name} {day}, {year}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ticks_to_seconds() {
        assert_eq!(ticks_to_seconds(0), 0);
        assert_eq!(ticks_to_seconds(10_000_000), 1);
        assert_eq!(ticks_to_seconds(36_000_000_000), 3600);
        // Rounding
        assert_eq!(ticks_to_seconds(15_000_000), 2);
        assert_eq!(ticks_to_seconds(14_999_999), 1);
    }

    #[test]
    fn test_seconds_to_time() {
        assert_eq!(seconds_to_time(0), "00:00");
        assert_eq!(seconds_to_time(59), "00:59");
        assert_eq!(seconds_to_time(60), "01:00");
        assert_eq!(seconds_to_time(3599), "59:59");
        assert_eq!(seconds_to_time(3600), "01:00:00");
        assert_eq!(seconds_to_time(3661), "01:01:01");
    }

    #[test]
    fn test_format_episode_code() {
        assert_eq!(format_episode_code(Some(1), Some(2)), "S01E02");
        assert_eq!(format_episode_code(Some(10), Some(15)), "S10E15");
        assert_eq!(format_episode_code(Some(1), None), "S01E00");
        assert_eq!(format_episode_code(None, Some(5)), "S00E05");
        assert_eq!(format_episode_code(None, None), "S00E00");
    }

    #[test]
    fn test_format_premiere_date() {
        assert_eq!(
            format_premiere_date("2024-01-15T00:00:00.0000000Z"),
            "Jan 15, 2024"
        );
        assert_eq!(format_premiere_date("2023-12-01T00:00:00Z"), "Dec 1, 2023");
        assert_eq!(
            format_premiere_date("2022-06-30T12:00:00.000Z"),
            "Jun 30, 2022"
        );
    }
}
