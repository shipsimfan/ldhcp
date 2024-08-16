use crate::{Database, LDHCPOptions, LDHCPD};
use oak::{fatal, LogController};
use std::sync::Arc;

mod early;
mod error;

pub use error::CreationError;

/// Initializes the server
pub fn init() -> Result<Option<LDHCPD>, ()> {
    // Parse arguments
    let (options, log_controller) =
        match early::early_init().map_err(|error| eprintln!("Error: {}", error))? {
            Some(result) => result,
            None => return Ok(None),
        };

    // Run the server
    let logger = log_controller.create_logger("init");
    LDHCPD::new(options, log_controller)
        .map(|ldhcpd| Some(ldhcpd))
        .map_err(|error| fatal!(logger, "Unable to start the server - {}", error))
}

impl LDHCPD {
    /// Creates a new [`LDHCPD`] server from `options`
    pub(self) fn new(
        options: LDHCPOptions,
        log_controller: Arc<LogController>,
    ) -> Result<Self, CreationError> {
        // Open the database and apply migrations
        let database = Database::open(
            &log_controller,
            &options.database_path,
            &options.migration_path,
        )?;

        // Start DHCP server on another thread

        Ok(LDHCPD {
            log_controller,
            database,
        })
    }
}
