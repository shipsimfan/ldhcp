use crate::database::UnknownField;
use huntsman_http::FromHTTPQueryParam;

/// A field to order selection by
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ReservationField {
    #[default]
    ID,
    ClientID,
    IPAddress,
    RenewalTime,
    Description,
}

impl<'a> FromHTTPQueryParam<'a> for ReservationField {
    type Error = UnknownField<'a>;

    fn from_query_param(query_param: &'a [u8]) -> Result<Self, Self::Error> {
        Ok(match query_param {
            b"id" => ReservationField::ID,
            b"client_id" => ReservationField::ClientID,
            b"ip_address" => ReservationField::IPAddress,
            b"renewal_time" => ReservationField::RenewalTime,
            b"description" => ReservationField::Description,
            _ => return Err(UnknownField::new(query_param)),
        })
    }
}
