use crate::cli::PlayingArgs;
use crate::client::Client;
use crate::config::Config;
use crate::emby::types::Session;
use crate::error::Result;
use crate::format::color::ColorConfig;
use crate::format::playing;

pub fn run(args: &PlayingArgs) -> Result<()> {
    let config = Config::load()?;
    let client = Client::new(&config);
    let sessions: Vec<Session> = client.get("/Sessions")?;

    // Filter by user names if provided
    let sessions: Vec<Session> = if args.users.is_empty() {
        sessions
    } else {
        sessions
            .into_iter()
            .filter(|s| {
                s.user_name
                    .as_ref()
                    .is_some_and(|name| args.users.iter().any(|u| u == name))
            })
            .collect()
    };

    if args.raw {
        let active: Vec<&Session> = sessions
            .iter()
            .filter(|s| s.now_playing_item.is_some())
            .collect();
        println!("{}", serde_json::to_string_pretty(&active)?);
        return Ok(());
    }

    let entries = playing::build_entries(&sessions);

    if args.json {
        let json: Vec<serde_json::Value> = entries
            .iter()
            .map(|e| {
                serde_json::json!({
                    "name": e.name,
                    "date": e.date,
                    "ip_address": e.ip_address,
                    "user": e.user,
                    "device": e.device,
                    "client": e.client,
                    "media_type": e.media_type,
                    "rating": e.rating,
                    "state": e.state,
                    "summary": e.summary,
                    "progress_percent": e.progress_percent,
                    "progress": e.progress,
                    "duration": e.duration,
                    "remaining": e.remaining,
                    "episode_code": e.episode_code,
                    "stream": e.stream,
                    "album": e.album,
                    "album_artist": e.album_artist,
                    "album_track": e.album_track,
                })
            })
            .collect();

        let colors = ColorConfig::new(args.plain);
        if colors.enabled {
            // Colored JSON output
            let json_str = serde_json::to_string_pretty(&json)?;
            println!("{json_str}");
        } else {
            println!("{}", serde_json::to_string_pretty(&json)?);
        }
        return Ok(());
    }

    let colors = ColorConfig::new(args.plain);
    println!("{}", playing::format_text(&entries, &colors));

    Ok(())
}
