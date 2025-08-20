use log::{info, warn};
use rust_decimal::Decimal;
use thiserror::Error;

/// Account represents a user's account with available, held, and total balances.
#[derive(Debug, Clone)]
pub struct Account {
    // Unique identifier for the client
    pub client_id: u16,
    // Available funds
    pub available: Decimal,
    // Held funds (e.g., in dispute)
    pub held: Decimal,
    // Indicates if the account is locked
    pub is_locked: bool,
}

impl Account {
    pub fn new(client_id: u16) -> Self {
        Self {
            client_id,
            available: Decimal::ZERO,
            held: Decimal::ZERO,
            is_locked: false,
        }
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
        self.available += amount;
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
        if self.available < amount {
            warn!(
                "Withdrawal of {} for client {} (tx {}) not processed. Insufficient funds.",
                amount, self.client_id, tx
            );
            return Err(AccountError::InsufficientFunds(self.client_id));
        }
        self.available -= amount;
        info!(
            "Withdrawal of {} for client {} (tx {}) processed.",
            amount, self.client_id, tx
        );
        Ok(())
    }

    pub fn get_total(&self) -> Decimal {
        self.available + self.held
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
        assert_eq!(account.available, deposit_amount);
        assert_eq!(account.get_total(), deposit_amount);
    }

    #[test]
    fn test_withdrawal() {
        let mut account = Account::new(1);
        let deposit_amount = Decimal::new(100, 2);
        let withdrawal_amount = Decimal::new(50, 2);

        account.deposit(1, deposit_amount).unwrap();
        assert!(account.withdraw(2, withdrawal_amount).is_ok());
        assert_eq!(account.available, deposit_amount - withdrawal_amount);
        assert_eq!(account.get_total(), deposit_amount - withdrawal_amount);
    }

    #[test]
    fn test_insufficient_funds() {
        let mut account = Account::new(1);
        let deposit_amount = Decimal::new(100, 2);
        let withdrawal_amount = Decimal::new(50, 2);

        account.deposit(1, deposit_amount).unwrap();
        let result = account.withdraw(3, deposit_amount + withdrawal_amount);
        assert!(result.is_err());
        assert_eq!(account.available, deposit_amount);
        assert_eq!(account.get_total(), deposit_amount);
    }
}
