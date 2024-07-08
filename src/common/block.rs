use serde::{Deserialize, Serialize};
use solana_transaction_status::UiConfirmedBlock;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Block {
    pub id: String,
    pub previous_blockhash: String,
    pub slot: u64,
    pub block_time: Option<i64>,
    pub block_height: Option<u64>,
}

impl From<UiConfirmedBlock> for Block {
    fn from(block: UiConfirmedBlock) -> Self {
        Block {
            id: block.blockhash.to_string(),
            previous_blockhash: block.previous_blockhash.to_string(),
            slot: block.parent_slot,
            block_time: block.block_time,
            block_height: block.block_height,
        }
    }
}

impl From<&UiConfirmedBlock> for Block {
    fn from(block: &UiConfirmedBlock) -> Self {
        block.into()
    }
}
