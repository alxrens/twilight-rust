use crate::Context;
use crate::utils::pak::get_pak;

#[poise::command(slash_command)]
pub async fn randomwords(ctx: Context<'_>) -> Result<(), anyhow::Error> {
    let word = get_pak().await;
    ctx.say(word).await?;
    Ok(())
}







