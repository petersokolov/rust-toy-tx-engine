# rust-toy-tx-engine

### Assumptions and Design Decisions

1. **Negative Balances**: Negative balances are possible if chargebacks or disputes remove more funds than are currently available. This matches real-world scenarios where clients can owe money after disputes.

1. **Locked Accounts**: Locked accounts disallow client-initiated transactions (deposit, withdrawal) but still process system reconciliation events (dispute, resolve, chargeback) so balances remain correct.
