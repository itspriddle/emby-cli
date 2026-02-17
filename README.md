# emby-cli

A command-line interface for [Emby](https://emby.media) media servers.

## Installation

Requires Rust 1.85+.

```sh
cargo install --path .
```

## Configuration

Create a config file at `~/.config/emby-api.json`:

```sh
jq --null-input \
  --arg api_key "your-api-key" \
  --arg api_url "http://emby.local:8096" \
  '$ARGS.named' > ~/.config/emby-api.json

chmod 600 ~/.config/emby-api.json
```

The config path can be overridden with `EMBY_CONFIG`, or you can set `EMBY_API_KEY` and `EMBY_API_URL` environment variables directly.

## Usage

```
emby <command>
```

| Command | Description |
|---|---|
| `playing` | Show what's currently playing |
| `latest` | Show recently added media |
| `next-up` | Show next episodes to watch |
| `upcoming` | Show upcoming TV episodes |
| `search <query>` | Search the library |
| `scan` | Trigger library scans |
| `libraries` | List libraries |
| `users` | List users |
| `devices` | List devices |
| `activity` | Show recent activity log |
| `tasks` | List and run scheduled tasks |
| `system` | Show system information |
| `restart` | Restart Emby |
| `find-server` | Find Emby servers on the local network |

Run `emby <command> --help` for command-specific options.

## License

MIT
