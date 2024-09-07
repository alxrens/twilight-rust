use std::sync::Arc;

use reqwest::Client;








pub struct NetConn {
    pub conn : Arc<Client>
}

pub fn init_client_connection() -> NetConn {
    NetConn { conn : Arc::new(Client::new()) }
}