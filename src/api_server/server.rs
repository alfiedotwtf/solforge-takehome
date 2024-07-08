use crate::{
    api_server::{api::v1, errors::ApiServerError, state::ApiServerState},
    database::server::DbConnection,
};

use serde_json::json;
use std::sync::Arc;
use tower_http::cors::{AllowMethods, Any, CorsLayer};

use axum::{
    http::Method,
    response::IntoResponse,
    routing::{get, Router},
    Json,
};

use tokio::{
    net::TcpListener,
    select, spawn,
    sync::{broadcast::Sender, Mutex},
    task::JoinHandle,
};

const CURRENT_API_VERSION: &str = "v1";

pub struct ApiServer {
    task: Option<JoinHandle<()>>,
    db_connection: Arc<Mutex<DbConnection>>,
    tx: Sender<()>,
}

impl ApiServer {
    pub fn new(db_connection: Arc<Mutex<DbConnection>>, tx: Sender<()>) -> Self {
        ApiServer {
            task: None,
            db_connection,
            tx,
        }
    }

    pub async fn start(&mut self) {
        if self.task.is_some() {
            println!("API server already running");
            return;
        }

        print!("Starting API server...");

        let routes = Router::new()
            .route("/", get(server_root))
            .nest("/api/v1", v1::routes())
            .fallback(bad_request)
            .with_state(ApiServerState::new(self.db_connection.clone()))
            .layer(
                CorsLayer::new()
                    .allow_methods(AllowMethods::list([Method::GET, Method::POST]))
                    .allow_headers(Any)
                    .allow_origin(Any),
            );

        self.task = Some(spawn({
            async move {
                let listener = TcpListener::bind("0.0.0.0:1337")
                    .await
                    .expect("Error binding to port 1337");

                println!(
                    "Listening on: {}",
                    listener
                        .local_addr()
                        .expect("Error getting listening socket info")
                );

                let _ = axum::serve(listener, routes).await;
            }
        }));

        println!("API server started")
    }

    pub async fn stop(&mut self) {
        if let Some(task) = &self.task.take() {
            println!("Stopping API server...");
            task.abort();
            println!("API server stopped!")
        }
    }

    pub async fn wait(&mut self) {
        if let Some(task) = self.task.take() {
            println!("API server running");
            let mut rx = self.tx.subscribe();

            select! {
                _ = task => println!("API server stopped on its own"),
                _ = rx.recv() => println!("API server interrupted"),
            }
        }

        self.stop().await;
    }
}

async fn server_root() -> impl IntoResponse {
    Json(json!({
        "cargo_version": env!("CARGO_PKG_VERSION"),
        "api_version": CURRENT_API_VERSION,
    }))
}

async fn bad_request() -> Result<(), ApiServerError> {
    Err(ApiServerError::BadRequest)
}
