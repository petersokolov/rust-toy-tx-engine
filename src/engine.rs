use std::collections::HashMap;

use log::{info, warn};
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
                    account.get_available(),
                    account.held,
                    account.total,
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

    // In the envent of dispute, client claims that a transaction was erroneous and should be reversed.
    // Clients available funds should be decreased by teh amount disputed, their held funds should
    // increase by the amount disputed, while their total funds should remain the same
    fn handle_dispute(&mut self, client: u16, tx: u32) {
        if let Some(record) = self.transactions.get_mut(&tx) {
            if record.dispute_state == DisputeState::None {
                if let Some(account) = self.accounts.get_mut(&client) {
                    if let Some(amount) = record.transaction.amount {
                        match account.dispute(amount) {
                            Ok(_) => {
                                record.dispute_state = DisputeState::Disputed;
                                info!(
                                    "Dispute of {} for client {} processed. Held funds updated to {}.",
                                    amount, client, account.held
                                );
                            }
                            Err(e) => {
                                warn!("Failed to process dispute for transaction {}: {}", tx, e);
                            }
                        }
                    } else {
                        warn!(
                            "Transaction {} for client {} has no associated amount to dispute.",
                            tx, client
                        );
                    }
                } else {
                    warn!("Client {} not found for transaction {}.", client, tx);
                }
            } else {
                warn!(
                    "Transaction {} for client {} is already in dispute.",
                    tx, client
                );
            }
        } else {
            warn!("Transaction {} not found for client {}.", tx, client);
        }
    }

    /// A resolve represents a resolution to a dispute, releasing the assotiated held funds. Funds that were
    /// previously disputed and no longer disputed. Held funds should be decreased by the disputed amount.
    /// Total should remain the same.
    fn handle_resolve(&mut self, client: u16, tx: u32) {
        if let Some(record) = self.transactions.get_mut(&tx) {
            if record.dispute_state == DisputeState::Disputed {
                if let Some(account) = self.accounts.get_mut(&client) {
                    if let Some(amount) = record.transaction.amount {
                        match account.resolve(amount) {
                            Ok(_) => {
                                record.dispute_state = DisputeState::Resolved;
                                info!(
                                    "Resolve of {} for client {} processed. Held funds updated to {}.",
                                    amount, client, account.held
                                );
                            }
                            Err(e) => {
                                warn!("Failed to process resolve for transaction {}: {}", tx, e);
                            }
                        }
                    } else {
                        warn!(
                            "Transaction {} for client {} has no associated amount to resolve.",
                            tx, client
                        );
                    }
                } else {
                    warn!("Client {} not found for transaction {}.", client, tx);
                }
            } else {
                warn!(
                    "Transaction {} for client {} is not in dispute.",
                    tx, client
                );
            }
        } else {
            warn!("Transaction {} not found for client {}.", tx, client);
        }
    }

    /// A chargeback is the final state of a dispute and represents the client reversing a transaction.
    /// Funds that were held have now been withdrawn. This means that the clients fheld funds and total funds
    /// should decreaseby the amount previously disputed.
    /// If a chargeback occurs the client account should be immediately frozen.
    fn handle_chargeback(&mut self, client: u16, tx: u32) {
        if let Some(record) = self.transactions.get_mut(&tx) {
            if record.dispute_state == DisputeState::Disputed {
                if let Some(account) = self.accounts.get_mut(&client) {
                    if let Some(amount) = record.transaction.amount {
                        match account.chargeback(amount) {
                            Ok(_) => {
                                record.dispute_state = DisputeState::ChargedBack;
                                info!(
                                    "Chargeback of {} for client {} processed. Account locked.",
                                    amount, client
                                );
                            }
                            Err(e) => {
                                warn!("Failed to process chargeback for transaction {}: {}", tx, e);
                            }
                        }
                    } else {
                        warn!(
                            "Transaction {} for client {} has no associated amount to chargeback.",
                            tx, client
                        );
                    }
                } else {
                    warn!("Client {} not found for transaction {}.", client, tx);
                }
            } else {
                warn!(
                    "Transaction {} for client {} is not in dispute.",
                    tx, client
                );
            }
        } else {
            warn!("Transaction {} not found for client {}.", tx, client);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal::Decimal;

    fn setup_engine_with_deposit(client_id: u16, tx_id: u32, amount: Decimal) -> Engine {
        let mut engine = Engine::new();
        engine.handle_deposit(client_id, tx_id, amount);
        engine
    }

    #[test]
    fn test_handle_dispute() {
        let client_id = 1;
        let tx_id = 1001;
        let deposit_amount = Decimal::new(100, 2); // 1.00

        let mut engine = setup_engine_with_deposit(client_id, tx_id, deposit_amount);

        // Dispute the transaction
        engine.handle_dispute(client_id, tx_id);
        let account = engine.accounts.get(&client_id).unwrap();
        assert_eq!(account.held, deposit_amount);
        assert_eq!(account.get_available(), Decimal::ZERO);

        // Verify transaction state
        let transaction = engine.transactions.get(&tx_id).unwrap();
        assert_eq!(transaction.dispute_state, DisputeState::Disputed);
    }

    #[test]
    fn test_handle_resolve() {
        let client_id = 1;
        let tx_id = 1001;
        let deposit_amount = Decimal::new(100, 2); // 1.00

        let mut engine = setup_engine_with_deposit(client_id, tx_id, deposit_amount);

        // Dispute the transaction
        engine.handle_dispute(client_id, tx_id);

        // Resolve the dispute
        engine.handle_resolve(client_id, tx_id);
        let account = engine.accounts.get(&client_id).unwrap();
        assert_eq!(account.held, Decimal::ZERO);
        assert_eq!(account.get_available(), deposit_amount);

        // Verify transaction state
        let transaction = engine.transactions.get(&tx_id).unwrap();
        assert_eq!(transaction.dispute_state, DisputeState::Resolved);
    }

    #[test]
    fn test_handle_chargeback() {
        let client_id = 1;
        let tx_id = 1001;
        let deposit_amount = Decimal::new(100, 2); // 1.00

        let mut engine = setup_engine_with_deposit(client_id, tx_id, deposit_amount);

        // Dispute the transaction
        engine.handle_dispute(client_id, tx_id);
        let account = engine.accounts.get(&client_id).unwrap();
        assert_eq!(account.held, deposit_amount);
        assert_eq!(account.get_available(), Decimal::ZERO);

        // Chargeback the transaction
        engine.handle_chargeback(client_id, tx_id);
        let account = engine.accounts.get(&client_id).unwrap();
        assert_eq!(account.total, Decimal::ZERO);
        assert_eq!(account.held, Decimal::ZERO);
        assert!(account.is_locked);

        // Verify transaction state
        let transaction = engine.transactions.get(&tx_id).unwrap();
        assert_eq!(transaction.dispute_state, DisputeState::ChargedBack);

        // Edge case: Chargeback a non-existent transaction
        let non_existent_tx_id = 9999;
        engine.handle_chargeback(client_id, non_existent_tx_id);
        // No panic or crash expected, just a warning log

        // Edge case: Chargeback a transaction not in dispute
        engine.handle_chargeback(client_id, tx_id);
        // No state change expected, just a warning log
    }
}
