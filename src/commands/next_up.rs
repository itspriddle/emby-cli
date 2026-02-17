use crate::cli::NextUpArgs;
use crate::client::Client;
use crate::config::Config;
use crate::emby::{ticks, users};
use crate::emby::types::QueryResultBaseItemDto;
use crate::error::Result;
use crate::format::table;

pub fn run(args: &NextUpArgs) -> Result<()> {
    let config = Config::load()?;
    let client = Client::new(&config);
    let user_id = users::resolve_user_id(&client, args.user.as_deref())?;
    let limit = args.limit.to_string();

    let response: QueryResultBaseItemDto = client.get_with_query(
        "/Shows/NextUp",
        &[
            ("UserId", user_id.as_str()),
            ("Limit", &limit),
            ("Fields", "Overview"),
        ],
    )?;

    let items = response.items.unwrap_or_default();

    if items.is_empty() {
        println!("No next up episodes");
        return Ok(());
    }

    let rows: Vec<Vec<String>> = items
        .iter()
        .map(|item| {
            let series = item.series_name.as_deref().unwrap_or("").to_string();
            let code = ticks::format_episode_code(item.parent_index_number, item.index_number);
            let episode_name = item.name.as_deref().unwrap_or("");
            let episode = format!("{code} - {episode_name}");
            let air_date = item
                .premiere_date
                .as_deref()
                .map_or_else(String::new, ticks::format_premiere_date);

            vec![series, episode, air_date]
        })
        .collect();

    println!(
        "{}",
        table::build_table(&["Series", "Episode", "Air Date"], rows)
    );

    Ok(())
}
