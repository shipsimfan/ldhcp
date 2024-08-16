use crate::{Database, LDHCPOptions, LDHCPD};
use oak::{fatal, LogController, Logger};
use std::sync::Arc;

mod early;
mod error;

pub use error::CreationError;

/// Initializes and runs the server
pub fn run() -> Result<(), ()> {
    // Parse arguments
    let (options, log_controller) =
        match early::early_init().map_err(|error| eprintln!("Error: {}", error))? {
            Some(result) => result,
            None => return Ok(()),
        };

    // Run the server
    let logger = log_controller.create_logger("init");
    let ldhcpd = LDHCPD::new(&options, log_controller, &logger)
        .map_err(|error| fatal!(logger, "Unable to start the server - {}", error))?;

    huntsman::run(ldhcpd, options.huntsman_options, options.http_options)
        .map_err(|error| fatal!(logger, "Unable to start the server - {}", error))
}

impl LDHCPD {
    /// Creates a new [`LDHCPD`] server from `options`
    pub(self) fn new(
        options: &LDHCPOptions,
        log_controller: Arc<LogController>,
        init_logger: &Logger,
    ) -> Result<Self, CreationError> {
        // Open the database and apply migrations
        let database = Database::open(
            &log_controller,
            &options.database_path,
            &options.migration_path,
        )?;

        // Start DHCP server on another thread

        // Create huntsman loggers
        let connection_logger = log_controller.create_logger("connections");
        let request_logger = log_controller.create_logger("requests");

        Ok(LDHCPD {
            log_controller,
            database,
            init_logger: init_logger.clone(),
            connection_logger,
            request_logger,
            log_bodies: options.log_bodies,
            log_headers: options.log_headers,
            log_reponses: options.log_reponses,
        })
    }
}
