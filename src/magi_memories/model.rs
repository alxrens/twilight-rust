use diesel::prelude::{AsChangeset, Insertable, Queryable, QueryableByName};
use serde::{Serialize, Deserialize};






#[derive(Serialize, Deserialize,Clone, AsChangeset, Queryable, QueryableByName, Insertable)]
#[diesel(table_name=crate::schema::magi_memories)]
pub struct MagiMemories {
    pub id : String,
    pub user_id : String,
    pub memory : String,
}