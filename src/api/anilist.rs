use std::sync::Arc;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::utils::connection::NetConn;


#[derive(Debug, Serialize, Deserialize)]
pub struct Anilist {
    pub name : String,
    pub img_url : Option<String>,
    pub episodes : u32,
    pub description : String,
    pub release_date : String,
    pub score : f32
}


pub async fn get_anilist(anime_name : &str, nsfw : Option<bool>, client : &Arc<NetConn>) -> Result<Vec<Anilist>, anyhow::Error> {
    let mut url = format!("https://api.jikan.moe/v4/anime?q={}&limit=10", anime_name);
    if nsfw.is_some() || nsfw == Some(true) {
        url = format!("{}&sfw=true", url);
    }
    // let data = reqwest::get(url).await?;

    let data = client.conn.get(url).send().await?;
    let json : Value   = data.json().await?;

    let mut anilist = Vec::new();

    if let Some(anime_data) = json["data"].as_array() {
        for anime in anime_data {
            let name = anime["title"].as_str().unwrap_or("Unknown").to_string();
            let img_url = anime["images"]["jpg"]["image_url"].as_str().map(|s| s.to_string());
            let episodes = anime["episodes"].as_u64().unwrap_or(0) as u32;
            let description = anime["synopsis"].as_str().unwrap_or("No description available").to_string();
            let release_date = anime["aired"]["string"].as_str().unwrap_or("Unknown").to_string();
            let score = anime["score"].as_f64().unwrap_or(0.0) as f32;
            let anime_item = Anilist {
                name,
                img_url,
                episodes,
                description,
                release_date,
                score,
            };
            anilist.push(anime_item);
        }
    }
    Ok(anilist)
}