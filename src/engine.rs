use std::collections::HashMap;

use rust_decimal::Decimal;

use crate::account::Account;
use crate::transaction::{
    DisputeState, Transaction, TransactionRecord, TransactionSource, TransactionType,
};

pub struct Engine {
    pub accounts: HashMap<u16, Account>,
    pub transactions: HashMap<u32, TransactionRecord>,
}

impl Default for Engine {
    fn default() -> Self {
        Self::new()
    }
}

impl Engine {
    pub fn new() -> Self {
        Self {
            accounts: HashMap::new(),
            transactions: HashMap::new(),
        }
    }

    pub fn process_transactions<T: TransactionSource>(&mut self, source: &mut T) {
        for transaction in source.transactions() {
            self.apply_transaction(transaction);
        }
    }

    pub fn apply_transaction(&mut self, transaction: Transaction) {
        match transaction.r#type {
            TransactionType::Deposit => {
                if let Some(amount) = transaction.amount {
                    self.handle_deposit(transaction.client, transaction.tx, amount);
                }
            }
            TransactionType::Withdrawal => {
                if let Some(amount) = transaction.amount {
                    self.handle_withdrawal(transaction.client, transaction.tx, amount);
                }
            }
            TransactionType::Dispute => {
                self.handle_dispute(transaction.client, transaction.tx);
            }
            TransactionType::Resolve => {
                self.handle_resolve(transaction.client, transaction.tx);
            }
            TransactionType::Chargeback => {
                self.handle_chargeback(transaction.client, transaction.tx);
            }
        }
    }

    // Generate a report of all accounts and their balances
    pub fn report(&self) {
        if !self.accounts.is_empty() {
            println!("client, available, held, total, locked");
            for (client_id, account) in &self.accounts {
                println!(
                    "{client_id}, {}, {}, {}, {}",
                    account.available,
                    account.held,
                    account.get_total(),
                    account.is_locked
                );
            }
        } else {
            println!("Engine has no accounts to report.");
        }
    }

    fn handle_deposit(&mut self, client: u16, tx: u32, amount: Decimal) {
        let account = self.accounts.entry(client).or_default();
        if account.deposit(tx, amount).is_ok() {
            // Record the transaction if deposit is successful
            self.transactions.insert(
                tx,
                TransactionRecord {
                    transaction: Transaction {
                        r#type: TransactionType::Deposit,
                        client,
                        tx,
                        amount: Some(amount),
                    },
                    dispute_state: DisputeState::None,
                },
            );
        }
    }

    fn handle_withdrawal(&mut self, client: u16, tx: u32, amount: Decimal) {
        if let Some(account) = self.accounts.get_mut(&client)
            && account.withdraw(tx, amount).is_ok()
        {
            self.transactions.insert(
                tx,
                TransactionRecord {
                    transaction: Transaction {
                        r#type: TransactionType::Withdrawal,
                        client,
                        tx,
                        amount: Some(amount),
                    },
                    dispute_state: DisputeState::None,
                },
            );
        }
    }
    fn handle_dispute(&mut self, client: u16, tx: u32) {
        todo!(
            "Handle dispute for client {} and transaction {}",
            client,
            tx
        );
    }

    fn handle_resolve(&mut self, client: u16, tx: u32) {
        todo!(
            "Handle resolve for client {} and transaction {}",
            client,
            tx
        );
    }

    fn handle_chargeback(&mut self, client: u16, tx: u32) {
        todo!(
            "Handle chargeback for client {} and transaction {}",
            client,
            tx
        );
    }
}
