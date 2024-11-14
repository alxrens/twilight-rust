use anyhow::Error;
use diesel::{query_dsl::QueryDsl, prelude::*, query_dsl::RunQueryDsl};
use uuid::Uuid;

use crate::utils::connection::DbPool;
use super::model::MagiMemories;
use crate::schema::magi_memories::dsl::*;


pub async fn get_by_user_id(input_user_id : &str, db_conn : &DbPool) -> Result<Vec<MagiMemories>, Error> {
    let mut conn = db_conn.get().unwrap();
    let memories : Vec<MagiMemories> = magi_memories.filter(user_id.eq(input_user_id)).load(&mut conn).unwrap();
    Ok(memories)
}

pub async fn update_magi_memories(input_user_id : &str, new_memory : &str, db_conn : &DbPool) -> Result<String, Error> {
    let mut conn = db_conn.get().unwrap();
    let _update = diesel::update(magi_memories.filter(user_id.eq(input_user_id))).set(memory.eq(new_memory)).get_result::<MagiMemories>(&mut conn);
    

Ok("updated".to_string())
}

pub async fn create_magi_memories(magi_mem : MagiMemories, db_conn : &DbPool) -> Result<String, Error> {
    let mut conn = db_conn.get().unwrap();
    let create_magi_mem = MagiMemories {
        id : Uuid::new_v4().to_string(),
        ..magi_mem
    };
    let _ = diesel::insert_into(magi_memories).values(create_magi_mem).execute(&mut conn);
    Ok("created".to_string())
}