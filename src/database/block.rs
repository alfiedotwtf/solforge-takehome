use crate::{common::block::Block, database::common::DbResponse};

impl From<DbResponse> for Block {
    fn from(response: DbResponse) -> Self {
        match response {
            DbResponse::Block(block) => block,
            _ => panic!("Error retrieving block"),
        }
    }
}
