use serde::{Deserialize, Serialize};
use diesel::{deserialize::Queryable, prelude::QueryableByName, AsChangeset, Insertable};




#[derive(Debug, Serialize, Deserialize, Queryable,QueryableByName, Insertable, AsChangeset)]
#[diesel(table_name=crate::schema::inventory_anicard)]
pub struct InventoryAnicard {
    pub id : String,
    pub anicard_id : i32,
    pub inventory_id : i32,
    pub status_payment : Option<String>,
    pub left_amount : Option<i32>
}
