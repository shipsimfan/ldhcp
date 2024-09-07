use crate::{
    model::{reservations::NewReservation, SQLTransactionError},
    LDHCPD,
};
use router::{endpoint, oak::info};

#[endpoint(body = new_reservation)]
pub fn post(ldhcpd: LDHCPD, new_reservation: NewReservation) -> Result<usize, SQLTransactionError> {
    info!(
        ldhcpd.updates,
        "Creating new reservation: {:?}", new_reservation
    );

    new_reservation.insert(&ldhcpd.database)
}
