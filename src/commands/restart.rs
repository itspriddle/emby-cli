use crate::client::Client;
use crate::config::Config;
use crate::error::Result;

pub fn run() -> Result<()> {
    let config = Config::load()?;
    let client = Client::new(&config);
    client.post("/System/Restart", None)?;
    Ok(())
}
