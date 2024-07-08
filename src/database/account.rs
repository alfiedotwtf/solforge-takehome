use crate::{common::account::Account, database::common::DbResponse};

impl From<DbResponse> for Account {
    fn from(response: DbResponse) -> Self {
        match response {
            DbResponse::Account(account) => account,
            _ => panic!("Error retrieving account"),
        }
    }
}
