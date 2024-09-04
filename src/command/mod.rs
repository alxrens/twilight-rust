use anyhow::Error;
use poise::Command;

use crate::Data;



mod world;

pub fn create_command_framework() -> Vec<Command<Data, Error>> {
    vec![
        world::hello(),
        world::embeed()
        ]
}