use crate::{
    common::{account::Account, block::Block, transaction::Transaction},
    database::{common::DbQuery, common::DbResponse, errors::DbError},
};

use std::collections::HashMap;
use tokio::time::{sleep, Duration};

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct DbConnection {
    blocks: HashMap<String, Block>,
    slots_to_blocks: HashMap<u64, String>,
    transactions: HashMap<String, Transaction>,
    accounts: HashMap<String, Account>,
}

//
// A fake database... I started out with a HashMap as Joel said, but then it
// grew into something that better resembled a database and database handle. In
// particular, it mimicks retrying for unarrived data and dropped database
// connections etc vs a straight HashMap.
//

impl DbConnection {
    pub fn new() -> Self {
        Self {
            blocks: HashMap::new(),
            slots_to_blocks: HashMap::new(),
            transactions: HashMap::new(),
            accounts: HashMap::new(),
        }
    }

    //
    // Dump the database for debugging
    //
    // pub async fn dump(&self) -> Self {
    //     self.clone()
    // }

    pub async fn query(&mut self, query: &DbQuery) -> Result<DbResponse, DbError> {
        loop {
            if let Ok(response) = self._query_no_retry(query).await {
                return Ok(response);
            }

            // TODO: Add config for user-specified timeouts
            sleep(Duration::from_millis(100)).await;
        }
    }

    pub async fn _query_no_retry(&mut self, query: &DbQuery) -> Result<DbResponse, DbError> {
        match query {
            DbQuery::InsertBlock(block) => self._insert_block(block.clone()).await,
            DbQuery::GetBlock(id) => self._get_block(id).await,
            DbQuery::GetBlockBySlot(slot) => self._get_block_by_slot(*slot).await,

            DbQuery::InsertTransaction(transaction) => self._insert_transaction(transaction).await,
            DbQuery::GetTransaction(id) => self._get_transaction(id).await,

            DbQuery::InsertAccount(account) => self._insert_account(account.clone()).await,
            DbQuery::GetAccount(id) => self._get_account(id).await,
        }
    }

    //
    // Block methods
    //

    async fn _insert_block(&mut self, block: Block) -> Result<DbResponse, DbError> {
        self.blocks.insert(block.id.clone(), block.clone());
        self.slots_to_blocks.insert(block.slot, block.id.clone());
        Ok(DbResponse::Ok)
    }

    async fn _get_block(&self, id: &str) -> Result<DbResponse, DbError> {
        match self.blocks.get(id) {
            Some(block) => Ok(DbResponse::Block(block.clone())),
            None => Err(DbError::BlockNotFound),
        }
    }

    async fn _get_block_by_slot(&self, slot: u64) -> Result<DbResponse, DbError> {
        match self.slots_to_blocks.get(&slot) {
            Some(block_id) => self._get_block(block_id).await,
            None => Err(DbError::SlotNotFound),
        }
    }

    //
    // Transaction methods
    //

    async fn _insert_transaction(
        &mut self,
        transaction: &Transaction,
    ) -> Result<DbResponse, DbError> {
        self.transactions
            .insert(transaction.id.clone(), transaction.clone());
        Ok(DbResponse::Ok)
    }

    async fn _get_transaction(&self, id: &str) -> Result<DbResponse, DbError> {
        match self.transactions.get(id) {
            Some(transaction) => Ok(DbResponse::Transaction(transaction.clone())),
            None => Err(DbError::TransactionNotFound),
        }
    }

    //
    // Account methods
    //

    async fn _insert_account(&mut self, account: Account) -> Result<DbResponse, DbError> {
        self.accounts.insert(account.id.clone(), account.clone());
        Ok(DbResponse::Ok)
    }

    async fn _get_account(&self, pubkey: &str) -> Result<DbResponse, DbError> {
        match self.accounts.get(pubkey) {
            Some(account) => Ok(DbResponse::Account(account.clone())),
            None => Err(DbError::AccountNotFound),
        }
    }
}
