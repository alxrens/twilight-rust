use crate::api::waifu::Uwu;

use crate::Context;



#[poise::command(slash_command)]
pub async fn uwuify(
    ctx : Context<'_>,
    #[description = "Select Your uwu type"]
    #[choices("uwu", "owo", "uvu" )]
    uwutype : &str,
    #[description = "Input text"]
    base_text : String
) -> Result<(), anyhow::Error> {

    let app_data = ctx.data();
    let conn = &app_data.netconn;
    if uwutype == "uwu" {
        let result = Uwu::uwu(base_text, &conn).await?;
        ctx.say(result.text).await?;
    } else if uwutype == "owo" {
        ctx.say(Uwu::owo(base_text, &conn).await?.text).await?;
    } else if uwutype == "uvu" {
        ctx.say(Uwu::uvu(base_text, &conn).await?.text).await?;
    } else {
        ctx.say("invalid uwu type").await?;
    }

    Ok(())
}