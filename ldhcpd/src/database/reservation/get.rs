use crate::{
    database::{page_size::PageSize, DatabaseError, OrderBy},
    model::{ClientID, Description},
};
use data_format::{MapSerializer, Serialize};
use net_utils::ip::v4::IPv4Address;
use sql::{Column, FromColumn, FromRow};
use sqlite::SQLite3Connection;
use std::num::NonZeroUsize;

/// A size of a requested reservation page
pub type ReservationPageSize = PageSize<100>;

/// A reservation to be read from the database
pub struct Reservation {
    id: Option<usize>,
    client_id: Option<ClientID>,
    ip_address: Option<IPv4Address>,
    renewal_time: Option<Option<NonZeroUsize>>,
    description: Option<Option<Description<'static>>>,
    scope: Option<usize>,
}

/// The fields to select from the database
pub struct ReservationFields {
    id: bool,
    client_id: bool,
    ip_address: bool,
    renewal_time: bool,
    description: bool,
    scope: bool,
}

/// A field to order selection by
pub enum ReservationField {
    ID,
    ClientID,
    IPAddress,
    RenewalTime,
    Description,
    Scope,
}

/// The default [`ReservationPageSize`] if none is provided
const DEFAULT_PAGE_SIZE: ReservationPageSize =
    unsafe { ReservationPageSize::new_unchecked(NonZeroUsize::new_unchecked(25)) };

/// Gets a set of reservations matching the provided parameters
pub fn get_many_reservations(
    db: &SQLite3Connection,
    page: usize,
    page_size: Option<ReservationPageSize>,
    order_by: Option<(OrderBy, ReservationField)>,
    fields: Option<ReservationFields>,
) -> Result<Vec<Reservation>, DatabaseError> {
    let page_size = page_size.unwrap_or(DEFAULT_PAGE_SIZE);
    let (order_by, order_by_field) = order_by.unwrap_or((OrderBy::Ascending, ReservationField::ID));
    let fields = fields.unwrap_or(ReservationFields::new_true());

    todo!()
}

/// Gets a reservation from the database
pub fn get_reservation(
    db: &SQLite3Connection,
    id: usize,
    fields: ReservationFields,
) -> Result<Reservation, DatabaseError> {
    todo!()
}

impl Serialize for Reservation {
    fn serialize<S: data_format::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(5))?;

        if let Some(id) = self.id {
            map.serialize_entry("id", &id)?;
        }

        if let Some(client_id) = &self.client_id {
            map.serialize_entry("client_id", client_id)?;
        }

        if let Some(ip_address) = self.ip_address {
            map.serialize_entry("ip_address", &ip_address)?;
        }

        if let Some(renewal_time) = self.renewal_time {
            match renewal_time {
                Some(renewal_time) => map.serialize_entry("renewal_time", &renewal_time),
                None => map.serialize_entry("renewal_time", &()),
            }?;
        }

        if let Some(description) = &self.description {
            match description {
                Some(description) => map.serialize_entry("description", description),
                None => map.serialize_entry("description", &()),
            }?;
        }

        if let Some(scope) = &self.scope {
            map.serialize_entry("scope", scope)?;
        }

        map.end()
    }
}

impl FromRow for Reservation {
    fn from_row<'a, R: sql::Row<'a>>(row: R) -> Result<Self, R::Error> {
        let mut id = None;
        let mut client_id = None;
        let mut ip_address = None;
        let mut renewal_time = None;
        let mut description = None;
        let mut scope = None;

        for column in row {
            let column = column?;
            match column.name()?.as_str() {
                "id" => id = Some(column.into_usize()?),
                "client_id" => client_id = Some(ClientID::from_column(column)?),
                "ip_address" => ip_address = Some(IPv4Address::from_bits(column.into_u32()?)),
                "renewal_time" => renewal_time = Some(NonZeroUsize::new(column.into_usize()?)),
                "description" => {
                    description = Some({
                        let description = Description::from_column(column)?;
                        if description.as_str().len() == 0 {
                            None
                        } else {
                            Some(description)
                        }
                    })
                }
                "scope" => scope = Some(column.into_usize()?),
                field => panic!("Unknown field from database: \"{}\"", field),
            }
        }

        Ok(Reservation {
            id,
            client_id,
            ip_address,
            renewal_time,
            description,
            scope,
        })
    }
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
            scope: true,
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
}
