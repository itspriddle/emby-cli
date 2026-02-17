use assert_cmd::cargo_bin_cmd;
use predicates::str::contains;
use std::io::Write;
use tempfile::NamedTempFile;

#[test]
fn missing_config_shows_error() {
    cargo_bin_cmd!("emby")
        .arg("system")
        .env("EMBY_CONFIG", "/tmp/nonexistent-emby-cli-test.json")
        .env_remove("EMBY_API_KEY")
        .env_remove("EMBY_API_URL")
        .assert()
        .failure()
        .stderr(contains("doesn't exist"));
}

#[test]
fn missing_api_key_shows_error() {
    let mut file = NamedTempFile::new().unwrap();
    writeln!(file, r#"{{"api_url": "http://localhost:8096"}}"#).unwrap();

    cargo_bin_cmd!("emby")
        .arg("system")
        .env("EMBY_CONFIG", file.path().to_str().unwrap())
        .env_remove("EMBY_API_KEY")
        .env_remove("EMBY_API_URL")
        .assert()
        .failure()
        .stderr(contains("api_key"));
}

#[test]
fn missing_api_url_shows_error() {
    let mut file = NamedTempFile::new().unwrap();
    writeln!(file, r#"{{"api_key": "test"}}"#).unwrap();

    cargo_bin_cmd!("emby")
        .arg("system")
        .env("EMBY_CONFIG", file.path().to_str().unwrap())
        .env_remove("EMBY_API_KEY")
        .env_remove("EMBY_API_URL")
        .assert()
        .failure()
        .stderr(contains("api_url"));
}

#[test]
fn help_flag_shows_usage() {
    cargo_bin_cmd!("emby")
        .arg("--help")
        .assert()
        .success()
        .stdout(contains("CLI for some random stuff in Emby"));
}

#[test]
fn no_args_shows_help() {
    cargo_bin_cmd!("emby")
        .assert()
        .failure()
        .stderr(contains("Usage"));
}
