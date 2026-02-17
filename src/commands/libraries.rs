use crate::client::Client;
use crate::config::Config;
use crate::emby::types::VirtualFolder;
use crate::error::Result;
use crate::format::table;

pub fn run() -> Result<()> {
    let config = Config::load()?;
    let client = Client::new(&config);
    let libraries: Vec<VirtualFolder> = client.get("/Library/VirtualFolders")?;

    if libraries.is_empty() {
        println!("No libraries found");
        return Ok(());
    }

    let rows: Vec<Vec<String>> = libraries
        .iter()
        .map(|l| {
            vec![
                l.name.as_deref().unwrap_or("").to_string(),
                l.collection_type.as_deref().unwrap_or("").to_string(),
                l.item_id.as_deref().unwrap_or("").to_string(),
            ]
        })
        .collect();

    println!("{}", table::build_table(&["Name", "Type", "ID"], rows));

    Ok(())
}
