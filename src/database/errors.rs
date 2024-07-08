use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Error)]
pub enum DbError {
    #[error("Block not found")]
    BlockNotFound,
    #[error("Slot not found")]
    SlotNotFound,
    #[error("Transaction not found")]
    TransactionNotFound,
    #[error("Account not found")]
    AccountNotFound,
    #[error("Other error: {0}")]
    _Other(String),
}
