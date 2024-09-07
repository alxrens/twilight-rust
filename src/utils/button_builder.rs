use poise::serenity_prelude::{ CreateActionRow, CreateButton};



pub async fn create_button(len : usize) -> Result<Vec<CreateActionRow>, anyhow::Error> {

    let mut buttons : Vec<CreateButton> = Vec::new();

    let mut action_row : Vec<CreateActionRow> = Vec::new();

    for i in 0..len {
        buttons.push(CreateButton::new((i + 1).to_string())
            .label((i+1).to_string())
            .style(poise::serenity_prelude::ButtonStyle::Secondary)
            .disabled(false)
        );

        if buttons.len() == 5 {
            action_row.push(CreateActionRow::Buttons(buttons));
            buttons = Vec::new();
        }
        
    }

    if !buttons.is_empty() {
        action_row.push(CreateActionRow::Buttons(buttons));
    }
    action_row.push(CreateActionRow::Buttons(
        vec![CreateButton::new("BackToList")
            .label("Back To List")
            .style(poise::serenity_prelude::ButtonStyle::Secondary)
            .disabled(false)]));

    Ok(action_row)

}