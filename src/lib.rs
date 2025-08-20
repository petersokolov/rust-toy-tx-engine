mod account;
pub mod engine;
pub mod transaction;

use engine::Engine;
use transaction::TransactionSource;

/// Runs the engine with the provided transaction source.
pub fn run_engine_with_source<T: TransactionSource>(engine: &mut Engine, source: &mut T) {
    engine.process_transactions(source);
}
