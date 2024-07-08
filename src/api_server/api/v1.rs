use crate::{
    api_server::{errors::ApiServerError, state::ApiServerState},
    common::{account::Account, block::Block, transaction::Transaction},
    database::common::DbQuery,
};

use axum::{
    extract::{Path, State},
    routing::{get, Router},
    Json,
};

pub fn routes() -> Router<ApiServerState> {
    // TODO: Handle backfilling missing data

    Router::new()
        .route("/block/:id", get(get_block))
        .route("/block-by-slot/:slot", get(get_block_by_slot))
        .route("/transaction/:id", get(get_transaction))
        .route("/account/:id", get(get_account))

    // TODO: Enable/disable via a config file
    //.route("/dump", get(dump)) // Used for debugging
}

async fn get_block(
    State(state): State<ApiServerState>,
    Path(id): Path<String>,
) -> Result<Json<Block>, ApiServerError> {
    let block = state
        .db_connection()
        .lock()
        .await
        .query(&DbQuery::GetBlock(id))
        .await?;

    Ok(Json(block.into()))
}

async fn get_block_by_slot(
    State(state): State<ApiServerState>,
    Path(slot): Path<u64>,
) -> Result<Json<Block>, ApiServerError> {
    let block = state
        .db_connection()
        .lock()
        .await
        .query(&DbQuery::GetBlockBySlot(slot))
        .await?;

    Ok(Json(block.into()))
}

async fn get_transaction(
    State(state): State<ApiServerState>,
    Path(id): Path<String>,
) -> Result<Json<Transaction>, ApiServerError> {
    let transaction = state
        .db_connection()
        .lock()
        .await
        .query(&DbQuery::GetTransaction(id))
        .await?;

    Ok(Json(transaction.into()))
}

async fn get_account(
    State(state): State<ApiServerState>,
    Path(id): Path<String>,
) -> Result<Json<Account>, ApiServerError> {
    let account = state
        .db_connection()
        .lock()
        .await
        .query(&DbQuery::GetAccount(id))
        .await?;

    Ok(Json(account.into()))
}

// use crate::database::server::DbConnection;
//
// async fn dump(State(state): State<ApiServerState>) -> Result<Json<DbConnection>, ApiServerError> {
//     Ok(Json(
//         state.db_connection().lock().await.dump().await.clone(),
//     ))
// }
