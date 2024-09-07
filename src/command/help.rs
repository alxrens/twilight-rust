use std::fs;
use crate::Context;


#[poise::command(slash_command)]
pub async fn help(ctx: Context<'_>) -> Result<(), anyhow::Error> {
    let command = fs::read_dir("src/command")?.map(|res| {
        res.map(|e| e.path())
    }).collect::<Result<Vec<_>, std::io::Error>>()?;
    let mut commands : Vec<String> = Vec::new();
    for c in command.iter() {
        let command_name = c.to_str()
        .unwrap()
        .split("/")
        .last()
        .unwrap()
        .replace(".rs", "");
        if command_name == "mod" || command_name == "help" {continue;}

        commands.push(format!("+ {command_name}"));
    }
    let commands = commands.join("\n");
    let reply =  format!("
    ```diff\n+ Arch Twilight\n+ Here are the list of the available commands: \n\r\r{commands}\n\r\r- start typing slash command now.```
    ");

    ctx.say(reply).await?;
    Ok(())
}  