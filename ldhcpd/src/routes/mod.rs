use huntsman_http::{HTTPResponse, HTTPStatus};

/// Routes a request to the correct endpoint
pub fn route<'a>() -> HTTPResponse<'a> {
    HTTPStatus::NotFound.into()
}
