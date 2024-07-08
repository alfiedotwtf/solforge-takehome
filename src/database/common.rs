use crate::common::{account::Account, block::Block, transaction::Transaction};

#[derive(Debug, Clone)]
pub enum DbQuery {
    InsertBlock(Block),
    GetBlock(String),
    GetBlockBySlot(u64),

    InsertTransaction(Transaction),
    GetTransaction(String),

    InsertAccount(Account),
    GetAccount(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum DbResponse {
    Ok,
    Block(Block),
    Transaction(Transaction),
    Account(Account),
}
