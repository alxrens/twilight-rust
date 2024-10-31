use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::utils::{connection::NetConn, prompt::{create_prompt, web_search, Message}};


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


pub async fn chat(question : &str, client : &Arc<NetConn>) -> Result<String, anyhow::Error> {
    // dotenv::dotenv().ok();
    let url = std::env::var("OLLAMA_URL").expect("OLLAMA_URL not set");
    let chat_url = format!("{}/api/chat", url);
    let se = web_search(question, client).await?;
    let web_result =format!("and here is the search engine result for your reference,try to check the link below. if my question is just simple interaction such as \"who are you?\" or \"what is your name?\" YOU DONT HAVE TO state the url in the respond but you can just say something about yourself. but if what i said is genuine question about knowledge YOU HAVE TO put the source url: {}", se);
    let mut generated_prompt = create_prompt(String::from("dolphin-mistral")).await;

    let new_question = format!("{} {}",question, web_result);
    let add_message = Message{
        role : "user".to_string(),
        content : new_question
    };

    generated_prompt.messages.push(add_message);
    let data = client.conn.post(chat_url).body(serde_json::to_string(&generated_prompt).unwrap()).send().await;
    match data {
        Ok(data) => {
            let json : ChatApiResponse   = data.json().await?;
            // let respond = format!("{}\n {}",json.message.content, se);
            Ok(json.message.content)
            // Ok(respond)
        },
        Err(e) => {
            log::error!("{:?}", e);
            Err(anyhow::anyhow!(e))
    }
}}







