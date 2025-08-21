use rust_decimal::Decimal;
use rust_toy_tx_engine::{
    engine::Engine, run_engine_with_source, transaction::CsvTransactionSource,
};

#[test]
fn test_read_sample_csv() {
    let csv_path = "tests/sample.csv";
    let mut engine = Engine::new();
    let mut source = CsvTransactionSource::new(csv_path);
    run_engine_with_source(&mut engine, &mut source);
    assert!(
        !engine.accounts.is_empty(),
        "Engine should have processed accounts"
    );

    // assert account balances based on sample.csv
    let account1 = engine.accounts.get(&1).unwrap();
    assert_eq!(account1.get_available(), Decimal::new(15, 1));
    assert_eq!(account1.held, Decimal::new(0, 2));
    assert_eq!(account1.total, Decimal::new(15, 1));

    let account2 = engine.accounts.get(&2).unwrap();
    assert_eq!(account2.get_available(), Decimal::new(20, 1));
    assert_eq!(account2.held, Decimal::new(0, 2));
    assert_eq!(account2.total, Decimal::new(20, 1));
}
