


use crate::{api::chat::chat, Context};



#[poise::command(prefix_command)]
pub async fn magi(
    ctx : Context<'_>,
    #[rest]
    #[description = "input your message"]
    message : String
) -> Result<(), anyhow::Error> {
    // ctx.say(message).await?;
    if message.len() < 10 {
        ctx.say("message is too short").await?;
        return Ok(())
    } else {
        let response = chat(&message, &ctx.data().netconn).await?;
        // log::info!("response: {}", response);
        if response.len() > 2000 {
            let response_part1 = &response[..2000];
            let response_part2 = &response[2000..];
            ctx.reply(response_part1).await?;
            ctx.say(response_part2).await?;
            Ok(())
        }else {
            ctx.reply(response).await?;
            Ok(())
        }
        // ctx.reply(response).await?;
        // Ok(())
    }
}