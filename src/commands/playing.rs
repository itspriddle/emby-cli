use std::io::{self, Write};
use std::time::Duration;

use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use crossterm::terminal::{self, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{cursor, execute};

use crate::cli::PlayingArgs;
use crate::client::Client;
use crate::config::Config;
use crate::emby::types::Session;
use crate::error::Result;
use crate::format::color::ColorConfig;
use crate::format::playing;

pub fn run(args: &PlayingArgs) -> Result<()> {
    if let Some(interval) = args.watch {
        run_watch(args, interval)
    } else {
        run_once(args)
    }
}

fn run_once(args: &PlayingArgs) -> Result<()> {
    let config = Config::load()?;
    let client = Client::new(&config);
    let sessions = fetch_sessions(&client, args)?;

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

fn run_watch(args: &PlayingArgs, interval: u64) -> Result<()> {
    let config = Config::load()?;
    let client = Client::new(&config);

    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    terminal::enable_raw_mode()?;
    let _guard = TerminalGuard;

    let colors = ColorConfig::new(args.plain);

    loop {
        execute!(
            stdout,
            cursor::MoveTo(0, 0),
            terminal::Clear(terminal::ClearType::All)
        )?;

        match fetch_sessions(&client, args) {
            Ok(sessions) => {
                let entries = playing::build_entries(&sessions);
                let output = playing::format_text(&entries, &colors);
                for line in output.lines() {
                    write!(stdout, "{line}\r\n")?;
                }
            }
            Err(e) => {
                write!(stdout, "Error fetching sessions: {e}\r\n")?;
            }
        }

        write!(
            stdout,
            "\r\nRefreshing every {interval}s | Press q or Ctrl+C to exit\r\n"
        )?;
        stdout.flush()?;

        if wait_for_quit(interval) {
            break;
        }
    }

    Ok(())
}

fn fetch_sessions(client: &Client, args: &PlayingArgs) -> Result<Vec<Session>> {
    let sessions: Vec<Session> = client.get("/Sessions")?;

    if args.users.is_empty() {
        Ok(sessions)
    } else {
        Ok(sessions
            .into_iter()
            .filter(|s| {
                s.user_name
                    .as_ref()
                    .is_some_and(|name| args.users.iter().any(|u| u == name))
            })
            .collect())
    }
}

/// Poll for quit keys (`q` or Ctrl+C) during the wait interval.
/// Returns `true` if the user wants to quit.
fn wait_for_quit(interval_secs: u64) -> bool {
    let total = Duration::from_secs(interval_secs);
    let poll_interval = Duration::from_millis(250);
    let mut elapsed = Duration::ZERO;

    while elapsed < total {
        let remaining = total.saturating_sub(elapsed);
        let wait = remaining.min(poll_interval);

        if event::poll(wait).unwrap_or(false) {
            if let Ok(Event::Key(key)) = event::read() {
                if key.code == KeyCode::Char('q')
                    || (key.code == KeyCode::Char('c')
                        && key.modifiers.contains(KeyModifiers::CONTROL))
                {
                    return true;
                }
            }
        }

        elapsed += wait;
    }

    false
}

struct TerminalGuard;

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        let _ = terminal::disable_raw_mode();
        let _ = execute!(io::stdout(), LeaveAlternateScreen);
    }
}
