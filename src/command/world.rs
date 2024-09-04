

use poise::{serenity_prelude::CreateEmbed, CreateReply};

use crate::Context;


#[poise::command(slash_command)]
pub async fn hello(
    ctx : Context<'_>,
    #[description = "hello"]
    cmd : String
) ->Result<(), anyhow::Error> {
    ctx.say(cmd).await?;
    Ok(())
}

#[poise::command(slash_command)]
pub async fn embeed(ctx : Context<'_>) -> Result<(), anyhow::Error> {
    let embeds = CreateEmbed::new().title("hello").description("world");

    let eeembed = poise::CreateReply::embed(CreateReply::default(),embeds);


    ctx.send(eeembed).await?;
    Ok(())
}