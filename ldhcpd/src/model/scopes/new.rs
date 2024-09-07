use super::r#type::ScopeType;
use crate::model::SQLTransactionError;
use router::{
    sql::{Statement, Transaction},
    sqlite::SQLite3Transaction,
};

/// Creates a new scope type with `r#type` returning the id of the new scope
pub fn new_scope(
    transaction: &mut SQLite3Transaction,
    r#type: ScopeType,
) -> Result<usize, SQLTransactionError> {
    assert_ne!(r#type, ScopeType::Global);

    let mut statement = transaction.prepare("INSERT INTO scope (type) VALUES (?)")?;
    statement.bind(1, &r#type)?;
    statement.execute()?;

    transaction
        .last_insert_id()
        .ok_or("no last insert id".into())
}
