use crate::{
    database::{get_many_reservations, ReservationFields, DEFAULT_RESERVATION_PAGE_SIZE},
    LDHCPD,
};
use huntsman_http::{FromHTTPQueryParam, HTTPQueryParam, HTTPResponse, HTTPStatus};
use std::borrow::Cow;

/// Gets a set of reservations based on the request
pub(super) fn get<'a>(app: &LDHCPD, query_params: &[HTTPQueryParam]) -> HTTPResponse<'a> {
    let mut page = None;
    let mut page_size = None;
    let mut order_by = None;
    let mut fields = None;
    for query_param in query_params {
        match match query_param.key() {
            b"page" => match page.is_some() {
                true => Err(Cow::Borrowed("Repeated field \"page\"")),
                false => usize::from_query_param(query_param.value())
                    .map(|new_page| {
                        page = Some(new_page);
                    })
                    .map_err(|error| error.to_string().into()),
            },
            b"page_size" => match page_size.is_some() {
                true => Err("Repeated field \"page_size\"".into()),
                false => todo!("NonZeroUsize::from_query_param"),
            },
            b"order_by" => todo!(),
            b"fields" => match fields.is_some() {
                true => Err("Repeated field \"fields\"".into()),
                false => ReservationFields::from_query_param(query_param.value())
                    .map(|new_fields| {
                        fields = Some(new_fields);
                    })
                    .map_err(|error| error.to_string().into()),
            },
            key => Err(format!(
                "invalid query parameter \"{}\"",
                String::from_utf8_lossy(key)
            )
            .into()),
        } {
            Ok(()) => {}
            Err(error) => {
                return HTTPResponse::new(
                    HTTPStatus::BadRequest,
                    json::to_bytes(&error).unwrap(),
                    b"application/json",
                )
            }
        }
    }

    match get_many_reservations(
        &app.database,
        page.unwrap_or(0),
        page_size.unwrap_or(DEFAULT_RESERVATION_PAGE_SIZE),
        order_by.unwrap_or_default(),
        fields.unwrap_or(ReservationFields::new_true()),
    ) {
        Ok(reservations) => HTTPResponse::new(
            HTTPStatus::OK,
            json::to_bytes(&reservations).unwrap(),
            b"application/json",
        ),
        Err(error) => HTTPResponse::new(
            HTTPStatus::InternalServerError,
            json::to_bytes(&error.to_string()).unwrap(),
            b"application/json",
        ),
    }
}
