use crate::cli::LatestArgs;
use crate::client::Client;
use crate::config::Config;
use crate::emby::types::BaseItemDto;
use crate::emby::{ticks, users};
use crate::error::{Error, Result};
use crate::format::table;

pub fn run(args: &LatestArgs) -> Result<()> {
    let config = Config::load()?;
    let client = Client::new(&config);
    let user_id = users::resolve_user_id(&client, args.user.as_deref())?;
    let limit = args.limit.to_string();

    let mut query = vec![
        ("Limit", limit.as_str()),
        ("Fields", "DateCreated,Overview"),
        ("GroupItems", "true"),
    ];

    let include_types = match args.r#type.as_deref() {
        Some("movies") => Some("Movie"),
        Some("shows") => Some("Series"),
        Some("music") => Some("Audio"),
        Some(other) => {
            return Err(Error::Config(format!(
                "Unknown type '{other}'. Use: movies, shows, music"
            )));
        }
        None => None,
    };

    if let Some(types) = include_types {
        query.push(("IncludeItemTypes", types));
    }

    let items: Vec<BaseItemDto> =
        client.get_with_query(&format!("/Users/{user_id}/Items/Latest"), &query)?;

    if items.is_empty() {
        println!("No recently added items");
        return Ok(());
    }

    let rows: Vec<Vec<String>> = items
        .iter()
        .map(|item| {
            let media_type = item.media_type.as_deref().unwrap_or("").to_string();
            let name = format_latest_name(item);
            let year = item
                .production_year
                .map_or_else(String::new, |y| y.to_string());

            vec![media_type, name, year]
        })
        .collect();

    println!("{}", table::build_table(&["Type", "Name", "Year"], rows));

    Ok(())
}

pub(crate) fn format_latest_name(item: &BaseItemDto) -> String {
    let name = item.name.as_deref().unwrap_or("");
    let media_type = item.media_type.as_deref().unwrap_or("");

    if media_type == "Episode" {
        let series = item.series_name.as_deref().unwrap_or("");
        let code = ticks::format_episode_code(item.parent_index_number, item.index_number);
        format!("{series} - {code} - {name}")
    } else if media_type == "Audio" {
        let artist = item.album_artist.as_deref().unwrap_or("");
        format!("{artist} - {name}")
    } else {
        name.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn item(media_type: &str) -> BaseItemDto {
        BaseItemDto {
            name: Some("Test Name".to_string()),
            id: None,
            media_type: Some(media_type.to_string()),
            series_name: None,
            index_number: None,
            parent_index_number: None,
            production_year: None,
            premiere_date: None,
            date_created: None,
            run_time_ticks: None,
            overview: None,
            container: None,
            official_rating: None,
            album: None,
            album_artist: None,
        }
    }

    #[test]
    fn format_episode() {
        let mut i = item("Episode");
        i.series_name = Some("Friends".to_string());
        i.parent_index_number = Some(5);
        i.index_number = Some(14);
        assert_eq!(format_latest_name(&i), "Friends - S05E14 - Test Name");
    }

    #[test]
    fn format_audio() {
        let mut i = item("Audio");
        i.album_artist = Some("Led Zeppelin".to_string());
        assert_eq!(format_latest_name(&i), "Led Zeppelin - Test Name");
    }

    #[test]
    fn format_movie() {
        let i = item("Movie");
        assert_eq!(format_latest_name(&i), "Test Name");
    }
}
