use crate::database::UnknownField;
use huntsman_http::FromHTTPQueryParam;

use super::ReservationField;

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
            scope: false,
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

    /// Creates a new [`ReservationFields`] contaning only the "scope" field
    ///
    /// Used to get the scope associated with a reservation for getting the options
    pub fn new_scope() -> Self {
        let mut fields = ReservationFields::new_false();
        fields.scope = true;
        fields
    }
}

impl<'a> FromHTTPQueryParam<'a> for ReservationFields {
    type Error = UnknownField<'a>;

    fn from_query_param(query_param: &'a [u8]) -> Result<Self, Self::Error> {
        let mut fields = ReservationFields::new_false();

        let mut i = 0;
        while i < query_param.len() {
            let start = i;
            while i < query_param.len() && query_param[i] != b',' {
                i += 1;
            }

            let field = &query_param[start..i];
            match ReservationField::from_query_param(field)? {
                ReservationField::ID => fields.id = true,
                ReservationField::ClientID => fields.client_id = true,
                ReservationField::IPAddress => fields.ip_address = true,
                ReservationField::RenewalTime => fields.renewal_time = true,
                ReservationField::Description => fields.description = true,
            }

            i += 1;
        }

        Ok(fields)
    }
}
