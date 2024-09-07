use crate::{model::reservations::Reservation, LDHCPD};
use router::{endpoint, sqlite::SQLite3FromRowError};

#[endpoint]
pub fn get(ldhcpd: LDHCPD) -> Result<Reservation<'static>, SQLite3FromRowError> {
    todo!()
}
