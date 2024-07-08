use crate::database::server::DbConnection;

use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct ApiServerState {
    db_connection: Arc<Mutex<DbConnection>>,
}

impl ApiServerState {
    pub fn new(db_connection: Arc<Mutex<DbConnection>>) -> Self {
        ApiServerState { db_connection }
    }

    pub fn db_connection(&self) -> Arc<Mutex<DbConnection>> {
        self.db_connection.clone()
    }
}
