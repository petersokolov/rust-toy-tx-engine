use std::collections::HashMap;

use crate::account::Account;
use crate::transaction::{DisputeState, Transaction, TransactionRecord};

pub struct Engine {
    pub accounts: HashMap<u16, Account>,
    pub transactions: HashMap<u32, TransactionRecord>,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            accounts: HashMap::new(),
            transactions: HashMap::new(),
        }
    }

    pub fn apply(&mut self, tx_id: Transaction) {
        // TODO: implement logic for each transaction type
        print!("Applying transaction: {:?}", tx_id);
    }
}
