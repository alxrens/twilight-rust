use crate::Context;

use crate::command::prefix::anicard_command as command;

#[poise::command(prefix_command)]
pub async fn anicard(ctx: Context<'_>, x: Option<String>) -> Result<(), anyhow::Error> {
    if x.is_none() {
        ctx.say("please provide a sub command. if you don't know how to use, type $anicard help").await?;
        return Ok(())
    }

    if let Some(subcommand) = x {
         match subcommand.as_str() {
             "register" => {
             let response = command::register::register(&ctx.author().name).await?;
             ctx.say(response).await?;
             }
             _ => {
                 ctx.say("invalid subcommand").await?;
             }
         }
}



    Ok(())
}