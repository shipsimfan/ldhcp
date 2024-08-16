use crate::{args, CreationError};
use oak::LogController;
use std::sync::Arc;

/// Parse the arguments and create the log controller
pub(super) fn early_init() -> Result<Option<(args::LDHCPOptions, Arc<LogController>)>, CreationError> {
    // Parse arguments
    let mut options = match args::parse()? {
        Some(options) => options,
        None => return Ok(None),
    };

    // Take the log outputs and filters
    let mut log_outputs = Vec::new();
    std::mem::swap(&mut log_outputs, &mut options.log_outputs);

    let mut log_filters = Vec::new();
    std::mem::swap(&mut log_filters, &mut options.log_filter);

    // Create the log controller
    let log_controller = oak::LogController::new(
        "ldhcpd",
        options.min_log_level,
        options.max_log_level,
        options.log_filter_type,
        log_filters,
        oak::StdLogOutput::convert_vec(log_outputs)?,
    )
    .map_err(CreationError::CreateLogControllerFailed)?;

    Ok(Some((options, log_controller)))
}
