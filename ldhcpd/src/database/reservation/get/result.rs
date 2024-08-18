use crate::model::{ClientID, Description};
use data_format::{MapSerializer, Serialize};
use net_utils::ip::v4::IPv4Address;
use sql::{Column, FromColumn, FromRow};
use std::num::NonZeroUsize;

/// A reservation to be read from the database
pub struct Reservation {
    id: Option<usize>,
    client_id: Option<ClientID>,
    ip_address: Option<IPv4Address>,
    renewal_time: Option<Option<NonZeroUsize>>,
    description: Option<Option<Description<'static>>>,
    scope: Option<usize>,
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
