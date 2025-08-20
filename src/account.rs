use rust_decimal::Decimal;

/// Account represents a user's account with available, held, and total balances.
#[derive(Debug, Clone)]
pub struct Account {
    pub client_id: u16,
    pub available: Decimal,
    pub held: Decimal,
    pub total: Decimal,
    pub is_locked: bool,
}

impl Account {
    pub fn new(client_id: u16) -> Self {
        Self {
            client_id,
            available: Decimal::ZERO,
            held: Decimal::ZERO,
            total: Decimal::ZERO,
            is_locked: false,
        }
    }
}
