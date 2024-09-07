
use crate::api::pets;

use crate::Context;

#[poise::command(slash_command)]
pub async fn pets(
    ctx : Context<'_>,
    #[description = "Input pet name"]
    #[choices("cat","fox", "shibes", "male_horse" )]
    pet : &str
) -> Result<(), anyhow::Error> {
    let app_data = ctx.data();
    let pet_image_url = pets::get_animal_image(pet, &app_data.netconn).await?;

    ctx.say(pet_image_url).await?;
    Ok(())
}