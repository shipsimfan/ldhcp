use init_error::InitializationError;

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

    // Open the database and apply migrations

    // Start DHCP server on another thread

    // Start the huntsman server on this thread

    Ok(())
}
