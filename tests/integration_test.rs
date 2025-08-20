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
    for account in engine.accounts.values() {
        assert_eq!(account.get_total(), account.available + account.held);
    }
}
