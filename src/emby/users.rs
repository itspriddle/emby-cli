use crate::client::Client;
use crate::emby::types::User;
use crate::error::{Error, Result};

/// Resolve a user ID from an optional user name.
/// If a name is given, find by case-insensitive match.
/// If None, return the first admin user's ID.
pub fn resolve_user_id(client: &Client, user_name: Option<&str>) -> Result<String> {
    let users: Vec<User> = client.get("/Users")?;

    if let Some(name) = user_name {
        users
            .iter()
            .find(|u| {
                u.name
                    .as_deref()
                    .is_some_and(|n| n.eq_ignore_ascii_case(name))
            })
            .and_then(|u| u.id.clone())
            .ok_or_else(|| Error::Config(format!("User '{name}' not found")))
    } else {
        users
            .iter()
            .find(|u| {
                u.policy
                    .as_ref()
                    .and_then(|p| p.is_administrator)
                    .unwrap_or(false)
            })
            .and_then(|u| u.id.clone())
            .ok_or_else(|| Error::Config("No admin user found".to_string()))
    }
}
