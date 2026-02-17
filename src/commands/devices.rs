use crate::client::Client;
use crate::config::Config;
use crate::emby::types::DevicesResponse;
use crate::error::Result;
use crate::format::table;

pub fn run() -> Result<()> {
    let config = Config::load()?;
    let client = Client::new(&config);
    let response: DevicesResponse = client.get("/Devices")?;

    let devices = response.items.unwrap_or_default();

    if devices.is_empty() {
        println!("No devices found");
        return Ok(());
    }

    let rows: Vec<Vec<String>> = devices
        .iter()
        .map(|d| {
            let app = format!(
                "{} ({})",
                d.app_name.as_deref().unwrap_or(""),
                d.app_version.as_deref().unwrap_or("")
            );
            vec![
                d.name.as_deref().unwrap_or("").to_string(),
                d.ip_address.as_deref().unwrap_or("").to_string(),
                d.last_user_name.as_deref().unwrap_or("").to_string(),
                app,
                d.id.as_deref().unwrap_or("").to_string(),
            ]
        })
        .collect();

    println!(
        "{}",
        table::build_table(
            &["Name", "IP Address", "Last User", "App (Version)", "ID"],
            rows
        )
    );

    Ok(())
}
