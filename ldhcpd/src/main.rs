use args::LDHCPOptions;
use database::Database;
use new::CreationError;
use oak::LogController;
use std::sync::Arc;

mod args;
mod database;
mod new;

/// The LDHCP Server
struct LDHCPD {
    /// The log controller for the server
    log_controller: Arc<LogController>,

    /// The database storing information for the server
    database: Database,
}

fn main() {
    if let Err(_) = run() {
        std::process::exit(1);
    }
}

fn run() -> Result<(), ()> {
    // Create the server object
    let ldhcp = new::init()?;

    // Start the huntsman server on this thread
    Ok(())
}
