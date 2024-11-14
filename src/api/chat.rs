use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::{magi_memories::{self, controller::{create_magi_memories, update_magi_memories}, model::MagiMemories}, utils::{connection::{DbPool, NetConn}, prompt::{create_prompt, update_message_history, Message}}};


#[derive(Serialize, Deserialize)]
pub struct ChatResponseMessage {
    pub role : String,
    pub content : String
}

#[derive(Serialize, Deserialize)]
pub struct ChatApiResponse {
    pub model : String,
    pub created_at : String,
    pub message : ChatResponseMessage,
    pub done_reason : String,
    pub done : bool,
    pub total_duration : i64,
    pub load_duration : i64,
    pub prompt_eval_count : i64,
    pub prompt_eval_duration : i64,
    pub eval_count : i64,
    pub eval_duration : i64
}


pub async fn chat(question : &str, client : &Arc<NetConn>, user_id: &str, dbpool : &DbPool) -> Result<String, anyhow::Error> {
    // dotenv::dotenv().ok();
    let url = std::env::var("OLLAMA_URL").expect("OLLAMA_URL not set");
    let chat_url = format!("{}/api/chat", url);
    
    //turn on to enable web_search
    // let se = web_search(question, client).await?;
    // let web_result =format!("and here is the search engine result for your reference,try to check the link below. if my question is just simple interaction such as \"who are you?\" or \"what is your name?\" YOU DONT HAVE TO state the url in the respond but you can just say something about yourself. but if what i said is genuine question about knowledge YOU HAVE TO put the source url: {}", se);
    let messages: Vec<Message>;
    let magimem = magi_memories::controller::get_by_user_id(user_id, dbpool).await?;
    if magimem.len() ==0 {
        messages = create_prompt(String::from("CognitiveComputations/dolphin-llama3.1")).await.messages;
        let magimem_tostr = serde_json::to_string(&messages).unwrap();
        let magmem = MagiMemories{
            id : "".to_string(),
            user_id : user_id.to_string(),
            memory : magimem_tostr
        };
        create_magi_memories(magmem, dbpool).await?;
    } else {
        let raw_messages = &magimem[0].memory;

        messages = serde_json::from_str(&raw_messages)?;
    }

    let new_question = format!("{} {}",question, "".to_string());

    let generated_prompt = update_message_history(&new_question, "user", messages, "CognitiveComputations/dolphin-llama3.1:latest".to_string()).await;
    let data = client.conn.post(chat_url).body(serde_json::to_string(&generated_prompt).unwrap()).send().await;
    match data {
        Ok(data) => {
            let json : ChatApiResponse   = data.json().await?;
            // let respond = format!("{}\n {}",json.message.content, se);

            let update_ai_mess = update_message_history(&json.message.content, &json.message.role, generated_prompt.messages, json.model).await;
            let message_tostr = serde_json::to_string(&update_ai_mess.messages)?;
            update_magi_memories(user_id, &message_tostr, dbpool).await?;
            Ok(json.message.content)
        },
        Err(e) => {
            log::error!("{:?}", e);
            Err(anyhow::anyhow!(e))
    }
}}







