

use serde_json::Value;

use crate::Context;

#[poise::command(slash_command)]
pub async fn waifu(
    ctx : Context<'_>,
    #[description = "Pick your poison"]
    #[choices("maid", "waifu", "oppai", "selfies", "uniform", "ass", "hentai", "milf", "oral", "paizuri", "ecchi", "ero")]
    anime_tag : &str
    ) -> Result<(), anyhow::Error> {
    let corn_tag = vec!["ass", "hentai", "milf", "oral", "paizuri", "ecchi", "ero"];
    let channel = ctx.guild_channel().await;
        if corn_tag.contains(&anime_tag) {
            if let Some(channel) = channel {
                if !channel.nsfw {
                    ctx.say("why are you executing this command on a non nsfw channel? begone you degenerate >:(").await?;
                    return Ok(());
                }
            }
        }

    let app_conn = &ctx.data().netconn;
    let img_data = app_conn.conn.get(format!("https://api.waifu.im/search?included_tags={}", anime_tag)).send().await?.text().await?;
    let json_data: Value = serde_json::from_str(&img_data)?;
    let img_url = json_data["images"][0]["url"].as_str().unwrap();
    ctx.say(img_url).await?;
    Ok(())
}