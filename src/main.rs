use clap::Clap;
use std::process;
use stoqed::{run, StoqedOptions};

fn main() {
    // Initialize logging
    pretty_env_logger::init();

    // Parse command line args.
    let opts: StoqedOptions = StoqedOptions::parse();

    // Let the library handle the actual application.
    if let Err(e) = run(opts) {
        log::error!("Application error: {}", e);
        process::exit(1);
    }
}
