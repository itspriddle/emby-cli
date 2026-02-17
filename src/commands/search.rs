use crate::cli::SearchArgs;
use crate::client::Client;
use crate::config::Config;
use crate::emby::ticks;
use crate::emby::types::{BaseItemDto, QueryResultBaseItemDto};
use crate::error::Result;
use crate::format::table;

pub fn run(args: &SearchArgs) -> Result<()> {
    let config = Config::load()?;
    let client = Client::new(&config);
    let limit = args.limit.to_string();
    let response: QueryResultBaseItemDto = client.get_with_query(
        "/Items",
        &[
            ("SearchTerm", args.query.as_str()),
            ("Recursive", "true"),
            ("Limit", &limit),
            ("Fields", "ProductionYear,PremiereDate,SeriesName"),
            ("ExcludeItemTypes", "Folder,UserView,CollectionFolder"),
        ],
    )?;

    let items = response.items.unwrap_or_default();

    if items.is_empty() {
        println!("No results found");
        return Ok(());
    }

    let rows: Vec<Vec<String>> = items
        .iter()
        .map(|item| {
            let media_type = item.media_type.as_deref().unwrap_or("").to_string();
            let name = format_search_name(item);
            let year = item
                .production_year
                .map_or_else(String::new, |y| y.to_string());
            let id = item.id.as_deref().unwrap_or("").to_string();

            vec![media_type, name, year, id]
        })
        .collect();

    println!(
        "{}",
        table::build_table(&["Type", "Name", "Year", "ID"], rows)
    );

    Ok(())
}

pub(crate) fn format_search_name(item: &BaseItemDto) -> String {
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
            id: None,
            name: Some("Test Name".to_string()),
            media_type: Some(media_type.to_string()),
            series_name: None,
            production_year: None,
            premiere_date: None,
            date_created: None,
            index_number: None,
            parent_index_number: None,
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
        let mut h = item("Episode");
        h.series_name = Some("Friends".to_string());
        h.parent_index_number = Some(1);
        h.index_number = Some(2);
        assert_eq!(format_search_name(&h), "Friends - S01E02 - Test Name");
    }

    #[test]
    fn format_audio() {
        let mut h = item("Audio");
        h.album_artist = Some("Queen".to_string());
        assert_eq!(format_search_name(&h), "Queen - Test Name");
    }

    #[test]
    fn format_movie() {
        let h = item("Movie");
        assert_eq!(format_search_name(&h), "Test Name");
    }
}
