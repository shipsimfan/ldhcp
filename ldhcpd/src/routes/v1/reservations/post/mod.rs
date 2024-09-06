use crate::{
    common::{scopes, SQLTransactionError},
    LDHCPD,
};
use new_reservation::NewReservation;
use router::{
    endpoint,
    oak::info,
    sql::{Connection, Statement, Transaction},
};

mod new_reservation;

#[endpoint(body = new_reservation)]
pub fn post(ldhcp: LDHCPD, new_reservation: NewReservation) -> Result<usize, SQLTransactionError> {
    info!(
        ldhcp.updates,
        "Creating new reservation: {:?}", new_reservation
    );

    let mut transaction = ldhcp.database.begin_trasaction()?;

    let scope_id = scopes::new_scope(&mut transaction, scopes::ScopeType::Reservation)?;

    let ip_address = new_reservation.ip_address.to_bits();
    let renewal_time = new_reservation
        .renewal_time
        .map(|val| val.get())
        .unwrap_or(0);

    let mut statement = transaction.prepare("INSERT INTO reservation (client_id, ip_address, scope, renewal_time, description) VALUES (?, ?, ?, ?, ?)")?;
    statement.bind(1, &new_reservation.client_id)?;
    statement.bind(2, &ip_address)?;
    statement.bind(3, &scope_id)?;
    statement.bind(4, &renewal_time)?;
    match &new_reservation.description {
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
