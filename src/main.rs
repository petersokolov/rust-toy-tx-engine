use log::info;

use rust_toy_tx_engine::{
    engine::Engine, run_engine_with_source, transaction::CsvTransactionSource,
};

fn main() {
    env_logger::init();

    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: cargo run -- <input.csv>");
        std::process::exit(1);
    }

    // TODO: handle path normalization (expand ~, check canonical form)
    let input_file = &args[1];
    info!("Input file: {}", input_file);

    let mut engine = Engine::new();
    let mut source = CsvTransactionSource::new(input_file);
    run_engine_with_source(&mut engine, &mut source);

    info!("Generating report...");
    engine.report();
}
