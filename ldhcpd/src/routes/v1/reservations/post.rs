use router::endpoint;

use crate::LDHCPD;

#[endpoint]
pub fn post(_ldhcp: &LDHCPD) -> Result<(), ()> {
    Ok(())
}
