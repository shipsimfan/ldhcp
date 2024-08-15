use init_error::InitializationError;
use oak::info;

mod args;
mod init_error;

fn main() {
    if let Err(error) = run() {
        eprintln!("Error: {}", error);
        std::process::exit(1);
    }
}

fn run() -> Result<(), InitializationError> {
    // Parse arguments
    let options = match args::parse()? {
        Some(options) => options,
        None => return Ok(()),
    };

    // Create logger
    let log_controller = oak::LogController::new(
        "ldhcp",
        options.min_log_level,
        options.max_log_level,
        options.log_filter_type,
        options.log_filter,
        oak::StdLogOutput::convert_vec(options.log_outputs)
            .map_err(InitializationError::OpenLogOutputFailed)?,
    )
    .map_err(InitializationError::CreateLogControllerFailed)?;

    let logger = log_controller.create_logger("init");
    info!(logger, "Log controller created");

    // Open the database and apply migrations

    // Start DHCP server on another thread

    // Start the huntsman server on this thread

    Ok(())
}
