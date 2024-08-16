use new::CreationError;
use oak::{info, LogController};
use std::sync::Arc;

mod args;
mod new;

/// The LDHCP Server
struct LDHCP {
    log_controller: Arc<LogController>,
}

fn main() {
    if let Err(error) = run() {
        eprintln!("Error: {}", error);
        std::process::exit(1);
    }
}

fn run() -> Result<(), CreationError> {
    // Parse arguments
    let options = match args::parse()? {
        Some(options) => options,
        None => return Ok(()),
    };

    // Create the server object
    let ldhcp = LDHCP::new(options)?;

    // Start the huntsman server on this thread

    Ok(())
}
