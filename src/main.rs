use log::info;

fn main() {
    env_logger::init();

    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: cargo run -- <input.csv>");
        std::process::exit(1);
    }

    let input_file = &args[1];
    info!("Input file: {}", input_file);
}
