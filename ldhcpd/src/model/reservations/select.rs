use crate::model::{ClientID, Description};
use data_format::{Deserialize, Serialize};
use net_utils::ip::v4::IPv4Address;
use router::{fields, IntoResponse};
use std::num::NonZeroUsize;

#[derive(Debug, Serialize, Deserialize, IntoResponse)]
pub struct Reservation<'a> {
    pub id: Option<usize>,
    pub client_id: Option<ClientID>,
    pub ip_address: Option<IPv4Address>,
    pub scope: Option<usize>,
    pub renewal_time: Option<NonZeroUsize>,
    pub description: Option<Description<'a>>,
}

fields!(
    pub struct ReservationFields;

    pub enum ReservationField {
        ID => id,
        ClientID => client_id,
        IPAddress => ip_address,
        Scope => scope,
        RenewalTime => renewal_time,
        Description => description
    }
);
