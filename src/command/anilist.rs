use poise::serenity_prelude::{self as serenity, Colour, ComponentInteractionCollector};

use crate::utils::button_builder::create_button;
use crate::Context;

use crate::api::anilist:: get_anilist;




#[poise::command(slash_command)]
pub async fn anilist(
    ctx: Context<'_>,
    #[description = "Input anime name"]
    anime_name : String,
    #[description = "Show NSFW content?"]
    nsfw : Option<bool>,
) -> Result<(), anyhow::Error> {
    let app_data = ctx.data();
    let channel= ctx.guild_channel().await;
    if nsfw.unwrap_or(false) {
        if channel.is_some() {
            if !channel.unwrap().nsfw {
                ctx.say("why are you executing this command on a non nsfw channel? begone you degenerate >:(").await?;
                return Ok(());
            }
        }
    }

    let anilist = get_anilist(&anime_name, nsfw, &app_data.netconn).await?;

    let anime_title = anilist
        .iter()
        .enumerate()
        .map(|(i, name) | format!("{}. {}.", i+1, name.name))
        .collect::<Vec<_>>()
        .join("\n");
    

    let action_row = create_button(anilist.len()).await?;



    let reply = poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .color(Colour::from_rgb(46, 81, 162))
            .title("Here's top 10 Matching Anime Based on your input.")
            .description(&anime_title)
    ).components(action_row);
    

    ctx.send(reply).await?;
    let mut message = None;

    while let Some(inter) = ComponentInteractionCollector::new(ctx).timeout(std::time::Duration::from_secs(60)).await
    {
        let mut msg = inter.message.clone();
        if let Ok(index) = inter.data.custom_id.parse::<i32>(){
            if index>= 0 && index < anilist.len() as i32  {
                let animedata = &anilist[index as usize - 1];
                
                let anime_image = &animedata.img_url.clone().unwrap_or("https://png.pngtree.com/png-vector/20221125/ourmid/pngtree-no-image-available-icon-flatvector-illustration-pic-design-profile-vector-png-image_40966566.jpg".to_string());
                
                let new_embed = serenity::CreateEmbed::new()
                    .color(Colour::from_rgb(46, 81, 162))
                    .title(&animedata.name)
                    .description(&animedata.description)
                    .field("Episodes", &animedata.episodes.to_string(), true)
                    .field("Release Date", &animedata.release_date, true)
                    .field("Score", &animedata.score.to_string(), true)
                    .image(anime_image);
                
                msg.edit(ctx, serenity::EditMessage::new().embed(new_embed)).await?;
                
                message = Some(msg);
                
                inter.create_response(ctx, serenity::CreateInteractionResponse::Acknowledge).await?;
            }

        } else if inter.data.custom_id == "BackToList" {
            let new_embed = serenity::CreateEmbed::new()
                .color(Colour::from_rgb(46, 81, 162))
                .title("Here's top 10 Matching Anime Based on your input.")
                .description(&anime_title);
            
            msg.edit(ctx, serenity::EditMessage::new().embed(new_embed)).await?;
            
            message = Some(msg);
            
            inter.create_response(ctx, serenity::CreateInteractionResponse::Acknowledge).await?;
        }

    }
    
    let builder = serenity::EditMessage::new().content("button expired.").components(
        vec![]
    );
    message.unwrap().edit(ctx, builder).await?;

    Ok(())
}