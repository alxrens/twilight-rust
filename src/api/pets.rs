use std::sync::Arc;

use serde_json::Value;

use crate::utils::connection::NetConn;



pub async fn get_animal_image(animal_name : &str, client : &Arc<NetConn>) -> Result<String, anyhow::Error> {
    let conn = &client.conn;
    if animal_name == "cat" {
        let data = conn.get("https://api.thecatapi.com/v1/images/search").send().await?;
        let json : Value   = data.json().await?;
        Ok(json[0]["url"].as_str().unwrap().to_string())
    } else if animal_name == "fox" {
        let data = conn.get("https://randomfox.ca/floof/").send().await?;
        let json : Value   = data.json().await?;
        Ok(json["image"].as_str().unwrap().to_string())
    } else if animal_name == "shibes" {
        let data = conn.get("https://dog.ceo/api/breed/shiba/images /random/1").send().await?;
        let json :Value = data.json().await?;
        Ok(json["message"][0].as_str().unwrap().to_string())
    } else if animal_name == "male_horse" {
        Ok(String::from("https://i.pinimg.com/564x/fc/e2/74/fce2740d6f3fe82e8d934d1d67169a4b.jpg"))
    } else {
        Err(anyhow::anyhow!("Invalid animal name"))
    }
}