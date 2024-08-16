use huntsman_http::{HTTPMethod, HTTPPath, HTTPRequest, HTTPResponse, HTTPStatus};

use crate::LDHCPD;

mod post;

/// Routes a reservation request to the correct endpoint
pub(super) fn route<'a>(
    app: &LDHCPD,
    request: &HTTPRequest,
    path: HTTPPath,
    segment: usize,
) -> HTTPResponse<'a> {
    match path.segment(segment) {
        None => {}
        _ => return HTTPStatus::NotFound.into(),
    }

    match request.method() {
        HTTPMethod::POST => post::post(app, request),
        _ => {
            let mut response: HTTPResponse = HTTPStatus::MethodNotAllowed.into();
            response.push_field(b"Allow", b"");
            response
        }
    }
}
