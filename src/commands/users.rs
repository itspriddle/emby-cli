use crate::client::Client;
use crate::config::Config;
use crate::emby::types::User;
use crate::error::Result;
use crate::format::table;

pub fn run() -> Result<()> {
    let config = Config::load()?;
    let client = Client::new(&config);
    let users: Vec<User> = client.get("/Users")?;

    if users.is_empty() {
        println!("No users found");
        return Ok(());
    }

    let rows: Vec<Vec<String>> = users
        .iter()
        .map(|u| {
            vec![
                u.name.as_deref().unwrap_or("").to_string(),
                u.id.as_deref().unwrap_or("").to_string(),
                u.policy
                    .as_ref()
                    .and_then(|p| p.is_administrator)
                    .unwrap_or(false)
                    .to_string(),
            ]
        })
        .collect();

    println!("{}", table::build_table(&["Name", "ID", "Admin"], rows));

    Ok(())
}
