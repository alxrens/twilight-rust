use std::sync::Arc;

use diesel::{r2d2, PgConnection, RunQueryDsl};
use reqwest::Client;

use super::model::InventoryAnicard;
use crate::utils::connection::DbPool;




pub async fn get_inventory_anicard(pool : &Arc<DbPool>) -> Result<Vec<InventoryAnicard>, anyhow::Error>{
    let res = diesel::sql_query("SELECT * FROM inventory_anicard")
    .load(&mut pool.get().unwrap())?;
    Ok(res)
}

pub async fn get_inventory_by_id(pool : &Arc<DbPool>, subid : String) -> Result<Option<InventoryAnicard>, anyhow::Error>{
    let res = diesel::sql_query("SELECT * FROM inventory_anicard WHERE id = $1")
    .bind::<diesel::sql_types::Text, _>(subid)
    .get_result(&mut pool.get().unwrap())?;

    Ok(Some(res))
}