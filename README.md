# rust-toy-tx-engine

### Assumptions and Design Decisions

1. **Total Balance Calculation**: The `total` balance is not stored as a separate field in the `Account` struct. Instead, it is calculated on demand as `available + held`. This decision was made to reduce redundancy and optimize for frequent deposit and withdrawal operations, assuming that `total` checks are less frequent. If this assumption changes, the design can be revisited.