use crate::CreationError;
use oak::{info, LogController, Logger};
use sqlite::SQLite3Connection;
use std::{path::Path, sync::Arc};

mod error;
mod order_by;
mod unknown_field;

mod page_size;
mod reservation;
mod scope;

pub use error::DatabaseError;
pub use order_by::{OrderBy, OrderByDirection};
pub use unknown_field::UnknownField;

pub use reservation::*;
pub use scope::*;

/// The global storage for this server
pub struct Database {
    /// The active connection
    connection: SQLite3Connection,

    /// The logger for database events
    logger: Logger,
}

impl Database {
    /// Open the [`Database`] and apply migrations
    pub fn open(
        log_controller: &Arc<LogController>,
        database_path: &Path,
        migration_path: &Path,
    ) -> Result<Self, CreationError> {
        // Create the logger
        let logger = log_controller.create_logger("database");

        // Open the database
        info!(logger, "Opening \"{}\"", database_path.display());
        let connection = SQLite3Connection::open(database_path).map_err(|error| {
            CreationError::OpenDatabaseFailed(error, database_path.to_path_buf())
        })?;

        // Apply migrations
        let applied_migrations =
            sql_migrations::migrate(&connection, migration_path).map_err(|error| {
                CreationError::MigrateDatabaseFailed(error, migration_path.to_path_buf())
            })?;
        for migration in applied_migrations.up() {
            info!(logger, "Applied up migration \"{}\"", migration.name());
        }
        for migration in applied_migrations.down() {
            info!(logger, "Applied down migration \"{}\"", migration.name());
        }

        Ok(Database { connection, logger })
    }
}
