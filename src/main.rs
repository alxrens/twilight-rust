use command::create_command_framework;
use poise::serenity_prelude::{self, GatewayIntents};
use anyhow::Error;


pub struct Data{
}
type Context<'a> = poise::Context<'a, Data, Error>;
mod command;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let token = std::env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    // let g = GuildId::new(std::env::var("GUILD_ID").expect("Expected a guild id in the environment").parse().unwrap());
    // let r = RoleId::new(std::env::var("ROLE_ID").expect("Expected a role id in the environment").parse().unwrap());
    let intents = GatewayIntents::non_privileged();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions{
            commands: create_command_framework(),
            ..Default::default()
        })
        .setup(
            |ctx, _ready, framework| {
                Box::pin(async move {
                    poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                    Ok(Data{})
                })
            })
        .options(poise::FrameworkOptions{
            event_handler: |ctx, event, framework, data| {
                Box::pin(event_handler(ctx, event, framework,   data))
            },
            ..Default::default() 
        })
        .build();

    let client = serenity_prelude::ClientBuilder::new(&token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
}

#[allow(unused_variables)]
async fn event_handler(
    ctx: &serenity_prelude::Context,
    event : &serenity_prelude::FullEvent,
    _framework : poise::FrameworkContext<'_, Data, Error>,
    data : &Data
) -> 
Result<(), Error> {
    match event {
        serenity_prelude::FullEvent::Ready { data_about_bot, .. }=> {
            println!("{} is connected!", data_about_bot.user.name);
        }
        _=> {}
    }
    Ok(())
}
