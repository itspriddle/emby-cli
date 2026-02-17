use crate::cli::ScanArgs;
use crate::client::Client;
use crate::config::Config;
use crate::emby::types::VirtualFolder;
use crate::error::{Error, Result};

pub fn run(args: &ScanArgs) -> Result<()> {
    let config = Config::load()?;
    let client = Client::new(&config);

    let recursive = !args.no_recursive;

    // Normalize library type names
    let target_types: Vec<String> = args
        .libraries
        .iter()
        .map(|s| match s.as_str() {
            "shows" | "tv" => "tvshows".to_string(),
            other => other.to_string(),
        })
        .collect();

    let is_all = target_types.iter().any(|t| t == "all");

    // Fetch virtual folders (libraries)
    let folders: Vec<VirtualFolder> = client.get("/Library/VirtualFolders")?;

    // Filter to supported collection types
    let supported = ["movies", "tvshows", "music"];
    let matching: Vec<&VirtualFolder> = folders
        .iter()
        .filter(|f| {
            f.collection_type
                .as_deref()
                .is_some_and(|ct| supported.contains(&ct))
        })
        .filter(|f| {
            is_all
                || f.collection_type
                    .as_deref()
                    .is_some_and(|ct| target_types.iter().any(|t| t == ct))
        })
        .collect();

    if matching.is_empty() {
        return Err(Error::Config("No libraries found".to_string()));
    }

    let body = serde_json::json!({
        "Recursive": recursive,
        "MetadataRefreshMode": args.metadata_refresh_mode,
        "ImageRefreshMode": args.image_refresh_mode,
        "ReplaceAllMetadata": args.replace_all_metadata,
        "ReplaceAllImages": args.replace_all_images,
    });

    for folder in &matching {
        let id = folder.item_id.as_deref().unwrap_or("");
        let collection_type = folder.collection_type.as_deref().unwrap_or("unknown");

        client.post(&format!("/Items/{id}/Refresh"), Some(&body))?;
        println!("Scanning for {collection_type} in library ID {id}");
    }

    Ok(())
}
