use crate::model::{scopes, ClientID, Description, SQLTransactionError};
use data_format::Deserialize;
use net_utils::ip::v4::IPv4Address;
use router::{
    sql::{Connection, Statement, Transaction},
    sqlite::SQLite3Connection,
    FromRequestBody,
};
use std::num::NonZeroUsize;

#[derive(Debug, Deserialize, FromRequestBody)]
pub struct NewReservation<'a> {
    pub client_id: ClientID,
    pub ip_address: IPv4Address,
    pub renewal_time: Option<NonZeroUsize>,
    pub description: Option<Description<'a>>,
}

const SQL: &str = include_str!("insert.sql");

impl<'a> NewReservation<'a> {
    /// Inserts the new reservation into the database, returning the id of the new reservation
    pub fn insert(self, db: &SQLite3Connection) -> Result<usize, SQLTransactionError> {
        let mut transaction = db.begin_trasaction()?;

        let scope_id = scopes::new_scope(&mut transaction, scopes::ScopeType::Reservation)?;

        let ip_address = self.ip_address.to_bits();
        let renewal_time = self.renewal_time.map(|val| val.get()).unwrap_or(0);

        let mut statement = transaction.prepare(SQL)?;
        statement.bind(1, &self.client_id)?;
        statement.bind(2, &ip_address)?;
        statement.bind(3, &scope_id)?;
        statement.bind(4, &renewal_time)?;
        match &self.description {
            Some(description) => statement.bind(5, description)?,
            None => statement.bind_str(5, "")?,
        };
        statement.execute()?;

        let id = transaction
            .last_insert_id()
            .ok_or(SQLTransactionError::Custom("no insert id for reservation"))?;

        transaction.commit()?;

        Ok(id)
    }
}
