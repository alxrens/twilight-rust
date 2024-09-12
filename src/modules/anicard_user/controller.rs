use std::sync::Arc;


use anyhow::anyhow;
use diesel::query_dsl::methods::FilterDsl;
use diesel::{ExpressionMethods, RunQueryDsl};

use super::model::AniCardUser;

use crate::utils::connection::DbPool;
use crate::schema::anicard_user::dsl::*;


pub async fn get_anicard_users(pool : &Arc<DbPool>) -> Result<Vec<AniCardUser>, anyhow::Error>{
    let res = diesel::sql_query("SELECT * FROM anicard_user")
    .load(&mut pool.get().unwrap())?;
    Ok(res)
}

pub async fn insert_anicard_user(pool : &Arc<DbPool>, input_anicard_user : AniCardUser) -> Result<(), anyhow::Error>{
    let check = get_anicard_users_by_id(pool, input_anicard_user.id.clone()).await?;
    if !check.is_none() {
     return  Err(anyhow!("Anicard user already exists"))
    }

    diesel::insert_into(anicard_user).values(input_anicard_user).execute(&mut pool.get().unwrap())?;

    Ok(())
}

pub async fn get_anicard_users_by_id(pool : &Arc<DbPool>, subid : String) -> Result<Option<AniCardUser>, anyhow::Error>{
    let res = diesel::sql_query("SELECT * FROM anicard_user WHERE id = $1")
    .bind::<diesel::sql_types::Text, _>(subid)
    .get_result(&mut pool.get().unwrap());

    if res.is_err() {
        return Ok(None)
    }

    Ok(Some(res?))
}

pub async fn update_anicard_user(pool : &Arc<DbPool>, subid : String, input_anicard_user : AniCardUser) -> Result<(), anyhow::Error>{
    diesel::update(anicard_user.filter(id.eq(subid))).set(input_anicard_user).execute(&mut pool.get().unwrap())?;
    Ok(())
}
pub async fn delete_anicard_user(pool : &Arc<DbPool>, subid : String) -> Result<(), anyhow::Error>{
    diesel::delete(anicard_user.filter(id.eq(subid))).execute(&mut pool.get().unwrap())?;
    Ok(())
}