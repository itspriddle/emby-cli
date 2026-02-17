use crate::cli::SearchArgs;
use crate::client::Client;
use crate::config::Config;
use crate::emby::ticks;
use crate::emby::types::SearchHintResponse;
use crate::error::Result;
use crate::format::table;

pub fn run(args: &SearchArgs) -> Result<()> {
    let config = Config::load()?;
    let client = Client::new(&config);
    let limit = args.limit.to_string();
    let response: SearchHintResponse = client.get_with_query(
        "/Search/Hints",
        &[("SearchTerm", args.query.as_str()), ("Limit", &limit)],
    )?;

    let hints = response.search_hints.unwrap_or_default();

    if hints.is_empty() {
        println!("No results found");
        return Ok(());
    }

    let rows: Vec<Vec<String>> = hints
        .iter()
        .map(|h| {
            let media_type = h.media_type.as_deref().unwrap_or("").to_string();
            let name = format_search_name(h);
            let year = h
                .production_year
                .map_or_else(String::new, |y| y.to_string());
            let id = h.item_id.map_or_else(String::new, |id| id.to_string());

            vec![media_type, name, year, id]
        })
        .collect();

    println!(
        "{}",
        table::build_table(&["Type", "Name", "Year", "ID"], rows)
    );

    Ok(())
}

pub(crate) fn format_search_name(hint: &crate::emby::types::SearchHint) -> String {
    let name = hint.name.as_deref().unwrap_or("");
    let media_type = hint.media_type.as_deref().unwrap_or("");

    if media_type == "Episode" {
        let series = hint.series.as_deref().unwrap_or("");
        let code = ticks::format_episode_code(hint.parent_index_number, hint.index_number);
        format!("{series} - {code} - {name}")
    } else if media_type == "Audio" {
        let artist = hint.album_artist.as_deref().unwrap_or("");
        format!("{artist} - {name}")
    } else {
        name.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::emby::types::SearchHint;

    fn hint(media_type: &str) -> SearchHint {
        SearchHint {
            item_id: None,
            name: Some("Test Name".to_string()),
            media_type: Some(media_type.to_string()),
            production_year: None,
            series: None,
            album: None,
            album_artist: None,
            index_number: None,
            parent_index_number: None,
            run_time_ticks: None,
        }
    }

    #[test]
    fn format_episode() {
        let mut h = hint("Episode");
        h.series = Some("Friends".to_string());
        h.parent_index_number = Some(1);
        h.index_number = Some(2);
        assert_eq!(format_search_name(&h), "Friends - S01E02 - Test Name");
    }

    #[test]
    fn format_audio() {
        let mut h = hint("Audio");
        h.album_artist = Some("Queen".to_string());
        assert_eq!(format_search_name(&h), "Queen - Test Name");
    }

    #[test]
    fn format_movie() {
        let h = hint("Movie");
        assert_eq!(format_search_name(&h), "Test Name");
    }
}
