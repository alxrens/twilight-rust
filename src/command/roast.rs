use crate::Context;



// TODO: add capability to roast someone using @mention if target is not specified, bot will roast the creator instead.
#[poise::command(slash_command)]
pub async fn roast(
    ctx : Context<'_>,
    #[description = "Who you gonna roast?"]
    target : Option<String>
) -> Result<(), anyhow::Error> {


        ctx.say(target.unwrap_or("You".to_string())).await?;

        Ok(())
}