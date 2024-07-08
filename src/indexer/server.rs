use crate::{
    common::{account::Account, block::Block, transaction::Transaction},
    database::{common::DbQuery, server::DbConnection},
    indexer::errors::IndexerError,
};

use futures_util::StreamExt;
use solana_pubsub_client::nonblocking::pubsub_client::PubsubClient;
use solana_sdk::commitment_config::{CommitmentConfig, CommitmentLevel};
use solana_transaction_status::UiConfirmedBlock;
use std::sync::Arc;

use solana_client::{
    client_error::ClientErrorKind, nonblocking::rpc_client::RpcClient, rpc_config::RpcBlockConfig,
    rpc_request::RpcError,
};

use tokio::{
    select,
    sync::{broadcast::Sender, Mutex},
    task::JoinHandle,
    time::sleep,
};

const WEBSOCKET_URL: &str = "ws://api.testnet.solana.com";
const RPC_URL: &str = "http://api.testnet.solana.com";
const SLOT_TIME: u64 = 400;
const RETRIES_PER_SLOT: u64 = 3;

pub struct Indexer {
    task: Option<JoinHandle<()>>,
    db_connection: Arc<Mutex<DbConnection>>,
    tx: Sender<()>,
}

impl Indexer {
    pub fn new(db_connection: Arc<Mutex<DbConnection>>, tx: Sender<()>) -> Self {
        Indexer {
            task: None,
            db_connection,
            tx,
        }
    }

    pub async fn start(&mut self) {
        if self.task.is_some() {
            println!("Indexer already running");
            return;
        }

        println!("Starting indexer...");

        self.task = Some(tokio::spawn({
            let db_connection = self.db_connection.clone();

            async move {
                let rpc_client = RpcClient::new(RPC_URL.to_string());
                let pubsub_client = Arc::new(
                    PubsubClient::new(WEBSOCKET_URL)
                        .await
                        .expect("Error subscribing to websocket"),
                );
                let (mut slot_notifications, _) = pubsub_client
                    .slot_subscribe()
                    .await
                    .expect("Error subscribing to slot notifications");

                while let Some(slot_info) = slot_notifications.next().await {
                    let db_connection = db_connection.clone();

                    loop {
                        let encoded_block = rpc_client
                            .get_block_with_config(
                                slot_info.slot - 100,
                                RpcBlockConfig {
                                    max_supported_transaction_version: Some(0),
                                    commitment: Some(CommitmentConfig {
                                        commitment: CommitmentLevel::Confirmed,
                                    }),
                                    ..Default::default()
                                },
                            )
                            .await;

                        match encoded_block {
                            Ok(encoded_block) => {
                                // TODO: Handle reorgs

                                let _ = process_block(db_connection, &encoded_block).await;

                                // Done. Move to the next slot
                                break;
                            }
                            Err(error) => match error.kind() {
                                ClientErrorKind::RpcError(RpcError::RpcResponseError {
                                    code: -32007,
                                    ..
                                })
                                | ClientErrorKind::RpcError(RpcError::RpcResponseError {
                                    code: -32009,
                                    ..
                                }) => {
                                    // Skip slot as it will never be availble to us
                                    break;
                                }
                                _ => {
                                    // Wait for the block to arrive
                                    sleep(tokio::time::Duration::from_millis(
                                        SLOT_TIME / RETRIES_PER_SLOT,
                                    ))
                                    .await
                                }
                            },
                        }
                    }
                }
            }
        }));

        println!("Indexer started")
    }

    pub async fn stop(mut self) {
        if let Some(task) = self.task.take() {
            println!("Stopping indexer...");
            task.abort();
            println!("Indexer stopped!");
        }
    }

    pub async fn wait(mut self) {
        if let Some(task) = self.task.take() {
            println!("Indexer running");
            let mut rx = self.tx.subscribe();

            select! {
                _ = task => println!("Indexer stopped on its own"),
                _ = rx.recv() => println!("Indexer interrupted"),
            }
        }

        self.stop().await;
    }
}

async fn process_block(
    db_connection: Arc<Mutex<DbConnection>>,
    confirmed_block: &UiConfirmedBlock,
) -> Result<(), IndexerError> {
    let block = Block::from(confirmed_block.clone());
    println!("Found block: {:?}", block.id);

    let _ = db_connection
        .lock()
        .await
        .query(&DbQuery::InsertBlock(block))
        .await;

    let txs = match &confirmed_block.transactions {
        Some(txs) => txs.clone(),
        None => Vec::new(),
    };

    for transaction in txs {
        let transaction = Transaction::from(transaction.clone());
        println!("Found transaction: {:?}", transaction.id);

        let _ = db_connection
            .lock()
            .await
            .query(&DbQuery::InsertTransaction(transaction.clone()))
            .await;

        for (account, balance) in transaction.accounts.iter().zip(transaction.balances.iter()) {
            let account = Account::new(account.to_string(), *balance);
            println!("Found account: {:?}", account.id);

            let _ = db_connection
                .lock()
                .await
                .query(&DbQuery::InsertAccount(account))
                .await;
        }
    }

    Ok(())
}
