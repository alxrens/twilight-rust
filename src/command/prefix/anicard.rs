use crate::Context;

use crate::command::prefix::anicard_command as command;

#[poise::command(prefix_command)]
pub async fn anicard(ctx: Context<'_>, x: Option<String>) -> Result<(), anyhow::Error> {
    if x.is_none() {
        ctx.say("please provide a sub command. if you don't know how to use, type $anicard help").await?;
        return Ok(())
    }

    let app_data = ctx.data();

    if let Some(subcommand) = x {
         match subcommand.as_str() {
             "register" => {
                let user = if ctx.author().global_name.is_some() {
                    ctx.author().global_name.as_ref().unwrap()
                } else {
                    let  user = &ctx.author().name;
                    let (first, _last) = user.split_at(3);
                    first
                };
             let response = command::register::register(&ctx.author().id.to_string(),&user, &app_data.db_conn).await?;
             ctx.say(response).await?;
             }

             _ => {
                 ctx.say("invalid subcommand").await?;
             }
         }
}



    Ok(())
}