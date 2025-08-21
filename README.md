# rust-toy-tx-engine

### Assumptions and Design Decisions

1. **Total Balance Calculation**: The `total` balance is not stored as a separate field in the `Account` struct. Instead, it is calculated on demand as `available + held`. This decision was made to reduce redundancy and optimize for frequent deposit and withdrawal operations, assuming that `total` checks are less frequent. If this assumption changes, the design can be revisited.

2. **Negative Balances**: Negative balances are possible if chargebacks or disputes remove more funds than are currently available. This matches real-world scenarios where clients can owe money after disputes.
