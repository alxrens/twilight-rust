use serde::{Deserialize, Serialize};
use diesel::{deserialize::Queryable, prelude::QueryableByName, AsChangeset, Insertable};









#[derive(Serialize, Deserialize, Queryable, Insertable, AsChangeset, QueryableByName)]
#[diesel(table_name=crate::schema::anicard)]
pub struct AniCards {
    pub id : String,
    pub character_name : String,
    pub favorites : i32,
    pub anime : String,
    pub img : String,
}