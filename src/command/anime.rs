use poise::serenity_prelude::{self as serenity, Colour};

use crate::api::waifu::WaifuQuote;



use crate::Context;



#[poise::command(slash_command)]
pub async fn aniquote(
    ctx : Context<'_>,
) -> Result<(), anyhow::Error>{
    let app_data = ctx.data();
    let quote = WaifuQuote::get_quote(&app_data.netconn).await?;
    let create_reply = poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .color(Colour::from_rgb(46, 81, 162))
            .title("Anime Quote")
            .description(&quote.quote)
            .field("Anime", &quote.anime, true)
            .field("Character", &quote.author, true)
    );
    ctx.send(create_reply).await?;
    Ok(())
}