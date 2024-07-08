use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Account {
    pub id: String,
    pub balance: u64,
}

impl Account {
    pub fn new(id: String, balance: u64) -> Self {
        Account { id, balance }
    }
}
