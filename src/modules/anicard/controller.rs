use std::sync::Arc;



use diesel::query_dsl::methods::FilterDsl;
use diesel::{ExpressionMethods, RunQueryDsl};

use super::model::AniCards;

use crate::utils::connection::DbPool;
use crate::schema::anicard::dsl::*;


pub async fn get_anicards(pool : &Arc<DbPool>) -> Result<Vec<AniCards>, anyhow::Error>{
    let res = diesel::sql_query("SELECT * FROM anicard")
    .load(&mut pool.get().unwrap())?;
    Ok(res)
}

pub async fn insert_anicard(pool : &Arc<DbPool>, input_anicard : AniCards) -> Result<(), anyhow::Error>{
    let check = get_anicards_by_id(pool, input_anicard.id.clone()).await;
    if !check.unwrap().is_none(){
        return Ok(());
    }

    diesel::insert_into(anicard).values(input_anicard).execute(&mut pool.get().unwrap())?;
    Ok(())
}

pub async fn get_anicards_by_id(pool : &Arc<DbPool>, subid : String) -> Result<Option<AniCards>, anyhow::Error>{
    let res = diesel::sql_query("SELECT * FROM anicard WHERE id = $1")
    .bind::<diesel::sql_types::Text, _>(subid)
    .get_result(&mut pool.get().unwrap())?;

    Ok(Some(res))
}

pub async fn update_anicard(pool : &Arc<DbPool>, subid : String, input_anicard : AniCards) -> Result<(), anyhow::Error>{
    diesel::update(anicard.filter(id.eq(subid))).set(input_anicard).execute(&mut pool.get().unwrap())?;
    Ok(())
}
pub async fn delete_anicard(pool : &Arc<DbPool>, subid : String) -> Result<(), anyhow::Error>{
    diesel::delete(anicard.filter(id.eq(subid))).execute(&mut pool.get().unwrap())?;
    Ok(())
}