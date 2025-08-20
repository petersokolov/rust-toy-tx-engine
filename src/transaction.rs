use csv::{ReaderBuilder, StringRecord};
use rust_decimal::Decimal;
use serde::Deserialize;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

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
    pub tx: u32,
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

/// TransactionSource is a trait for types that can provide transactions.
/// Think about transactions coming from CSV files, network, or other sources.
pub trait TransactionSource {
    fn transactions(&mut self) -> Box<dyn Iterator<Item = Transaction>>;
}

/// CsvTransactionSource reads transactions from a CSV file.
pub struct CsvTransactionSource {
    pub path: String,
}

impl CsvTransactionSource {
    pub fn new(path: &str) -> Self {
        Self {
            path: path.to_string(),
        }
    }
}

/// Utility function to parse transactions from a CSV file, trimming whitespace from headers.
pub fn parse_transactions_with_trimmed_headers(path: &str) -> Vec<Transaction> {
    let file = File::open(path).expect("Failed to open CSV file");
    let mut reader = BufReader::new(file);

    // Read the first line (headers)
    let mut first_line = String::new();
    reader
        .read_line(&mut first_line)
        .expect("Failed to read header line");
    let headers: Vec<String> = first_line
        .split(',')
        .map(|h| h.trim().to_string())
        .collect();

    // Build a CSV reader with custom headers and trimming
    let mut rdr = ReaderBuilder::new()
        .has_headers(false)
        .trim(csv::Trim::All)
        .from_reader(reader);

    // Set the cleaned headers
    let header_record = StringRecord::from(headers);
    let records = rdr.records();

    // Iterator that deserializes each record
    let txs: Vec<Transaction> = records
        .map(|result| {
            let record = result.expect("Failed to read record");
            record
                .deserialize(Some(&header_record))
                .expect("Failed to parse transaction")
        })
        .collect();

    txs
}

impl TransactionSource for CsvTransactionSource {
    fn transactions(&mut self) -> Box<dyn Iterator<Item = Transaction>> {
        let txs = parse_transactions_with_trimmed_headers(&self.path);
        Box::new(txs.into_iter())
    }
}
