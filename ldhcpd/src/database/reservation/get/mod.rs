use crate::database::{page_size::PageSize, Database, DatabaseError, OrderBy};
use std::num::NonZeroUsize;

mod field;
mod fields;
mod result;

pub use field::ReservationField;
pub use fields::ReservationFields;
pub use result::Reservation;

/// A size of a requested reservation page
pub type ReservationPageSize = PageSize<100>;

/// The default [`ReservationPageSize`] if none is provided
pub const DEFAULT_RESERVATION_PAGE_SIZE: ReservationPageSize =
    unsafe { ReservationPageSize::new_unchecked(NonZeroUsize::new_unchecked(25)) };

/// Gets a set of reservations matching the provided parameters
pub fn get_many_reservations(
    db: &Database,
    page: usize,
    page_size: ReservationPageSize,
    order_by: OrderBy<ReservationField>,
    fields: ReservationFields,
) -> Result<Vec<Reservation>, DatabaseError> {
    todo!()
}

/// Gets a reservation from the database
pub fn get_reservation(
    db: &Database,
    id: usize,
    fields: ReservationFields,
) -> Result<Reservation, DatabaseError> {
    todo!()
}
