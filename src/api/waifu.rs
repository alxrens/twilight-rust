use std::sync::Arc;

use serde::Deserialize;

use crate::utils::connection::NetConn;

static URL : &str = "https://waifu.it/api/v4";


#[derive(Deserialize)]
pub struct WaifuQuote {
    pub _id : u64,
    pub quote : String,
    pub anime : String,
    pub author : String
}


impl WaifuQuote {
    pub async fn get_quote(client : &Arc<NetConn>) -> Result<WaifuQuote, anyhow::Error> {
        let data = client.conn.get(format!("{}/quote", URL)).header("Authorization", std::env::var("WAIFUIT_TOKEN").expect("WAIFUIT_TOKEN not set")).send().await?.text().await?;
        let json : WaifuQuote   = serde_json::from_str(&data)?;
        Ok(json)
    }
}


//this one bellow is for the lolz
#[derive(Deserialize)]
pub struct Uwu {
    pub text : String,
}


impl Uwu {
    pub async fn uwu(base_text : String, client : &Arc<NetConn>) -> Result<Uwu, anyhow::Error> {
        let data = client.conn.get(format!("{}/uwuify?text={}", URL, base_text))
        .header("Authorization", std::env::var("WAIFUIT_TOKEN").expect("WAIFUIT_TOKEN not set"))
        .send().await?;
        let json : Uwu   = data.json().await?;
        Ok(json)
    }

    pub async fn owo(base_text : String, client : &Arc<NetConn>) -> Result<Uwu, anyhow::Error> {
        let data = client.conn.get(format!("{}/owoify?text={}", URL, base_text))
        .header("Authorization", std::env::var("WAIFUIT_TOKEN").expect("WAIFUIT_TOKEN not set"))
        .send().await?;
        let json : Uwu   = data.json().await?;
        Ok(json)
    }

    pub async fn uvu(base_text : String, client : &Arc<NetConn>) -> Result<Uwu, anyhow::Error> {
        let data = client.conn.get(format!("{}/uvuify?text={}", URL, base_text))
        .header("Authorization", std::env::var("WAIFUIT_TOKEN").expect("WAIFUIT_TOKEN not set"))
        .send().await?;
        let json : Uwu   = data.json().await?;
        Ok(json)
    }

}
