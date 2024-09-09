use crate::Context;



#[poise::command(slash_command)]
pub async fn roast(
    ctx : Context<'_>,
    #[description = "Who you gonna roast?"]
    target : Option<String>
) -> Result<(), anyhow::Error> {
    let app_conn = &ctx.data().netconn.conn;

    let insult = app_conn.get("https://evilinsult.com/generate_insult.php?lang=en&type=plaintext").send().await?.text().await?;

    if target.is_some() {
        ctx.say(format!("hey {}, {}", target.unwrap(), insult)).await?;
    }else {
        ctx.say(format!("hey {}, {}", ctx.author(), insult)).await?;
    }
        Ok(())
}