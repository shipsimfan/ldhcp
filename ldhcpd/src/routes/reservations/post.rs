use crate::{
    database::{self, NewReservation},
    util::validate_body,
    LDHCPD,
};
use huntsman_http::{HTTPRequest, HTTPResponse, HTTPStatus};
use oak::info;

/// Creates a new reservation based on the provided body
pub(super) fn post<'a>(app: &LDHCPD, request: &HTTPRequest) -> HTTPResponse<'a> {
    let reservation = match validate_body::<NewReservation>(request) {
        Ok(reservation) => reservation,
        Err(response) => return response,
    };

    info!(app.updates_logger, "Creating new reservation");

    match database::insert_reservation(&app.database, reservation) {
        Ok(row_id) => HTTPResponse::new(
            HTTPStatus::OK,
            json::to_bytes(&row_id).unwrap(),
            b"application/json",
        ),
        Err(error) => HTTPResponse::new(
            HTTPStatus::InternalServerError,
            json::to_bytes(&error.to_string()).unwrap(),
            b"application/json",
        ),
    }
}
