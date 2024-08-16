use args::LDHCPOptions;
use database::Database;
use new::CreationError;
use oak::{LogController, Logger};
use std::sync::Arc;

mod app;
mod args;
mod database;
mod new;
mod routes;

/// The LDHCP Server
struct LDHCPD {
    /// The log controller for the server
    log_controller: Arc<LogController>,

    /// The database storing information for the server
    database: Database,

    /// The logger for initialization
    init_logger: Logger,

    /// The logger for connections
    connection_logger: Logger,

    /// The logger for requests
    request_logger: Logger,

    /// The logger for updates to the system
    updates_logger: Logger,

    /// Should request headers be logged?
    log_headers: bool,

    /// Should request bodies be logged?
    log_bodies: bool,

    /// Should response codes and paths be logged?
    log_reponses: bool,
}

fn main() {
    if let Err(_) = new::run() {
        std::process::exit(1);
    }
}
