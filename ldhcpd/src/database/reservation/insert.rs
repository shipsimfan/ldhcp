use crate::{
    database::{insert_scope, Database, DatabaseError, ScopeType},
    model::{ClientID, Description},
};
use data_format::Deserialize;
use net_utils::ip::v4::IPv4Address;
use oak::info;
use sql::{Connection, Statement, Transaction};
use std::num::NonZeroUsize;

/// A new reservation requested to be made by a client
#[derive(Deserialize, Debug)]
pub struct NewReservation<'a> {
    /// The id of the client device this reservation is for
    client_id: ClientID,

    /// The ip address to be assigned to the client device
    ip_address: IPv4Address,

    /// A description of this reservation
    description: Option<Description<'a>>,

    /// The length of time a client can hold a reservation before renewing
    renewal_time: Option<NonZeroUsize>,
}

/// The SQL to insert a new reservation
const SQL: &str = include_str!("insert.sql");

/// Inserts a [`NewReservation`] into the database, returning it's id
pub fn insert_reservation(
    database: &Database,
    new_reservation: NewReservation,
) -> Result<usize, DatabaseError> {
    info!(database.logger, "Inserting new reservation");

    let transaction = database.connection.begin_trasaction()?;

    // Insert the scope
    insert_scope(&database.logger, &transaction, ScopeType::Reservation)?;

    // Insert the reservation
    let mut statement = transaction.prepare(SQL)?;
    statement.bind_blob(1, new_reservation.client_id.as_slice())?;
    statement.bind_u32(2, new_reservation.ip_address.to_bits())?;

    statement.bind_str(
        3,
        match &new_reservation.description {
            Some(description) => description.as_str(),
            None => "",
        },
    )?;

    statement.bind_usize(
        4,
        new_reservation
            .renewal_time
            .map(|renewal_time| renewal_time.get())
            .unwrap_or(0),
    )?;

    statement.execute()?;
    let id = transaction.last_insert_id().unwrap();

    transaction.commit()?;
    Ok(id)
}
