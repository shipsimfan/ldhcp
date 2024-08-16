use crate::LDHCPD;
use huntsman_http::{HTTPPath, HTTPRequest, HTTPResponse, HTTPStatus};

mod reservations;

/// Routes a request to the correct endpoint
pub fn route<'a>(
    app: &LDHCPD,
    request: &HTTPRequest,
    path: HTTPPath,
    segment: usize,
) -> HTTPResponse<'a> {
    match path.segment(0) {
        Some(b"reservations") => reservations::route(app, request, path, segment + 1),
        _ => HTTPStatus::NotFound.into(),
    }
}
