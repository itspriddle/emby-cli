use crate::client::Client;
use crate::config::Config;
use crate::emby::types::SystemInfo;
use crate::error::Result;

pub fn run() -> Result<()> {
    let config = Config::load()?;
    let client = Client::new(&config);
    let info: SystemInfo = client.get("/System/Info")?;

    let version = info.version.as_deref().unwrap_or("Unknown");
    let server_name = info.server_name.as_deref().unwrap_or("Unknown");
    let os = info
        .operating_system_display_name
        .as_deref()
        .unwrap_or("Unknown");
    let update = if info.has_update_available.unwrap_or(false) {
        "Yes"
    } else {
        "No"
    };

    println!("Emby Version:     {version}");
    println!("Emby URL:         {}", client.api_url());
    println!("Server Name:      {server_name}");
    println!("Operating System: {os}");
    println!("Update Available: {update}");

    Ok(())
}
