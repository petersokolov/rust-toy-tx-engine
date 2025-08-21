use log::{info, warn};
use rust_decimal::Decimal;
use thiserror::Error;

/// Account represents a user's account with total, held, and calculated available balances.
#[derive(Debug, Clone)]
pub struct Account {
    // Unique identifier for the client
    pub client_id: u16,
    // Total funds (available + held)
    pub total: Decimal,
    // Held funds (e.g., in dispute)
    pub held: Decimal,
    // Indicates if the account is locked
    pub is_locked: bool,
}

impl Account {
    pub fn new(client_id: u16) -> Self {
        Self {
            client_id,
            total: Decimal::ZERO,
            held: Decimal::ZERO,
            is_locked: false,
        }
    }

    /// Calculate available funds as total - held.
    pub fn get_available(&self) -> Decimal {
        self.total - self.held
    }

    // Deposit funds into the account. If account is locked, the deposit should not be processed.
    pub fn deposit(&mut self, tx: u32, amount: Decimal) -> Result<(), AccountError> {
        if self.is_locked {
            warn!(
                "Account {} is locked. Deposit of {} for tx {} not processed.",
                self.client_id, amount, tx
            );
            return Err(AccountError::AccountLocked(self.client_id));
        }
        self.total += amount;
        info!(
            "Deposit of {} for client {} (tx {}) processed.",
            amount, self.client_id, tx
        );
        Ok(())
    }

    /// Withdraw funds from the account. If account is locked, the withdrawal should not be processed.
    pub fn withdraw(&mut self, tx: u32, amount: Decimal) -> Result<(), AccountError> {
        if self.is_locked {
            warn!(
                "Account {} is locked. Withdrawal of {} for tx {} not processed.",
                self.client_id, amount, tx
            );
            return Err(AccountError::AccountLocked(self.client_id));
        }
        if self.get_available() < amount {
            warn!(
                "Withdrawal of {} for client {} (tx {}) not processed. Insufficient funds. Available: {}, Held: {}",
                amount,
                self.client_id,
                tx,
                self.get_available(),
                self.held
            );
            return Err(AccountError::InsufficientFunds(self.client_id));
        }
        self.total -= amount;
        info!(
            "Withdrawal of {} for client {} (tx {}) processed.",
            amount, self.client_id, tx
        );
        Ok(())
    }

    /// Dispute a transaction by increasing held funds for the account.
    pub fn dispute(&mut self, amount: Decimal) -> Result<(), AccountError> {
        self.held += amount;
        Ok(())
    }

    /// Resolve a dispute by releasing the held funds.
    pub fn resolve(&mut self, amount: Decimal) -> Result<(), AccountError> {
        self.held -= amount;
        Ok(())
    }

    /// Chargeback a transaction by withdrawing held funds for the account.
    /// Total should be reduced by amount and account should be locked.
    pub fn chargeback(&mut self, amount: Decimal) -> Result<(), AccountError> {
        self.held -= amount;
        self.total -= amount;
        self.is_locked = true;
        Ok(())
    }
}

impl Default for Account {
    fn default() -> Self {
        Self::new(0)
    }
}

/// AccountError represents errors that can occur during account operations.
#[derive(Debug, Error)]
pub enum AccountError {
    #[error("Account {0} is locked.")]
    AccountLocked(u16),
    #[error("Insufficient funds for client {0}.")]
    InsufficientFunds(u16),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deposit() {
        let mut account = Account::new(1);
        let deposit_amount = Decimal::new(100, 2);

        assert!(account.deposit(1, deposit_amount).is_ok());
        assert_eq!(account.get_available(), deposit_amount);
        assert_eq!(account.total, deposit_amount);
    }

    #[test]
    fn test_withdrawal() {
        let mut account = Account::new(1);
        let deposit_amount = Decimal::new(100, 2);
        let withdrawal_amount = Decimal::new(50, 2);

        account.deposit(1, deposit_amount).unwrap();
        assert!(account.withdraw(2, withdrawal_amount).is_ok());
        assert_eq!(account.get_available(), deposit_amount - withdrawal_amount);
        assert_eq!(account.total, deposit_amount - withdrawal_amount);
    }

    #[test]
    fn test_insufficient_funds() {
        let mut account = Account::new(1);
        let deposit_amount = Decimal::new(100, 2);
        let withdrawal_amount = Decimal::new(50, 2);

        account.deposit(1, deposit_amount).unwrap();
        let result = account.withdraw(3, deposit_amount + withdrawal_amount);
        assert!(result.is_err());
        assert_eq!(account.get_available(), deposit_amount);
        assert_eq!(account.total, deposit_amount);
    }

    #[test]
    fn test_dispute() {
        let mut account = Account::new(1);
        let deposit_amount = Decimal::new(100, 2);
        let dispute_amount1 = Decimal::new(30, 2);
        let dispute_amount2 = Decimal::new(80, 2);

        account.deposit(1, deposit_amount).unwrap();

        let result = account.dispute(dispute_amount1);
        assert!(result.is_ok());
        assert_eq!(account.held, dispute_amount1);
        assert_eq!(account.get_available(), deposit_amount - dispute_amount1);

        let result = account.dispute(dispute_amount2);
        assert!(result.is_ok());
        assert_eq!(account.held, dispute_amount1 + dispute_amount2);
        assert_eq!(
            account.get_available(),
            deposit_amount - dispute_amount1 - dispute_amount2
        );
        assert_eq!(account.total, deposit_amount);
        assert_eq!(account.get_available(), Decimal::new(-10, 2));
        assert_eq!(account.is_locked, false);
    }

    #[test]
    fn test_resolve() {
        let mut account = Account::new(1);
        let deposit_amount = Decimal::new(100, 2);
        let dispute_amount = Decimal::new(30, 2);

        account.deposit(1, deposit_amount).unwrap();
        account.dispute(dispute_amount).unwrap();

        let result = account.resolve(dispute_amount);
        assert!(result.is_ok());
        assert_eq!(account.held, Decimal::ZERO);
        assert_eq!(account.get_available(), deposit_amount);
    }

    #[test]
    fn test_chargeback() {
        let mut account = Account::new(1);
        let deposit_amount = Decimal::new(100, 2);
        let dispute_amount = Decimal::new(50, 2);

        account.total = deposit_amount;
        account.held = dispute_amount;

        assert!(account.chargeback(dispute_amount).is_ok());

        assert_eq!(account.total, deposit_amount - dispute_amount);
        assert_eq!(account.held, Decimal::ZERO);
        assert!(account.is_locked);
    }
}
