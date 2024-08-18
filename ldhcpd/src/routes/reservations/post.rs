use crate::{
    model::{ClientID, Description},
    util::validate_body,
    LDHCPD,
};
use data_format::Deserialize;
use huntsman_http::{HTTPRequest, HTTPResponse, HTTPStatus};
use net_utils::ip::v4::IPv4Address;
use oak::info;
use std::num::NonZeroUsize;

/// A new reservation requested to be made by a client
#[derive(Deserialize, Debug)]
struct NewReservation<'a> {
    /// The id of the client device this reservation is for
    client_id: ClientID,

    /// The ip address to be assigned to the client device
    ip_address: IPv4Address,

    /// A description of this reservation
    description: Option<Description<'a>>,

    /// The length of time a client can hold a reservation before renewing
    renewal_time: Option<NonZeroUsize>,
}

/// Creates a new reservation based on the provided body
pub(super) fn post<'a>(app: &LDHCPD, request: &HTTPRequest) -> HTTPResponse<'a> {
    let reservation = match validate_body::<NewReservation>(request) {
        Ok(reservation) => reservation,
        Err(response) => return response,
    };

    info!(
        app.updates_logger,
        "Creating new reservation: {:?}", reservation
    );

    HTTPStatus::NotImplemented.into()
}
