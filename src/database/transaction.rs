use crate::{common::transaction::Transaction, database::common::DbResponse};

impl From<DbResponse> for Transaction {
    fn from(response: DbResponse) -> Self {
        match response {
            DbResponse::Transaction(transaction) => transaction,
            _ => panic!("Error retrieving transaction"),
        }
    }
}
