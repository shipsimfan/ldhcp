use crate::{args::LDHCPOptions, LDHCP};
use oak::info;

mod error;

pub use error::CreationError;

impl LDHCP {
    /// Creates a new [`LDHCP`] server from `options`
    pub fn new(options: LDHCPOptions) -> Result<Self, CreationError> {
        // Create logger
        let log_controller = oak::LogController::new(
            "ldhcp",
            options.min_log_level,
            options.max_log_level,
            options.log_filter_type,
            options.log_filter,
            oak::StdLogOutput::convert_vec(options.log_outputs)
                .map_err(CreationError::OpenLogOutputFailed)?,
        )
        .map_err(CreationError::CreateLogControllerFailed)?;

        let logger = log_controller.create_logger("init");
        info!(logger, "Log controller created");

        // Open the database and apply migrations

        // Start DHCP server on another thread

        Ok(LDHCP { log_controller })
    }
}
