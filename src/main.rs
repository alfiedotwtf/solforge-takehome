mod api_server;
mod common;
mod database;
mod indexer;

use crate::{
    api_server::server::ApiServer, database::server::DbConnection, indexer::server::Indexer,
};

use std::sync::Arc;
use tokio::{join, signal::ctrl_c, sync::broadcast, sync::Mutex};

#[tokio::main]
async fn main() {
    let db_connection = Arc::new(Mutex::new(DbConnection::new()));

    let (tx, _) = broadcast::channel(1);
    let shutdown = shutdown(tx.clone());

    let mut api_server = ApiServer::new(db_connection.clone(), tx.clone());
    api_server.start().await;

    let mut indexer = Indexer::new(db_connection.clone(), tx.clone());
    indexer.start().await;

    // TODO: separate into different processes rather than separate tasks within
    // the same process. That way the indexer can run by itself without needing
    // to serve the API, or they can even run on multiple machines for scaleout etc.

    join!(api_server.wait(), indexer.wait(), shutdown);
}

async fn shutdown(tx: broadcast::Sender<()>) {
    let _ = ctrl_c().await;

    println!("Shutting down...");
    let _ = tx.send(());
}
