use serde::de::DeserializeOwned;

use crate::config::Config;
use crate::error::Result;

pub struct Client {
    agent: ureq::Agent,
    base_url: String,
    api_key: String,
}

impl Client {
    pub fn new(config: &Config) -> Self {
        let agent = ureq::Agent::new_with_config(ureq::config::Config::builder().build());
        Self {
            agent,
            base_url: config.api_url.trim_end_matches('/').to_string(),
            api_key: config.api_key.clone(),
        }
    }

    pub fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T> {
        let url = self.url(path);
        let response = self
            .agent
            .get(&url)
            .header("X-Emby-Token", &self.api_key)
            .header("Accept", "*/*")
            .call()?;

        let body: T = response.into_body().read_json()?;
        Ok(body)
    }

    pub fn get_with_query<T: DeserializeOwned>(
        &self,
        path: &str,
        query: &[(&str, &str)],
    ) -> Result<T> {
        let url = self.url(path);
        let mut request = self
            .agent
            .get(&url)
            .header("X-Emby-Token", &self.api_key)
            .header("Accept", "*/*");

        for (key, value) in query {
            request = request.query(key, value);
        }

        let response = request.call()?;
        let body: T = response.into_body().read_json()?;
        Ok(body)
    }

    pub fn post(&self, path: &str, body: Option<&serde_json::Value>) -> Result<()> {
        let url = self.url(path);
        let request = self
            .agent
            .post(&url)
            .header("X-Emby-Token", &self.api_key)
            .header("Accept", "*/*")
            .header("Content-Type", "application/json");

        if let Some(json) = body {
            request.send_json(json)?;
        } else {
            request.send_empty()?;
        }

        Ok(())
    }

    /// Returns the base API URL, e.g., `http://emby.local:8096`
    pub fn api_url(&self) -> &str {
        &self.base_url
    }

    fn url(&self, path: &str) -> String {
        let path = path.strip_prefix('/').unwrap_or(path);
        format!("{}/emby/{path}", self.base_url)
    }
}
