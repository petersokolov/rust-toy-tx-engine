use rust_decimal::Decimal;
use serde::Deserialize;

// TransactionType defines the type of transaction.
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum TransactionType {
    Deposit,
    Withdrawal,
    Dispute,
    Resolve,
    Chargeback,
}

/// Transaction stores information about a financial transaction.
/// amount is Optional. Only present for deposit/withdrawal
#[derive(Debug, Deserialize, Clone)]
pub struct Transaction {
    pub r#type: TransactionType, // `r#type` since "type" is reserved
    pub client: u16,
    pub id: u32,
    pub amount: Option<Decimal>,
}

/// DisputeState represents the state of a transaction in a dispute.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DisputeState {
    None,
    Disputed,
    Resolved,
    ChargedBack,
}

/// TransactionRecord combines a Transaction with its dispute state for storage.
#[derive(Debug, Clone)]
pub struct TransactionRecord {
    pub transaction: Transaction,
    pub dispute_state: DisputeState,
}
