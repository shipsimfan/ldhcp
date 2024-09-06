use crate::LDHCPD;

mod v1;

router::router!(pub route, LDHCPD, (), [], [b"v1" => v1::route], []);
