use std::sync::Arc;



use diesel::query_dsl::methods::FilterDsl;
use diesel::{ExpressionMethods, RunQueryDsl};

use super::model::Inventory;

use crate::utils::connection::DbPool;
use crate::schema::inventory::dsl::*;


pub async fn get_inventorys(pool : &Arc<DbPool>) -> Result<Vec<Inventory>, anyhow::Error>{
    let res = diesel::sql_query("SELECT * FROM inventory")
    .load(&mut pool.get().unwrap())?;
    Ok(res)
}

pub async fn get_inventory_by_id(pool : &Arc<DbPool>, subid : String) -> Result<Option<Inventory>, anyhow::Error>{
    let res = diesel::sql_query("SELECT * FROM inventory WHERE id = $1")
    .bind::<diesel::sql_types::Text, _>(subid)
    .get_result(&mut pool.get().unwrap())?;

    Ok(Some(res))
}

pub async fn create_inventory(pool : &Arc<DbPool>, input_inventory : Inventory) -> Result<(), anyhow::Error>{
    diesel::insert_into(inventory).values(input_inventory).execute(&mut pool.get().unwrap())?;
    Ok(())
}

pub async fn update_inventory(pool : &Arc<DbPool>, subid : String, input_inventory : Inventory) -> Result<(), anyhow::Error>{
    diesel::update(inventory.filter(id.eq(subid))).set(input_inventory).execute(&mut pool.get().unwrap())?;
    Ok(())
}
pub async fn delete_inventory(pool : &Arc<DbPool>, subid : String) -> Result<(), anyhow::Error>{
    diesel::delete(inventory.filter(id.eq(subid))).execute(&mut pool.get().unwrap())?;
    Ok(())
}