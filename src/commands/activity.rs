use crate::cli::ActivityArgs;
use crate::client::Client;
use crate::config::Config;
use crate::emby::ticks;
use crate::emby::types::ActivityLogResponse;
use crate::error::Result;
use crate::format::table;

pub fn run(args: &ActivityArgs) -> Result<()> {
    let config = Config::load()?;
    let client = Client::new(&config);
    let limit = args.limit.to_string();
    let response: ActivityLogResponse = client.get_with_query(
        "/System/ActivityLog/Entries",
        &[("Limit", limit.as_str())],
    )?;

    let entries = response.items.unwrap_or_default();

    if entries.is_empty() {
        println!("No activity found");
        return Ok(());
    }

    let rows: Vec<Vec<String>> = entries
        .iter()
        .map(|e| {
            let date = e
                .date
                .as_deref()
                .map_or_else(String::new, ticks::format_premiere_date);
            let severity = e.severity.as_deref().unwrap_or("").to_string();
            let name = e.name.as_deref().unwrap_or("").to_string();
            let overview = e
                .short_overview
                .as_deref()
                .or(e.overview.as_deref())
                .unwrap_or("")
                .to_string();

            vec![date, severity, name, overview]
        })
        .collect();

    println!(
        "{}",
        table::build_table(&["Date", "Severity", "Name", "Overview"], rows)
    );

    Ok(())
}
