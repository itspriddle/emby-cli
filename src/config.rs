use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use crate::error::{Error, Result};

#[derive(Debug)]
pub struct Config {
    pub api_url: String,
    pub api_key: String,
}

impl Config {
    pub fn load() -> Result<Self> {
        if let (Ok(api_key), Ok(api_url)) = (env::var("EMBY_API_KEY"), env::var("EMBY_API_URL")) {
            if !api_key.is_empty() && !api_url.is_empty() {
                return Ok(Self { api_url, api_key });
            }
        }

        let path = config_path();
        let contents = fs::read_to_string(&path).map_err(|_| {
            Error::Config(format!(
                "Config '{}' doesn't exist\n{}",
                path.display(),
                configure_help(&path)
            ))
        })?;

        let json: serde_json::Value = serde_json::from_str(&contents)
            .map_err(|e| Error::Config(format!("Failed to parse '{}': {e}", path.display())))?;

        let api_url = json
            .get("api_url")
            .and_then(|v| v.as_str())
            .filter(|s| !s.is_empty())
            .map(String::from)
            .ok_or_else(|| {
                Error::Config(format!(
                    "Must set 'api_url' in {}\n{}",
                    path.display(),
                    configure_help(&path)
                ))
            })?;

        let api_key = json
            .get("api_key")
            .and_then(|v| v.as_str())
            .filter(|s| !s.is_empty())
            .map(String::from)
            .ok_or_else(|| {
                Error::Config(format!(
                    "Must set 'api_key' in {}\n{}",
                    path.display(),
                    configure_help(&path)
                ))
            })?;

        Ok(Self { api_url, api_key })
    }
}

fn config_path() -> PathBuf {
    if let Ok(path) = env::var("EMBY_CONFIG") {
        return PathBuf::from(path);
    }

    let config_home = env::var("XDG_CONFIG_HOME").unwrap_or_else(|_| {
        let home = env::var("HOME").unwrap_or_else(|_| String::from("~"));
        format!("{home}/.config")
    });

    PathBuf::from(config_home).join("emby-api.json")
}

fn configure_help(path: &Path) -> String {
    format!(
        r#"
Create `{}' with:

  jq --null-input \
    --arg api_key "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa" \
    --arg api_url "http://emby.local:8096" \
    '$ARGS.named' > "{}"

  chmod 600 "{}""#,
        path.display(),
        path.display(),
        path.display()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_path_defaults_to_xdg() {
        // config_path() should return a path ending in emby-api.json
        let path = config_path();
        assert!(path.to_string_lossy().ends_with("emby-api.json"));
    }

    #[test]
    fn configure_help_includes_instructions() {
        let path = PathBuf::from("/tmp/test-config.json");
        let help = configure_help(&path);
        assert!(help.contains("jq --null-input"));
        assert!(help.contains("/tmp/test-config.json"));
    }
}
