/// The fields to select from the database
pub struct ReservationFields {
    id: bool,
    client_id: bool,
    ip_address: bool,
    renewal_time: bool,
    description: bool,
    scope: bool,
}

impl ReservationFields {
    /// Creates a new [`ReservationFields`] containing all fields
    pub fn new_true() -> Self {
        ReservationFields {
            id: true,
            client_id: true,
            ip_address: true,
            renewal_time: true,
            description: true,
            scope: true,
        }
    }

    /// Creates a new [`ReservationFields`] containing no fields
    pub fn new_false() -> Self {
        ReservationFields {
            id: false,
            client_id: false,
            ip_address: false,
            renewal_time: false,
            description: false,
            scope: false,
        }
    }
}
