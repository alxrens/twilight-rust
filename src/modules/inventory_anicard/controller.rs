use std::sync::Arc;

use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};


use super::model::InventoryAnicard;
use crate::utils::connection::DbPool;

use crate::schema::inventory_anicard::dsl::*;


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

pub async fn create_inventory_anicard(pool : &Arc<DbPool>, input_inventory_anicard : InventoryAnicard) -> Result<(), anyhow::Error>{
    diesel::insert_into(inventory_anicard).values(input_inventory_anicard).execute(&mut pool.get().unwrap())?;
    Ok(())
}

pub async fn update_inventory_anicard(pool : &Arc<DbPool>, subid : String, input_inventory_anicard : InventoryAnicard) -> Result<(), anyhow::Error>{
    diesel::update(inventory_anicard.filter(id.eq(subid))).set(input_inventory_anicard).execute(&mut pool.get().unwrap())?;
    Ok(())
}
pub async fn delete_inventory_anicard(pool : &Arc<DbPool>, subid : String) -> Result<(), anyhow::Error>{
    diesel::delete(inventory_anicard.filter(id.eq(subid))).execute(&mut pool.get().unwrap())?;
    Ok(())
}