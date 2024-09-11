use std::sync::Arc;

use diesel::{r2d2, PgConnection};
use reqwest::Client;

pub type DbPool = r2d2::Pool<r2d2::ConnectionManager<PgConnection>>;

pub struct DbConnection{
    pub conn : Arc<DbPool>
}

impl DbConnection {
    pub fn new() -> DbConnection {
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        let manager = r2d2::ConnectionManager::<PgConnection>::new(database_url);

        let pool = r2d2::Pool::builder().build(manager).expect("failed to create connection pool to database");

        DbConnection {
            conn : Arc::new(pool)
        }
    }
}

pub struct NetConn {
    pub conn : Arc<Client>
}

pub fn init_client_connection() -> NetConn {
    NetConn { conn : Arc::new(Client::new()) }
}
