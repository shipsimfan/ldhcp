use std::num::NonZeroUsize;

use crate::{
    database::{
        get_many_reservations, OrderBy, ReservationField, ReservationFields, ReservationPageSize,
        DEFAULT_RESERVATION_PAGE_SIZE,
    },
    LDHCPD,
};
use huntsman_http::{HTTPQueryParam, HTTPResponse, HTTPStatus};

/// Gets a set of reservations based on the request
pub(super) fn get<'a>(app: &LDHCPD, query_params: &[HTTPQueryParam]) -> HTTPResponse<'a> {
    let mut page = None;
    let mut page_size = None;
    let mut order_by = None;
    let mut fields = None;
    for query_param in query_params {
        let value = match std::str::from_utf8(query_param.value()) {
            Ok(value) => value,
            Err(_) => return HTTPStatus::BadRequest.into(),
        };

        match query_param.key() {
            b"page" => {
                if page.is_some() {
                    return HTTPStatus::BadRequest.into();
                }

                match usize::from_str_radix(value, 10) {
                    Ok(new_page) => page = Some(new_page),
                    Err(_) => return HTTPStatus::BadRequest.into(),
                }
            }
            b"page_size" => {
                if page_size.is_some() {
                    return HTTPStatus::BadRequest.into();
                }

                match usize::from_str_radix(value, 10) {
                    Ok(new_page_size) => match NonZeroUsize::new(new_page_size) {
                        Some(new_page_size) => match ReservationPageSize::new(new_page_size) {
                            Some(new_page_size) => page_size = Some(new_page_size),
                            None => return HTTPStatus::BadRequest.into(),
                        },
                        None => return HTTPStatus::BadRequest.into(),
                    },
                    Err(_) => return HTTPStatus::BadRequest.into(),
                }
            }
            b"order_by" => todo!(),
            b"fields" => todo!(),
            _ => return HTTPStatus::BadRequest.into(),
        }
    }

    match get_many_reservations(
        &app.database,
        page.unwrap_or(0),
        page_size.unwrap_or(DEFAULT_RESERVATION_PAGE_SIZE),
        order_by.unwrap_or((OrderBy::Ascending, ReservationField::ID)),
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
