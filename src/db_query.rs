use anyhow::Result;
use serde::{Deserialize, Serialize};
use reqwest::Client;

#[derive(Debug, Serialize, Deserialize)]
pub struct AboutMe {
    pub entry_title: String,
    pub content: String,
}

pub async fn fetch_about_me() -> Result<Vec<AboutMe>> {
    let client = Client::new();
    let res = client
        .get("http://127.0.0.1:8000/records")
        .send()
        .await?
        .json::<Vec<AboutMe>>()
        .await?;

    Ok(res)
}
