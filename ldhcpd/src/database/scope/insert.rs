use super::ScopeType;
use crate::database::DatabaseError;
use oak::{info, Logger};
use sql::{Statement, Transaction};
use sqlite::SQLite3Transaction;

/// The SQL to insert a new scope
const SQL: &str = include_str!("insert.sql");

pub fn insert_scope(
    logger: &Logger,
    transaction: &SQLite3Transaction,
    r#type: ScopeType,
) -> Result<(), DatabaseError> {
    info!(logger, "Inserting new scope of type {}", r#type);

    let mut statement = transaction.prepare(SQL)?;
    statement.bind_usize(1, r#type as usize)?;
    Ok(statement.execute()?)
}
