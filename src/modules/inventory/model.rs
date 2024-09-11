use serde::{Deserialize, Serialize};
use diesel::{deserialize::Queryable, prelude::QueryableByName, AsChangeset, Insertable};


#[derive(Debug, Serialize, Deserialize, Queryable,QueryableByName, Insertable, AsChangeset)]
#[diesel(table_name=crate::schema::inventory)]
pub struct Inventory {
    pub id : String,
    pub owner_id : String,
}






