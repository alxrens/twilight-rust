use std::fs;
use crate::Context;


#[poise::command(slash_command)]
pub async fn help(ctx: Context<'_>) -> Result<(), anyhow::Error> {
    let command = fs::read_dir("src/command")?.map(|res| {
        res.map(|e| e.path())
    }).collect::<Result<Vec<_>, std::io::Error>>()?;
    let prefix_command = fs::read_dir("src/command/prefix")?.map(|res| {
        res.map(|e| e.path())
    }).collect::<Result<Vec<_>, std::io::Error>>()?;
    let mut commands : Vec<String> = Vec::new();
    let mut prefix_commands : Vec<String> = Vec::new();
    for c in command.iter() {
        //this code is so lazy but i love it lmao
        let command_name = c.to_str()
        .unwrap()
        .split("/")
        .last()
        .unwrap()
        .replace(".rs", "");
        if command_name == "mod" || command_name == "help" || command_name == "prefix" {continue;}

        commands.push(format!("+ {command_name}"));
    }

    for c in prefix_command.iter() {
        //this code is so lazy but i love it lmao
        let command_name = c.to_str()
        .unwrap()
        .split("/")
        .last()
        .unwrap()
        .replace(".rs", "");
        if command_name == "mod" || command_name == "help" {continue;}
        prefix_commands.push(format!("+ {command_name}"));
    }
    let commands = commands.join("\n");
    let prefix_commands = prefix_commands.join("\n");
    let reply =  format!("
    ```diff\n+ Arch Twilight\n+ Here are the list of the available commands: \n\r\r- slash commands : \n{commands}\n \r\r- prefix($) commands : \n{prefix_commands}\n\r\r- start typing slash command now.```
    ");

    ctx.say(reply).await?;
    Ok(())
}  