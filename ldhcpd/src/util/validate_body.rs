use data_format::Deserialize;
use huntsman_http::{HTTPRequest, HTTPResponse, HTTPStatus};

/// Validates a body is json and deserializes it into the requested type, returning an appropriate
/// response if it fails
pub fn validate_body<'a, 'b, T: Deserialize<'a>>(
    request: &'a HTTPRequest<'a>,
) -> Result<T, HTTPResponse<'b>> {
    let body = match request.body() {
        Some(body) => body,
        None => return Err(HTTPStatus::BadRequest.into()),
    };

    if body.len() == 0
        || request
            .field(b"Content-Type")
            .map(|content_type| content_type.value() != b"application/json")
            .unwrap_or(true)
    {
        return Err(HTTPStatus::BadRequest.into());
    }

    json::from_bytes(body).map_err(|error| {
        HTTPResponse::new(
            HTTPStatus::BadRequest,
            json::to_bytes(&error.to_string()).unwrap(),
            b"application/json",
        )
    })
}
