use anyhow::Error;
use poise::Command;

use crate::Data;


pub mod about;
pub mod anilist;
pub mod help;
pub mod pets;
pub mod roast;

pub async fn create_command_framework() -> Vec<Command<Data, Error>> {
    vec![
        help::help(),
        about::about(),
        anilist::anilist(),
        pets::pets(),
        roast::roast()
    ]
}