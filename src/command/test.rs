use poise::serenity_prelude::{self as serenity, Attachment, Colour};
use tokio::{fs::File, io::AsyncWriteExt};
use crate::Context;
use base64; 








#[poise::command(prefix_command)]
pub async fn test(ctx : Context<'_>,
                 attach : Option<Attachment>,
                 #[rest]
                 #[description = "input your message"]
                 message: String
) -> Result<(), anyhow::Error> {
    log::info!("message: {}", message);
    if let Some(attachment) = attach {
        let bit = attachment.download().await?;
        let filepath = format!("./tmp/{}", attachment.filename);

        let mut file = File::create(filepath).await?;

        file.write_all(&bit).await?;
        // log::info!("{:?}", attachment)
        ctx.say("attachment received").await?;
        return Ok(());
    }
    ctx.say("message received").await?;
    Ok(())
}