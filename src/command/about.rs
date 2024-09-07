


use poise::serenity_prelude::{self, Colour, CreateEmbedAuthor};
use crate::Context;


#[poise::command(prefix_command, slash_command)]
pub async fn about(ctx : Context<'_>) -> Result<(), anyhow::Error> {
    let reply = poise::CreateReply::default().embed(
        serenity_prelude::CreateEmbed::new()
            .color(Colour::from_rgb(255, 255, 255))
            .author(CreateEmbedAuthor::new("Naoko Akagi").icon_url("https://pbs.twimg.com/profile_images/870999773/Naoko02rounded_400x400.png"))
            .title("Arch Twilight")
            .image("https://www.shareicon.net/download/2015/09/18/102854_archlinux_512x512.png")
            .description(r#"The S.C. Magi System (マギ) are a trio of supercomputers designed by Dr. Naoko Akagi during her research into bio-computers while at Gehirn.
            The Magi's 7th generation organic computers were implanted with three differing aspects of Dr. Naoko Akagi's personality using the Personality Transplant OS (Operating System),
            being her persona as a woman (Casper-Magi 3), her persona as a mother (Balthasar-Magi 2), and her persona as a scientist (Melchior-Magi 1).
            Upon completion of the Magi and the subsequent death of Naoko, the Magi were transferred into Alxren's possession under the name of Twillight(タソガレ) to prevent the ownership of the computers to NERV.
            The computers were then improved and then renamed into Arch Twillight.
            "#)
            .field("Author", "Naoko Akagi", true)
            .field("Current Maintainer", "Alxren & Han", true)
    );


    ctx.send(reply).await?;

    Ok(())
}