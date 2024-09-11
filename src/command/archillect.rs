

use rand::Rng;
use rand::rngs::StdRng;
use rand::SeedableRng;

use crate::Context;

static URL : &str = "https://archillect.com/";

#[poise::command(slash_command)]
pub async fn archillect(
    ctx : Context<'_>,
) -> Result<(), anyhow::Error> {
    let limitnum = 410187;
    let mut rng = StdRng::from_entropy();
    let math_min = rng.gen_range(1..=limitnum);
    let math_plus = rng.gen_range(1..=limitnum);

    if math_plus> math_min {
        let index_number = limitnum - math_min;
        ctx.say(format!("{}{}", URL, index_number)).await?;
    }else {
        let calculate = limitnum - math_min + math_plus;
        if calculate>limitnum{
            let index_number = limitnum - math_min;
            ctx.say(format!("{}{}", URL, index_number)).await?;
        } else {
            ctx.say(format!("{}{}", URL, calculate)).await?;
        }
    }

    Ok(())
}