use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Tag {
    pub public_id: Arc<str>,
    pub version: usize,
    pub format: Arc<str>,
    pub width: u32,
    pub height: u32,
    #[serde(rename = "type")]
    pub resource_type: Arc<str>,
    pub created_at: Arc<str>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TagList {
    pub resources: Vec<Tag>,
    pub updated_at: Arc<str>,
}

/// Loads a list of all images with a given tag
pub async fn get_tags(cloud_name: Arc<str>, tag_name: Arc<str>) -> Result<TagList> {
    let url = format!(
        "https://res.cloudinary.com/{}/image/list/{}.json",
        cloud_name, tag_name
    );
    let response = reqwest::get(&url)
        .await
        .context(format!("load tag {}", tag_name))?;
    let text = response
        .text()
        .await
        .context("parsing responce into text")?;
    let json = serde_json::from_str(&text).context(format!("parsing into json:\n{}", text))?;
    Ok(json)
}
