use crate::common::{ClientID, Description};
use data_format::Deserialize;
use net_utils::ip::v4::IPv4Address;
use router::FromRequestBody;
use std::num::NonZeroUsize;

#[derive(Debug, Deserialize, FromRequestBody)]
pub(super) struct NewReservation<'a> {
    pub(super) client_id: ClientID,
    pub(super) ip_address: IPv4Address,
    pub(super) renewal_time: Option<NonZeroUsize>,
    pub(super) description: Option<Description<'a>>,
}
