use std::sync::Arc;

use command::create_command_framework;
use poise::serenity_prelude::{self, ActivityData, GatewayIntents};
use anyhow::Error;
use utils::connection::NetConn;
use env_logger::{Env, Builder};

pub struct Data{
    netconn : Arc<NetConn>
}
type Context<'a> = poise::Context<'a, Data, Error>;

mod command;
mod api;
mod utils;


async fn event_handler(
    ctx : &serenity_prelude::Context,
    event : &serenity_prelude::FullEvent,
    _framework : poise::FrameworkContext<'_, Data, Error>,
) -> 
Result<(), Error> {
    match event {
        serenity_prelude::FullEvent::Ready { data_about_bot, .. }=> {
            // println!("{} is connected!", data_about_bot.user.name);
            log::info!("{} is connected!", data_about_bot.user.name);
            ctx.set_presence(Some(ActivityData::listening("Your Voice")), serenity_prelude::OnlineStatus::DoNotDisturb);
        }
        _=> {}
    }
    Ok(())
}




#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let token = std::env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    // let g = GuildId::new(std::env::var("GUILD_ID").expect("Expected a guild id in the environment").parse().unwrap());
    // let r = RoleId::new(std::env::var("ROLE_ID").expect("Expected a role id in the environment").parse().unwrap());
    let intents = GatewayIntents::non_privileged();
    let netconn = Arc::new(utils::connection::init_client_connection());

    Builder::from_env(Env::new().default_filter_or("debug"))
        .filter_module("h2", log::LevelFilter::Off)
        .filter_module("hyper", log::LevelFilter::Off)
        .filter_module("reqwest", log::LevelFilter::Off)
        .filter_module("rustls", log::LevelFilter::Off)
        .filter_module("poise", log::LevelFilter::Debug)
        .filter_module("serenity", log::LevelFilter::Off)
        .filter_module("tracing::span", log::LevelFilter::Off)
        .init();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions{
            commands: create_command_framework().await,
            event_handler: |ctx, event, framework, _data| {
                Box::pin(event_handler( ctx, event, framework))
            },
            ..Default::default()
        })
        .setup(
            |ctx, _ready, framework| {
                Box::pin(async move {
                    poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                    Ok(Data{netconn: netconn.clone()})
                })
            })
        .build();

    let client = serenity_prelude::ClientBuilder::new(&token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
}