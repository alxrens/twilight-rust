use serde::{Deserialize, Serialize};
use diesel::{deserialize::Queryable, prelude::QueryableByName, AsChangeset, Insertable};






#[derive(Debug, Serialize, Deserialize, Queryable,QueryableByName, Insertable, AsChangeset)]
#[diesel(table_name=crate::schema::anicard_user)]
pub struct AniCardUser {
    pub id : String,
    pub currency : Option<i32>,
}