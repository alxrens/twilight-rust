use std::sync::Arc;

use reqwest::Client;

use diesel::{r2d2, PgConnection};

pub type DbPool=r2d2::Pool<r2d2::ConnectionManager<PgConnection>>;



pub struct NetConn {
    pub conn : Arc<Client>
}

pub fn init_client_connection() -> NetConn {
    NetConn { conn : Arc::new(Client::new()) }
}

pub fn init_db_pool() -> DbPool {
    let dburl = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");
    let manager = r2d2::ConnectionManager::<PgConnection>::new(dburl);
    r2d2::Pool::builder().build(manager).expect("Failed to create db connection pool")
}