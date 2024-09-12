use std::sync::Arc;

use crate::{modules::anicard_user::{controller, model}, utils::connection::DbConnection};


pub async fn register(auhtor_id : &str, author_name : &str, pool: &Arc<DbConnection>) -> Result<String, anyhow::Error> {
    let user = model::AniCardUser {
        id : String::from(auhtor_id),
        currency : Some(500),
    };
    let reg = controller::insert_anicard_user(&pool.conn, user).await;
    if reg.is_err(){
        return Ok(format!("{} you already registered to this bot.", author_name))
    }
    Ok(format!("{} thank you for registering for anicard here are 500 Arch Coins for you.", author_name))
}